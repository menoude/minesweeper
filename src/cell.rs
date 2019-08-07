use crate::content::Content;
use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub content: Content,
    pub adjacent_mines: u16,
    pub visible: bool,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Cell {
                content: Content::Empty,
                adjacent_mines,
                ..
            } => write!(f, "{} ", adjacent_mines),
            Cell { content, .. } => write!(f, "{} ", content),
        }
    }
}
