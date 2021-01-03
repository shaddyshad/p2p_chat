extern crate peer_chat;
use log::{info};


use peer_chat::{Peer, storage::MemoryStorage, Subscriber, Group,Publisher, Message};

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


// dummy publisher 
struct DummyPublisher;

impl Publisher<Message> for DummyPublisher {
    fn publish(&mut self, msg: Message) {
        info!("message {} published to {}", msg.id(), msg.group_name);
    }
}

fn main() {
    pretty_env_logger::init();
    // create two a peer 
    let peer =  Peer::new("shaddyshad".to_string(), "ank3r".to_string());

    // get the sub and memory storage 
    let subscriptions = DummySubscriptions;
    let storage: MemoryStorage<Group> = MemoryStorage::new();

    // create a new group 
    let mut new_group = peer.new_group("chat001".to_string(), storage, subscriptions);

    // save the group by subscribing 
    new_group.subscribe();


    // deps 
    let publisher = DummyPublisher;
    let msg_store: MemoryStorage<Message> = MemoryStorage::new();
    // send amessage to the group 
    let mut new_msg = peer.new_message("chat001", "message", msg_store, publisher);
    new_msg.send();

    // finally, unsubscribe 
    new_group.unsubscribe();

    
    
}
