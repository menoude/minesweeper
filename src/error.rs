use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum MineError {
    TerminalError(String),
    OutputError,
    InputError,
    NbMinesError,
    SizeError,
    IoError(String),
}

impl std::error::Error for MineError {}

impl Display for MineError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match self {
            MineError::TerminalError(e) => format!("Error with the terminal: {}", e),
            MineError::OutputError => String::from("Output error"),
            MineError::InputError => String::from("Input error"),
            MineError::NbMinesError => String::from("Wrong number of mines"),
            MineError::SizeError => {
                String::from("Wrong size arguments, they are limited from 2x2 to 30x24")
            }
            MineError::IoError(e) => format!("I/O error: {}", e),
        };
        write!(f, "{}", message)
    }
}

impl From<std::io::Error> for MineError {
    fn from(err: std::io::Error) -> Self {
        MineError::IoError(err.to_string())
    }
}

impl From<std::fmt::Error> for MineError {
    fn from(_err: std::fmt::Error) -> Self {
        MineError::OutputError
    }
}

impl From<std::num::ParseIntError> for MineError {
    fn from(_err: std::num::ParseIntError) -> Self {
        MineError::SizeError
    }
}