use crossterm::event::KeyEvent;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::Stdout;

pub mod list;

#[derive(Debug)]
pub struct ViewError(String);

impl Display for ViewError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<io::Error> for ViewError {
    fn from(error: io::Error) -> Self {
        ViewError(format!("{}", error))
    }
}

impl Error for ViewError {}

pub trait View {
    fn process_key(&mut self, ke: KeyEvent) -> Result<Option<Box<dyn View>>, ViewError>;
    fn tick(&mut self);
    fn draw(&self, stdout: &mut Stdout, term_size: (u16, u16)) -> Result<(), ViewError>;
}
