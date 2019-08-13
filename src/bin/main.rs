use minesweeper::{
    args::get_args,
    game::{run, Game},
};
use std::process::exit;

fn main() {
    let params = match get_args() {
        Ok(arguments) => arguments,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    if let Err(e) = run(Game::new(params)) {
        println!("{}", e);
        exit(1);
    }
}
