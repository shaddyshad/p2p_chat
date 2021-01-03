pub mod storage;
pub mod messaging;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use messaging::{Peer, Group, Subscriber, Publisher,Message};
