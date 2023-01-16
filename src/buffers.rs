use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead},
    path::PathBuf,
};
use thiserror::Error;
use tui::{
    text::Text,
    widgets::{Block, Borders, Paragraph}, Terminal, backend::Backend,
};

pub trait Buffer: Sized {
    type OpenOptions;
    type OpenError: Error;

    fn title(&self) -> String;
    fn open(options: Self::OpenOptions) -> Result<Self, Self::OpenError>;
    // TODO: Make this return something better than `Text`
    fn get_lines(&self) -> Result<Text, io::Error>;

    fn draw<B: Backend>(&self, term: &mut Terminal<B>) -> io::Result<()> {
        let paragraph = Paragraph::new(self.get_lines()?)
            .block(Block::default().borders(Borders::ALL).title(self.title()));
        term.draw(|f| f.render_widget(paragraph, f.size()))?;
        Ok(())
    }
}

pub struct FileBuffer {
    path: PathBuf,
    file: File,
}

#[derive(Error, Debug)]
pub enum FileBufferOpenFailure {
    #[error("File does not exist")]
    FileDoesntExist,
    #[error("Error opening file")]
    FileIOError(#[from] std::io::Error),
}

impl Buffer for FileBuffer {
    type OpenOptions = PathBuf;
    type OpenError = FileBufferOpenFailure;

    fn title(&self) -> String {
        self.path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string()
    }

    fn open(options: Self::OpenOptions) -> Result<Self, Self::OpenError> {
        if !options.exists() {
            return Err(FileBufferOpenFailure::FileDoesntExist);
        }
        Ok(Self {
            path: options.clone(),
            file: fs::File::open(&options)?,
        })
    }

    // TODO: Make this take a parameter for the location, which can be changed when scrolling
    fn get_lines(&self) -> Result<Text, io::Error> {
        let text: Result<Vec<_>, _> = io::BufReader::new(&self.file).lines().collect();
        Ok(Text::from(text?.join("\n")))
    }
}
