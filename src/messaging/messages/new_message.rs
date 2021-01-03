use super::{Message, Storage, Publisher, QueryPredicate};
use uuid::Uuid;

/// type to send a new message 
pub struct NewMessage<P: Publisher<Message>, M: Storage<Item=Message>> {
    pub publisher: P,
    pub storage: M,
    pub message: Message
}

/// A predicate to check if any messages exists by their id 
struct FindById(pub Uuid);

impl QueryPredicate<Message> for FindById {
    fn matches(&self, other: &Message) -> bool {
        self.0 == other.id
    }
}

impl <P: Publisher<Message>, M: Storage<Item=Message>> NewMessage<P, M>{
    /// send a message to peers in the network 
    pub fn send(&mut self){
        if let Ok(()) = self.storage.save(self.message.clone()){
            // publish using the publisher 
            self.publisher.publish(self.message.clone())
        }
    }
    

    /// Check if a message has been saved in the local storage
    pub fn exists(&self) -> bool {
        let found = self.storage.find(FindById(self.message.id.clone()));

        !found.is_empty()
    }
}