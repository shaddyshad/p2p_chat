use super::events;
use std::io;
use io::Stdout;
use tui::{
    Terminal,
    backend::TermionBackend,
    widgets::{Block, Borders},
    layout::{Layout, Constraint,Direction},
};
use termion::raw::{IntoRawMode, RawTerminal};

pub mod notifications;

pub use notifications::Notifications;

/// Main UI
/// renders the main layout 
pub struct Ui{
    terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>
}

impl Ui {
    pub fn new() -> Self {
        let stdout = io::stdout().into_raw_mode().expect("can retrieve stdout");
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend).expect("can create a new terminal");

        Ui {
            terminal
        }
    }

    // render the GUI
    pub fn render(&mut self) -> Result<(), io::Error> {
        // clear the terminal 
        self.clear();

        // draw 
        self.terminal.draw(|f| {
            // create a layout with 3 parts, header, body, footer 
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20)
                ].as_ref())
                .split(f.size());

            // create blocks 
            let header = Block::default()
                    .title("Status")
                    .borders(Borders::ALL);

            f.render_widget(header, chunks[0]);

            // content block 
            let content = Block::default()
                .title("peer chat")
                .borders(Borders::ALL);

            f.render_widget(content, chunks[1]);

            // footer block 
            let input = Block::default()
                    .title("Input")
                    .borders(Borders::ALL);

            f.render_widget(input, chunks[2]);
        })
    }


    /// clear the input 
    pub fn clear(&mut self){
        self.terminal.clear().expect("can clear terminal")
    }
}