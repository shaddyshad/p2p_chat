pub mod storage;
pub mod messaging;
#[allow(dead_code)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use messaging::{Peer, Group, subscriptions::Subscriptions};
