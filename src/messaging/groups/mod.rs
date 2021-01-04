pub mod new_group;
pub mod list_groups;

use uuid::Uuid;
use std::time::SystemTime;

use crate::storage::{QueryPredicate};

pub use super::Subscriber;

pub use new_group::NewGroup;
/// Representation of a group chat 
#[derive(Debug, Clone)]
pub struct Group {
    name: String,
    id: Uuid,
    creator: String,
    ts: SystemTime
}

impl Group {
    pub fn new(name: String, creator: String) -> Self {
        Self {
            name,
            creator,
            id: Uuid::new_v4(),
            ts: SystemTime::now()
        }
    }

    /// get the peer id of the creator 
    pub fn peer_id(&self) -> String {
        self.creator.clone()
    }
}


/// A trait for types that can list all groups a peer is subsscribed to 
pub trait ListGroups: Iterator<Item=Group> {}