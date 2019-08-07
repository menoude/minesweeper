use clap::{App, Arg};

pub struct Params {
    pub height: usize,
    pub width: usize,
    pub nb_mines: usize,
}

pub fn get_args() -> Params {
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

    let height = matches
        .value_of("height")
        .and_then(|val| val.parse().ok())
        .unwrap_or(8);
    let width = matches
        .value_of("width")
        .and_then(|val| val.parse().ok())
        .unwrap_or(8);
    let nb_mines = matches
        .value_of("nb_mines")
        .and_then(|val| val.parse().ok())
        .unwrap_or(height * width / 2);

    if (width < 2 && height < 2) || height > 30 || width > 24 {
        panic!("Wrong size, please choose a size between 2x2 and 100x100.")
    } else if nb_mines > (height - 1) * (width - 1) {
        panic!("Number of mines shouldn't exceed half of the number of cells in the board.")
    }

    Params {
        height,
        width,
        nb_mines,
    }
}
