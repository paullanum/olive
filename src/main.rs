mod input;
mod commands;

use std::{
    fs,
    io::{self, BufRead},
    path::PathBuf,
};

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
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
async fn main() -> Result<()> {
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

            let title = format!(
                "{}",
                &open_file
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
            );
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
    input::handle_input().await?;

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    Ok(())
}
