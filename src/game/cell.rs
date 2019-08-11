use crate::game::content::Content;
use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Aspect {
    Visible,
    Hidden,
    Flagged,
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub content: Content,
    pub adjacent_mines: u16,
    pub aspect: Aspect,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Cell {
                aspect: Aspect::Hidden,
                ..
            } => write!(f, "📦 "),
            Cell {
                aspect: Aspect::Flagged,
                ..
            } => write!(f, "🏳 "),
            Cell {
                content: Content::Empty,
                adjacent_mines,
                ..
            } => {
                if *adjacent_mines > 0 {
                    write!(f, "{} ", adjacent_mines)
                } else {
                    write!(f, "  ")
                }
            }
            Cell { content, .. } => write!(f, "{} ", content),
        }
    }
}

impl Cell {
    pub fn set_visible(&mut self) {
        self.aspect = Aspect::Visible;
    }

    pub fn is_visible(self) -> bool {
        self.aspect == Aspect::Visible
    }

    pub fn has_mine(self) -> bool {
        self.content == Content::Mine
    }

    pub fn has_adjacent_mine(self) -> bool {
        self.adjacent_mines > 0
    }

    pub fn set_mine(&mut self) {
        self.content = Content::Mine;
    }

    pub fn toggle_flag(&mut self) {
        match self.aspect {
            Aspect::Hidden => self.aspect = Aspect::Flagged,
            Aspect::Flagged => self.aspect = Aspect::Hidden,
            _ => {}
        }
    }
}
