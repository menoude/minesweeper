pub mod args;
pub mod game;
pub mod error;

type Result<T> = std::result::Result<T, error::MineError>;
