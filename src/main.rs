mod commands;
mod input;

use std::{
    error::Error,
    fs,
    io::{self, BufRead},
    path::PathBuf,
};

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{channel::mpsc::unbounded, future::select, pin_mut};
use tui::{
    backend::CrosstermBackend,
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

/// A text editor trying its hardest to not edit text
#[derive(Parser)]
struct Opts {
    /// The name of the file to open
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
}

trait Filter {
    fn filter(&self, buf: &Text) -> Text;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Opts::parse();

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;
    if let Some(open_file) = cli.file {
        if open_file.exists() {
            let file = fs::File::open(&open_file)?;

            let title = open_file
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string();

            let buffer = Text {
                lines: io::BufReader::new(file)
                    .lines()
                    .map(|s| Spans::from(vec![Span::from(s.unwrap_or_default())]))
                    .collect::<Vec<_>>(),
            };
            let paragraph = Paragraph::new(buffer.lines)
                .block(Block::default().borders(Borders::ALL).title(title));
            term.draw(|f| f.render_widget(paragraph, f.size()))?;
        } else {
            dbg!("ERROR NO FILE");
        }
    }
    let (tx, rx) = unbounded();
    let input_handler = input::InputHandler::new(tx);
    let input_handle = input_handler.handle();
    let command_handler = commands::CommandHandler::new(rx).handle();
    pin_mut!(input_handle);
    pin_mut!(command_handler);
    select(input_handle, command_handler).await;

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    Ok(())
}
