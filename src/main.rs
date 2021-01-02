extern crate peer_chat;
use log::{info, error};
use peer_chat::{Peer, Message};
use tokio::{io::{stdin, BufReader, AsyncBufReadExt}};

enum EvtType {
    Input(String),
    Response(Message)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    info!("Welcome to peer chat. ");

    let mut peer = Peer::new("shaddy shad");

    
    info!("PeerId: {}", peer.get_id());
    info!("Peer name: {}", peer.name);

    let mut stdin = BufReader::new(stdin()).lines();

    peer.connect();

    loop {
        let evt = tokio::select! {
            line = stdin.next_line() => Some(EvtType::Input(line.expect("can get a line ").expect("can read a line from stdin"))),
            response = peer.receiver.recv() => Some(EvtType::Response(response.expect("an event exists")))
        };

        if let Some(event) = evt {
            match event {
                EvtType::Input(line) => match line.as_str(){
                    "q" => break,
                    _ => error!("unknown command")
                },
                EvtType::Response(resp) => {
                    info!("New message ")
                }
            }
        } 
    }

    Ok(())

}
