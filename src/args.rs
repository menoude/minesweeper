use clap::{App, Arg};

use crate::{error::MineError, game::GameConfig, Result};

use std::cmp::max;

pub fn get_args() -> Result<GameConfig> {
    let matches = App::new("Minesweeper")
        .version("1.0")
        .author("menoude")
        .about("Minesweeper game!")
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("HEIGHT")
                .help("Sets the height of the board")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .help("Sets the width of the board")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("nb_mines")
                .short("m")
                .long("mines")
                .value_name("NB_MINES")
                .help("Sets the number of mines in the board")
                .takes_value(true),
        )
        .get_matches();

    let height = match matches.value_of("height") {
        Some(param) => match param.parse()? {
            n @ 2..=30 => n,
            _ => Err(MineError::SizeError)?,
        },
        None => 8,
    };

    let width = match matches.value_of("width") {
        Some(param) => match param.parse()? {
            n @ 2..=24 => n,
            _ => Err(MineError::SizeError)?,
        },
        None => 8,
    };

    let nb_mines_limit = height * width - 2;
    let nb_mines = match matches.value_of("nb_mines") {
        Some(param) => match param.parse()? {
            n if n > 0 && n < nb_mines_limit => n,
            _ => Err(MineError::NbMinesError)?,
        },
        None => max(height, width),
    };

    Ok(GameConfig {
        height,
        width,
        nb_mines,
    })
}
