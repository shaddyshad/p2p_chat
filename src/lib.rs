use log::info;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::{ sync::{Arc, RwLock, Weak}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub msg: String,
    pub id: Uuid,
    pub sender: String,
    pub reply_id: Option<Uuid>,
    pub topic: String
}

impl Message {
    pub fn new(msg: String, sender: String, topic: String) -> Self {
        Self {
            msg,
            sender,
            reply_id: None,
            id: Uuid::new_v4(),
            topic
        }
    }


}

/// events from network 
pub enum Event {
    Msg(Message)
}


/// Trait for types that can 
pub trait EventListener {
    fn handle(&mut self, evt: &Event);
    fn subject(&self) -> String;
}

/// Trait for emitters 
pub trait EventEmitter<T: EventListener>{
    fn subscribe(&mut self, handler: Arc<RwLock<T>>);
    fn emit(&mut self, evt: Event);
} 



/// network notifier
pub struct Notifier<T: EventListener> {
    handlers: Vec<Weak<RwLock<T>>>
}

impl <T: EventListener> Notifier<T>{
    pub fn new() -> Self {
        Self {
            handlers: vec![]
        }
    }

    pub fn count(&self) -> usize {
        self.handlers.len()
    }
}


impl<T: EventListener> EventEmitter<T> for Notifier<T>{
    fn subscribe(&mut self, handler: Arc<RwLock<T>>) {
        self.handlers.push(Arc::downgrade(&handler));
    }

    fn emit(&mut self, evt: Event) {
        let mut cleanup = false;

        for handler in self.handlers.iter(){
            if let Some(rc) = handler.upgrade() {
                let mut h = rc.write().unwrap();

                h.handle(&evt);
            }else{
                info!("Needs cleanup");
                cleanup = true;
            }
        }


        if cleanup{
            self.handlers.retain(|ref h| {
                let rf = h.clone().upgrade();
                if let Some(_) = rf {
                    true 
                }else{
                    false
                }
            })
        }
    }
} 


