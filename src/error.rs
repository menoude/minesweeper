use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum MineError {
    FormatError(std::fmt::Error),
    NbMinesError,
    SizeError,
    IoError(std::io::Error),
}

impl std::error::Error for MineError {}

impl Display for MineError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match self {
            MineError::FormatError(e) => format!("Output error: {}", e),
            MineError::NbMinesError => {
                String::from("Wrong number of mines, please indicate 1 < m < height * width - 2")
            }
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
        MineError::IoError(err)
    }
}

impl From<std::fmt::Error> for MineError {
    fn from(err: std::fmt::Error) -> Self {
        MineError::FormatError(err)
    }
}

impl From<std::num::ParseIntError> for MineError {
    fn from(_err: std::num::ParseIntError) -> Self {
        MineError::SizeError
    }
}
