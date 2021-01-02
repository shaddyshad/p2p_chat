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
    let mut name = "shaddy shad".to_string();

    if let Some(n) = std::env::args().nth(1){
        name = n;
    }

    let mut peer = Peer::new(&name);
    peer.connect();
    peer.conn_status();


    if let Some(to_dial) = std::env::args().nth(2){
        info!("Dialed {:?}", to_dial);
        peer.dial(to_dial);
    }

    
    info!("PeerId: {}", peer.get_id());
    info!("Peer name: {}", peer.name);

    let mut stdin = BufReader::new(stdin()).lines();

    

    loop {
        let evt = tokio::select! {
            line = stdin.next_line() => Some(EvtType::Input(line.expect("can get a line ").expect("can read a line from stdin"))),
            response = peer.receiver.recv() => Some(EvtType::Response(response.expect("an event exists")))
        };

        if let Some(event) = evt {
            match event {
                EvtType::Input(line) => match line.as_str(){
                    "q" => break,
                    cmd if line.starts_with("create t") => handle_create_topic(cmd, &mut peer),
                    _ if line.starts_with("ls t") => handle_list_topics(&peer),
                    _ if line.starts_with("ls m") => handle_list_messages(&peer),
                    cmd if line.starts_with("join t") => handle_join_topic(cmd, &mut peer),
                    cmd if line.starts_with("create m") => handle_create_message(cmd, &mut peer),
                    _ => error!("unknown command")
                },
                EvtType::Response(_resp) => {
                    info!("New message ")
                }
            }
        } 
    }

    Ok(())

}

/// create a new topic 
fn handle_create_topic(cmd: &str, peer: &mut Peer){
    let rest = cmd.strip_prefix("create t").expect("no topic found");

    peer.join_topic(rest);
    info!("new topic {} added!", rest);
}

/// list subscribed topics 
fn handle_list_topics(peer: &Peer){
    let topics = peer.get_topics();

    topics.iter().for_each(|t| info!("Topic: {}", t));
}

// join a topic 
fn handle_join_topic(cmd: &str, peer: &mut Peer){
    let rest = cmd.strip_prefix("join t").expect("no topic found");

    peer.join_topic(rest);

    info!("joined topic {} added!", rest);
}

fn handle_list_messages(peer: &Peer){
    peer.messages
        .iter()
        .for_each(|msg| info!("{:?}", msg));
}

/// Create a new message 
fn handle_create_message(cmd: &str, peer: &mut Peer){
    let rest = cmd.strip_prefix("create m ").expect("invalid command");
    let elements: Vec<&str> = rest.split(" ").collect();

    if elements.len() < 2 {
        info!("Too few arguments - format topic|message");
    }else{
        let topic = elements.get(0).expect("can get a topic");
        let message = &elements[1..].join(" ");

        peer.send_message(topic, message);
    }
}