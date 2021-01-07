use std::sync::{Arc, RwLock, Weak};
use super::pubsub::Message;

/// event emitted when a topic is subscribed 
#[derive(Debug)]
pub struct Subscribed {
    pub peer: String,
    pub topic: String 
} 


/// Events that can be emitted in the network
#[derive(Debug)]
pub enum Event {
    Msg(Message),
    Sub(Subscribed),
    Unsub(Subscribed),
    PeerAdded(String),
    PeerRemoved(String)
}

/// Types that can handle events
pub trait EventHandler {
    /// handle an event 
    fn handle(&mut self, event: &Event);
}


/// Types that can emit events in the network 
pub trait EventEmitter<T> 
where 
    T: EventHandler
{
    /// add an event handler 
    fn subscribe(&mut self, handler: Arc<RwLock<T>>);

    /// emit an event to the handlers 
    /// optionally, cleanup handlers whose references are released
    fn emit(&mut self, event: Event);
}


/// Network event emitter 
#[derive(Debug)]
pub struct NetworkEventEmitter<T> 
where 
    T: EventHandler
{
    handlers: Vec<Weak<RwLock<T>>>
}

impl<T> NetworkEventEmitter<T> 
where 
    T: EventHandler
{
    pub fn new() -> Self {
        Self {
            handlers: vec![]
        }
    }
}


// implement emitter trait 
impl<T> EventEmitter<T> for NetworkEventEmitter<T>
where   
    T: EventHandler
{
    fn subscribe(&mut self, handler: Arc<RwLock<T>>) {
        self.handlers.push(Arc::downgrade(&handler));
    }

    fn emit(&mut self, event: Event) {
        // check if we need cleanup
        let mut cleanup = false;

        for handler in self.handlers.iter(){
            if let Some(rc) = handler.upgrade(){
                // borrow the contents mutably and emit event 
                let mut h = rc.write().expect("handler locked");

                h.handle(&event);
            }else{
                cleanup = true;
            }
        }


        // perform cleanup 
        if cleanup {
            self.handlers.retain(|ref h| {
                // if no handler, remove 
                if let Some(_) = h.clone().upgrade() {
                    true
                }else{
                    false 
                }
            })
        }

    }
}