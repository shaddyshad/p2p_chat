use std::io;
use tui::{
    Terminal,
    backend::TermionBackend,
    widgets::{Block, Borders},
    layout::{Layout, Constraint,Direction},
};
use termion::raw::IntoRawMode;

/// main function 
fn main() -> Result<(), io::Error>{
    pretty_env_logger::init();
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(f.size());

        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);

        f.render_widget(block, chunks[0]);

        let block = Block::default()
            .title("Block 2")
            .borders(Borders::ALL);

        f.render_widget(block, chunks[1]);

    })
}