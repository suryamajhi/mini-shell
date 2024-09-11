use std::error::Error;
use std::fmt::{Display, Formatter};

pub enum Command {
    Exit,
    Echo,
    Type,
    Pwd,
    CD,
    BuiltIn(String, String)
}

#[derive(Debug)]
pub enum CommandErr {
    InvalidCommand(String),
    CommandNotFound(String),
}

impl Display for CommandErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandErr::InvalidCommand(message) => write!(f, "{message}"),
            CommandErr::CommandNotFound(message) => write!(f, "{message}: command not found"),
        }
    }
}

impl Error for CommandErr {}
