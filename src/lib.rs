use libp2p::{
    floodsub::{Topic, Floodsub, FloodsubEvent},
    PeerId,
    identity,
    swarm::{NetworkBehaviourEventProcess, SwarmBuilder, Swarm},
    mdns::{Mdns, TokioMdns, MdnsEvent},
    NetworkBehaviour, Transport, 
    noise::{Keypair, X25519Spec, NoiseConfig},
    tcp::TokioTcpConfig,
    core::upgrade,
    yamux::YamuxConfig,
    Multiaddr
};

use tokio::{sync::mpsc};
use log::{info, error};
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

// a message that can be exchanged between peers 
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: Uuid,
    msg: String,
    source: String,
    ts: SystemTime,
    topic: String
}

impl Message {
    // Create a new message
    pub fn new(msg: String, source: String, topic: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            msg,
            source,
            ts: SystemTime::now(),
            topic
        }
    }
}

///Network behavior for our chat app 
#[derive(NetworkBehaviour)]
struct ChatBehavior {
    floodsub: Floodsub,
    mdns: TokioMdns,
    #[behaviour(ignore)]
    sender: mpsc::UnboundedSender<Message>

}

/// flood sub events 
impl NetworkBehaviourEventProcess<FloodsubEvent> for ChatBehavior {
    fn inject_event(&mut self, event: FloodsubEvent) {
        match event {
            _ => {
                info!("New event")
            }
        }
    }
}

/// mdns events 
impl NetworkBehaviourEventProcess<MdnsEvent> for ChatBehavior {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            _ => {
                info!("Mdns event")
            }
        }
    }
}


pub struct Peer {
    topics: Vec<Topic>,
    peer_id: PeerId,
    keys: identity::Keypair,
    pub name: String,
    messages: Vec<Message>,
    swarm: Swarm<ChatBehavior>,
    addr: Multiaddr
}


impl Peer {
    /// generates a new peer identity details and creates 
    /// a new network peer.
    pub fn new(name: &str ) -> Self {
        let keys =identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keys.public());

        // create a stream channel 
        let (sender, mut receiver) = mpsc::unbounded_channel();

        let auth_keys = Keypair::<X25519Spec>::new()
            .into_authentic(&keys)
            .expect("can create auth keys");

        // authenticate with noise 
        let noise = NoiseConfig::xx(auth_keys).into_authenticated();
        let yamux = YamuxConfig::default();

        let transport = TokioTcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(noise)
            .multiplex(yamux)
            .boxed();

        let behavior = ChatBehavior {
            floodsub: Floodsub::new(peer_id.clone()),
            mdns: TokioMdns::new().expect("Can create an mdns"),
            sender
        };

        // create a swarm
        let mut swarm = SwarmBuilder::new(transport, behavior, peer_id.clone())
            .executor(Box::new(|fut|{
                tokio::spawn(fut);
            })).build();

            

        Self {
            topics: vec![],
            keys,
            peer_id,
            name: name.to_string(),
            messages: vec![],
            swarm,
            addr: "/ip4/0.0.0.0/tcp/0".parse().expect("can parse address to multiaddress")
        }
    }

    /// Create a new topic and subscribe it to the network
    pub fn join_topic(&mut self, topic_name: String) {
        let topic = Topic::new(topic_name);

        // subscribe it to the network 
        if self.swarm.floodsub.subscribe(topic.clone()) /*Insert subscription code*/{
            self.topics.push(topic);
        }
    }

    /// Get a list of all topics subscribed 
    pub fn get_topics(&self) -> Vec<String> {
        self.topics.iter()
            .map(|topic| topic.id().to_string())
            .collect()
    }

    /// connect to the network
    /// it assumes caller is an async function 
    pub fn connect(&mut self) {
        Swarm::listen_on(
            &mut self.swarm, self.addr.clone()
        ).expect("swarm can be started");
    }

    /// get the peer id 
    pub fn get_id(&self) -> PeerId {
        self.peer_id.clone()
    }


}