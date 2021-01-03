/// Trait for types that provides a behavior for how events in the 
/// messaging streams are handled.
/// It allows peers to subcribe and unsubscribe from topics in the message streams 

pub trait Subscriber {
    /// Subscribe a peer to the network 
    /// Returns true if subscription worked and false if the subscription already exists.
    fn subscribe(&mut self, peer_id: &str, topic: &str) -> bool;

    /// Unsubscribe a peer from the network
    /// Returns true if we were subscribed and false otherwise 
    fn unsubscribe(&mut self, peer_id: &str, topic: &str) -> bool;
}


/// Trait for types that allow pushing messages to a message stream 
/// Messages are assumed to be generic types
/// types should ensure that messages are delivered
pub trait Publisher<T> {
    /// publish a message to a chat board 
    fn publish(&mut self, source: &str, topic: &str, msg: T);
}