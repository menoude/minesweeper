use minesweeper::{
    args::{get_args,},
    game::Game,
};
use std::process::exit;

fn main() {
    let params = get_args();

    if let Err(e) = Game::new(params).run() {
        println!("{}", e);
        exit(1);
    }
}
