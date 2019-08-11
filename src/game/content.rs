use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Content {
    Mine,
    Empty,
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let character = match self {
            Content::Mine => '💥',
            Content::Empty => '.',
        };
        write!(f, "{}", character)
    }
}
