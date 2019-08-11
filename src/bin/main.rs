use minesweeper::{
    args::{get_args, Params},
    game::Game,
};
use std::process::exit;

fn main() {
    let Params {
        height,
        width,
        nb_mines,
    } = get_args();

    if let Err(e) = Game::run(height, width, nb_mines) {
        println!("{}", e);
        exit(1);
    }
}
