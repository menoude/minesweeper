use minesweeper::{
    args::get_args,
    game::{run, Game},
};
use std::process::exit;

fn main() {
    let params = get_args();

    if let Err(e) = run(Game::new(params)) {
        println!("{}", e);
        exit(1);
    }
}
