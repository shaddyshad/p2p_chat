extern crate peer_chat;
use log::{info};


use peer_chat::{Peer, storage::MemoryStorage, Subscriber, Group};

// stub subscriptions 
struct DummySubscriptions;

impl Subscriber for DummySubscriptions {
    fn subscribe(&mut self, peer_id: &str, topic: &str) -> bool {
        info!("{} subscribed to {}", peer_id, topic);

        true 
    }

    fn unsubscribe(&mut self, peer_id: &str, topic: &str) -> bool {
        info!("{} unsubscribed from {}", peer_id, topic);

        true 
    }
}

fn main() {
    pretty_env_logger::init();
    // create two a peer 
    let peer =  Peer::new("asdhsdf".to_string(), "ank3r".to_string());

    // get the sub and memory storage 
    let subscriptions = DummySubscriptions;
    let storage: MemoryStorage<Group> = MemoryStorage::new();

    // create a new group 
    let mut new_group = peer.new_group("chat001".to_string(), storage, subscriptions);

    // save the group by subscribing 
    new_group.subscribe();

    // finally, unsubscribe 
    new_group.unsubscribe();
    
}
