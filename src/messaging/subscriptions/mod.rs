/// Trait for types that provides a behavior for how events in the 
/// messaging streams are handled.
/// It allows peers to subcribe and unsubscribe from topics in the message streams 

pub trait Subscriptions {
    /// Subscribe a peer to the network 
    /// Returns true if subscription worked and false if the subscription already exists.
    fn subscribe(&mut self, peer_id: &str, topic: &str) -> bool;

    /// Unsubscribe a peer from the network
    /// Returns true if we were subscribed and false otherwise 
    fn unsubscribe(&mut self, peer_id: &str, topic: &str) -> bool;
}