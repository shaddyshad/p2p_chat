use super::{ui, events, pubsub, keystore};
use libp2p::{Transport, core::upgrade, floodsub::{Floodsub, Topic}, mdns::Mdns, mplex, noise, swarm::{SwarmBuilder, Swarm}, tcp::TokioTcpConfig};
use log::info;
use std::sync::{Arc, RwLock};
use tokio::spawn;

pub mod behaviour;

use keystore::KeyStore;
use ui::Notifications;
use events::{EventEmitter, NetworkEventEmitter};
use behaviour::ChatBehaviour;
use pubsub::Message;

/// Network manager 
/// Responsible for starting 
pub struct NetworkManager {
    swarm: Swarm<ChatBehaviour>
}

impl NetworkManager {
    /// create a new network manager 
    pub fn new() -> Self {
        let keystore = KeyStore::new();

        // create a transport 
        let transport = TokioTcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(keystore.noise_keys()).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed();

        // notifications handler 
        let handler = Arc::new(RwLock::new(Notifications::new()));

        // notifications emitter 
        let mut emitter: NetworkEventEmitter<Notifications> = NetworkEventEmitter::new();
        emitter.subscribe(handler.clone());

        let swarm = {
            let mdns = Mdns::new().expect("can create an mdns");
            let  behaviour = ChatBehaviour {
                mdns,
                floodsub: Floodsub::new(keystore.get_id()),
                notifier: emitter
            };

            SwarmBuilder::new(transport, behaviour, keystore.get_id())
                .executor(Box::new(|fut| {
                    spawn(fut);
                }))
                .build()
        };
        
        NetworkManager {
            swarm
        }
    }


    /// start swarm 
    pub fn start(&mut self){
        Swarm::listen_on(&mut self.swarm, "/ip4/0.0.0.0/tcp/0".parse().expect("can parse multi address"))
            .expect("can start swarm");
    }

    /// poll listening address 
    pub fn poll_addr(&self){
        for addr in Swarm::listeners(&self.swarm){
            info!("listening on {:?}", addr);
        }
    }

    ///subscribe to a topic 
    pub fn subscribe(&mut self, topic: &str) {
        let topic = Topic::new(topic);

        self.swarm.floodsub.subscribe(topic);
    }

    /// send a message 
    pub fn send_message(&mut self, message: Message, topic: &str){
        let topic = Topic::new(topic);
        let message = message.serialize();

        self.swarm.floodsub.publish(topic, message.as_bytes());
    }
}