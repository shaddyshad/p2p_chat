use uuid::Uuid;
mod new_group;
use std::time::SystemTime;

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
}
