use minesweeper::run;
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
        exit(1);
    }
}
