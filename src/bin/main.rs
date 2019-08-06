use minesweeper::Field;

fn main() {
    let field = Field::new(5, 5).populate_with_mines(5);
    println!("{}", field);
}
