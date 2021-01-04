use futures::prelude::*;
use libp2p::{
    Multiaddr, 
    NetworkBehaviour, PeerId, Swarm, Transport, core::upgrade, floodsub::{self, Floodsub, FloodsubEvent}, 
    identity, mdns::{Mdns, MdnsEvent}, mplex, noise, swarm::{NetworkBehaviourEventProcess, SwarmBuilder}, tcp::TokioTcpConfig};
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt};
use log::{info};

// custom behaviour 
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    floodsub: Floodsub,
    mdns: Mdns 
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for MyBehaviour {
    // called when flood sub emits eventss 
    fn inject_event(&mut self, message: FloodsubEvent){
        if let FloodsubEvent::Message(msg) = message {
            info!("Received: '{}' from '{}'", String::from_utf8_lossy(&msg.data), msg.source);
        }
    }
}


impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
    fn inject_event(&mut self, event: MdnsEvent){
        match event {
            MdnsEvent::Discovered(list) => {
                for(peer, _) in list {
                    self.floodsub.add_node_to_partial_view(peer);
                }
            },
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    self.floodsub.remove_node_from_partial_view(&peer);
                }
            }
        }
    }
}


/// main function 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    pretty_env_logger::init();

    // random keypair 
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from_public_key(id_keys.public());

    info!("local id {}", peer_id);

    // create a keypair for transport 
    let noise_keys = noise::Keypair::<noise::X25519>::new()
        .into_authentic(&id_keys)
        .expect("signing libp2p-noise static DH keypair failed");

    // tokio based tcpconnection using noise for authenticated transport and mplex streams
    let transport = TokioTcpConfig::new().nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    // flood sub topic 
    let floodsub_topic = floodsub::Topic::new("chat");

    let mut swarm = {
        let mdns = Mdns::new()?;
        let mut behaviour = MyBehaviour {
            floodsub: Floodsub::new(peer_id.clone()),
            mdns
        };

        behaviour.floodsub.subscribe(floodsub_topic.clone());   

        SwarmBuilder::new(transport, behaviour, peer_id.clone())
            // spawn background tasks into tokio rt
            .executor(Box::new(|fut| {tokio::spawn(fut);}))
            .build()
    };

    // reach out 
    if let Some(to_dial) = std::env::args().nth(1){
        let addr: Multiaddr = to_dial.parse()?;
        Swarm::dial_addr(&mut swarm, addr)?;
        info!("Dialed {:?}", to_dial);
    }


    // read lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // listen 
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse()?)?;

    // off 
    let mut listening = false;
    loop {
        let to_publish = {
            tokio::select! {
                line = stdin.try_next() => Some((floodsub_topic.clone(), line?.expect("stdin closed"))),
                event = swarm.next() => {
                    info!("New event: {:?}", event);
                    None 
                }
            }
        };

        if let Some((topic,line)) = to_publish {
            swarm.floodsub.publish(topic, line.as_bytes());
        }

        if !listening {
            for addr in Swarm::listeners(&swarm){
                info!("listening on {:?}", addr);
                listening = true;
            }
        }
    }
}