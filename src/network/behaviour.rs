use super::ui::Notifications;
use super::events::{Event, EventEmitter, NetworkEventEmitter, Subscribed};
use libp2p::{
    NetworkBehaviour,
    floodsub::{Floodsub, FloodsubEvent},
    mdns::{Mdns, MdnsEvent},
    swarm::{NetworkBehaviourEventProcess}
};
use super::pubsub::Message;

/// defines the general network behaviour 
#[derive(NetworkBehaviour)]
pub struct ChatBehaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,
    #[behaviour(ignore)]
    pub notifier: NetworkEventEmitter<Notifications>
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for ChatBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        match event {
            FloodsubEvent::Message(msg) => {
                // parse the message into a Message and emit 
                let msg_string = String::from_utf8_lossy(&msg.data);
                let message = Message::deserialize(&msg_string);

                self.notifier.emit(Event::Msg(message));
            }
            FloodsubEvent::Subscribed {peer_id, topic} => {
                let subscribed = Subscribed {peer: peer_id.to_string(), topic: topic.id().to_string()};

                self.notifier.emit(Event::Sub(subscribed));
            }
            FloodsubEvent::Unsubscribed {peer_id, topic} => {
                let unsubscribed = Subscribed {peer: peer_id.to_string(), topic: topic.id().to_string()};
                self.notifier.emit(Event::Unsub(unsubscribed))
            }
        }
    }
}


impl NetworkBehaviourEventProcess<MdnsEvent> for ChatBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(peers) => {
                // add the peers to known list and emit a notification 
                for (peer, _) in peers {
                    self.notifier.emit(Event::PeerAdded(peer.to_string()));

                    self.floodsub.add_node_to_partial_view(peer);
                }
            },
            MdnsEvent::Expired(expired) => {
                // removed expired peers from list and emit a notification 
                for (peer, _) in expired {
                    self.floodsub.remove_node_from_partial_view(&peer);


                    // notify peer removed 
                    self.notifier.emit(Event::PeerRemoved(peer.to_string()));
                }
            }
        }
    }
}