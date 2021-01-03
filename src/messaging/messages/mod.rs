use std::time::SystemTime;
use uuid::Uuid;

use super::{Publisher, Storage, QueryPredicate};
pub mod new_message;
pub use new_message::NewMessage;
/// A message that can be shared between peers in a network chat group 
#[derive(Debug, Clone)]
pub struct Message {
    id: Uuid,
    msg: String,
    source: String,         // peer id of the sender 
    ts: SystemTime,         // timestamp 
    group_name: String
} 



impl Message {
    pub fn new(msg: &str, source: &str, group_name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            ts: SystemTime::now(),
            source: source.to_string(),
            group_name: group_name.to_string(),
            msg: msg.to_string()
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }
}