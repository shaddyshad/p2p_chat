extern crate peer_chat;
use log::info;
use peer_chat::Peer;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let mut peer = Peer::new("shaddy shad");

    info!("Welcome to peer chat. ");
    info!("PeerId: {}", peer.get_id());
    info!("Peer name: {}", peer.name);

}
