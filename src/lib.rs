use libp2p::{Multiaddr, NetworkBehaviour, PeerId, floodsub::{Floodsub, FloodsubConfig, FloodsubEvent, Topic}, identity, mdns::{Mdns, TokioMdns, MdnsEvent}, noise::{Keypair, X25519Spec, NoiseConfig}, swarm::{NetworkBehaviourEventProcess, SwarmBuilder, Swarm}, tcp::TokioTcpConfig, yamux::YamuxConfig};

use tokio::{sync::mpsc};
use log::{info};
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;


// a message that can be exchanged between peers 
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    id: Uuid,
    msg: String,
    source: String,
    ts: SystemTime,
    topic: String
}

impl Message {
    // Create a new message
    pub fn new(msg: &str, source: PeerId, topic: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            msg: msg.to_string(),
            source: source.to_string(),
            ts: SystemTime::now(),
            topic: topic.to_string()
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
            FloodsubEvent::Message(msg) => {
                if let Ok(resp) = serde_json::from_slice::<Message>(&msg.data){
                    info!("Message from {} - {}", msg.source, resp.msg);
                }
            },
            _ => ()
        }
    }
}

/// mdns events 
impl NetworkBehaviourEventProcess<MdnsEvent> for ChatBehavior {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Expired(expired_list) => {
                for (peer, _) in expired_list{
                    if !self.mdns.has_node(&peer){
                        self.floodsub.remove_node_from_partial_view(&peer)
                    }
                }
            },
            MdnsEvent::Discovered(discovered) => {
                for (peer,_) in discovered{
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            
        }
    }
}


pub struct Peer {
    topics: Vec<Topic>,
    peer_id: PeerId,
    pub name: String,
    pub messages: Vec<Message>,
    swarm: Swarm<ChatBehavior>,
    addr: Multiaddr,
    pub receiver: mpsc::UnboundedReceiver<Message>,
}


impl Peer {
    /// generates a new peer identity details and creates 
    /// a new network peer.
    pub fn new(name: &str ) -> Self {
        let keys =identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keys.public());

        // create a stream channel 
        let (sender, receiver) = mpsc::unbounded_channel();

        let transport = libp2p::build_development_transport(keys).expect("can build a transport");


        let behavior = ChatBehavior {
            floodsub: Floodsub::from_config(FloodsubConfig{local_peer_id: peer_id.clone(), subscribe_local_messages: true}),
            mdns: TokioMdns::new().expect("Can create an mdns"),
            sender
        };

        // create a swarm
        let swarm = SwarmBuilder::new(transport, behavior, peer_id.clone())
            .executor(Box::new(|fut|{
                tokio::spawn(fut);
            })).build();

            

        Self {
            topics: vec![],
            peer_id,
            name: name.to_string(),
            messages: vec![],
            swarm,
            addr: "/ip4/0.0.0.0/tcp/5050".parse().expect("can parse address to multiaddress"),
            receiver
        }
    }

    /// Create a new topic and subscribe it to the network
    pub fn join_topic(&mut self, topic_name: &str) {
        let topic = Topic::new(topic_name);

        // if topic exists just exit 
        if self.topics.contains(&topic){
            return;
        }

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
        info!("Connecting to the network!");

        Swarm::listen_on(
            &mut self.swarm, self.addr.clone()
        ).expect("swarm can be started");

    }

    // Print connection status 
    pub fn conn_status(&self){  
        for addr in Swarm::listeners(&self.swarm){
            info!("Listening on {:?}", addr);
        }
    }

    /// get the peer id 
    pub fn get_id(&self) -> PeerId {
        self.peer_id.clone()
    }

    /// send a message 
    pub fn send_message(&mut self, topic: &str, message: &str){
        // find the topic in the subscriptions
        let message = Message::new(message, self.get_id(), topic);
        let ser_message = serde_json::to_string(&message).expect("can jsonify message");
        
        self.swarm.floodsub.publish(Topic::new(topic), ser_message);

        self.messages.push(message);
        info!("Message sent");
    }


    /// Dial another node
    pub fn dial(&mut self, dial: String) {
        let addr: Multiaddr = dial.parse().expect("can convert to multi address");

        Swarm::dial_addr(&mut self.swarm, addr).expect("cannot dial user");
    }

}