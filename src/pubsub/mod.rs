/// Pubsub implementation
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    id: Uuid,
    source: String,
    msg: String,
    reply_id: Option<String>,
    topic: String 
}


impl Message {
    pub fn new(source: String, msg: String, topic: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            msg,
            topic,
            reply_id: None 
        }
    }

    /// serialize the message into a string 
    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).expect("can serialize message")
    }

    /// deserialize a json string into a message
    pub fn deserialize(string: &str) -> Self {
        serde_json::from_str(string).expect("can deserialize json into message")
    }

}