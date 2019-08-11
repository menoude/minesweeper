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

    if let Err(e) = Game::new((height, width), nb_mines).run() {
        println!("{}", e);
        exit(1);
    }
}
