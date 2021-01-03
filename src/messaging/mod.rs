pub mod pubsub;
pub mod groups;
pub mod messages;

pub use groups::{NewGroup, Group};
pub use pubsub::{Subscriber, Publisher};
use super::storage::{Storage, QueryPredicate};
pub use messages::{Message, NewMessage};

/// A peer is the main actor in the messaging system 
#[derive(Debug)]
pub struct Peer {
    peer_id: String,
    username: String
}

impl Peer {
    /// Create a new peer 
    pub fn new(peer_id: String, username: String) -> Self {
        Self {
            peer_id,
            username
        }
    }


    /// Create a new group 
    pub fn new_group<S , M>(&self, group_name: String, storage: S, subscriber: M) -> NewGroup<S, M>
    where 
        S: Storage<Item=Group>,
        M: Subscriber
    {
        NewGroup {
            group_name,
            storage,
            subscriber,
            peer_id: self.peer_id.clone()
        }
    }


    /// Create a new message 
    pub fn new_message<P, S>(&self, group_name: &str, message: &str, storage: S, publisher: P) -> NewMessage<P, S>
    where 
        P: Publisher<Message>,
        S: Storage<Item=Message>
    {
        let msg = Message::new(message, &self.peer_id, &group_name);

        NewMessage {
            publisher,
            storage,
            message: msg.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use log::info;

    use crate::storage::memory_storage::MemoryStorage;

    use super::*;

    fn setup() -> Peer {
        Peer::new("ank3r".to_string(), "shaddyshad".to_string())
    }

    // stub Subscriber 
    struct Subs;

    impl Subscriber for Subs {
        fn subscribe(&mut self, peer_id: &str, topic: &str) -> bool {
            info!("{} subscribed to {}", peer_id, topic);

            true 
        }

        fn unsubscribe(&mut self, peer_id: &str, topic: &str) -> bool {
            info!("{} unsubscribed from {}", peer_id, topic);

            true 
        }
    }

    // stub publisher 
    struct Pub;

    impl Publisher<Message> for Pub {
        fn publish(&mut self, msg: Message) {
            info!("{:?}", msg);
        }
    }

    #[test]
    fn test_can_create_group(){
        let peer = setup();
        
        // dependencies 
        let subs = Subs;
        let storage: MemoryStorage<Group> = MemoryStorage::new();

        let mut new_group = peer.new_group("chat001".to_string(), storage, subs);

        // check that group is not saved
        assert!(!new_group.exists());
        new_group.subscribe();
        assert!(new_group.exists());
    }


    #[test]
    fn test_can_send_msg(){
        let peer = setup();

        // deps 
        let publisher = Pub;
        let storage: MemoryStorage<Message> = MemoryStorage::new();


        let mut new_msg = peer.new_message("chat001", "Hey bro, ssup", storage, publisher);

        // before publishing, the message is not saved yet 
        assert!(!new_msg.exists());
        // afterpublishing it is saved 
        new_msg.send();
        assert!(new_msg.exists());

    }
}