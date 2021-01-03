pub mod subscriptions;
pub mod groups;

pub use groups::{NewGroup, Group};
use subscriptions::{Subscriptions};
use super::storage::{Storage};

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
    pub fn new_group<S: , M>(&self, group_name: String, storage: S, subscriptions: M) -> NewGroup<S, M>
    where 
        S: Storage<Item=Group>,
        M: Subscriptions
    {
        NewGroup {
            group_name,
            storage,
            subscriptions,
            peer_id: self.peer_id.clone()
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

    // stub subscriptions 
    struct Subs;

    impl Subscriptions for Subs {
        fn subscribe(&mut self, peer_id: &str, topic: &str) -> bool {
            info!("{} subscribed to {}", peer_id, topic);

            true 
        }

        fn unsubscribe(&mut self, peer_id: &str, topic: &str) -> bool {
            info!("{} unsubscribed from {}", peer_id, topic);

            true 
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
}