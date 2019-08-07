use minesweeper::{
    args::{get_args, Params},
    field::Field,
};

use std::io::{stdout, Write};

fn main() {
    let Params {
        height,
        width,
        nb_mines,
    } = get_args();

    let field = Field::new(height, width).populate_with_mines(nb_mines);
    print!("{}", field);
    stdout().flush().unwrap();
}
