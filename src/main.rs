use termion::raw::IntoRawMode;
use tui::{
    terminal::Terminal,
    backend::TermionBackend,
    layout::{Layout, Constraint, Direction, Alignment},
    widgets::{Block, Borders, BorderType, Paragraph, Wrap, List, ListItem},
    style::{Color, Style, Modifier},
    text::{Span}
};
use std::io::{stdout, Error};
use peer_chat::network::NetworkManager;

/// main function 
fn main() -> Result<(), Error>{
    pretty_env_logger::init();
    let stdout = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut network = NetworkManager::new();

    let online = if network.is_connected() {
        Span::styled("online", Style::default().fg(Color::Green))
    }else{
        Span::styled("offline", Style::default().fg(Color::Red))
    };



    terminal.draw(|f| {
        let size = f.size();
        // create a block with a green background and white fg 
        let body = Block::default()
            .title("Peer chat")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Black).fg(Color::White));

        f.render_widget(body, size);

        // chunk the display into three areas 
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20)
            ].as_ref())
            .split(f.size());

        let top_area = chunks[0];
        let content_area = chunks[1];
        let footer_area = chunks[2];

        // divide the top area into 2 parts 1/3 - 2/3 
        let top_chunks = Layout::default()
            .margin(2)
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(2,3), Constraint::Ratio(1,3)].as_ref())
            .split(top_area);

        let top_left = top_chunks[0];
        let top_right = top_chunks[1];

        // render the peer id 
        let p = Paragraph::new(network.peer_id())
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(Alignment::Left)
            .wrap(Wrap {trim: true});

        f.render_widget(p, top_left);

        let status = Paragraph::new(online)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(Alignment::Right);

        f.render_widget(status, top_right);


        // content block 
        let content_block = Block::default()
            .borders(Borders::TOP)
            .title("Chats");
        f.render_widget(content_block, content_area);

        // divide the content area into 3 parts, 20% for topics, 10% space 70% listing
        let content_chunks = Layout::default()
            .margin(1)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(10), Constraint::Percentage(70)].as_ref())
            .split(content_area);

        let listing = content_chunks[0];
        let messages = content_chunks[2];

        // list some topics 
        let topics = [ListItem::new("Chat 001"), ListItem::new("Chat 002"), ListItem::new("Chat 003"), ListItem::new("Chat 004")];

        let list = List::new(topics)
            .block(Block::default().title("Topics").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        f.render_widget(list, listing);


    })

}