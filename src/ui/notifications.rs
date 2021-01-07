use log::info;

use super::events::{EventHandler, Event};

/// Notifications module for the ui
///
/// Handles the notifications from the network 
pub struct Notifications;

impl Notifications {
    pub fn new() -> Self {
        Notifications
    }
}


impl EventHandler for Notifications {
    fn handle(&mut self, event: &Event) {
        info!("New event {:?}", event);
    }
}