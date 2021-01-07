use peer_chat::Ui;

/// main function 
fn main(){
    pretty_env_logger::init();
    
    // create a new UI 
    let mut ui = Ui::new();

    ui.render().expect("can render ui");
}