pub mod cell;
pub mod content;
pub mod field;
pub mod output;

use field::Field;
use output::Output;

use std::io::{stdin, Stdin};

use termion::{
	event::{Event, Key, MouseButton, MouseEvent},
	input::TermRead,
};


#[derive(PartialEq)]
enum Mode {
	Flag,
	Normal,
}

pub struct Game {
	revealed_cells: usize,
	nb_mines: usize,
	characters_height: usize,
	characters_width: usize
}

impl Game {
	
pub fn run(height: usize, width:usize, nb_mines: usize) -> Result<(), std::io::Error> {
	let field = Field::new(height, width).populate_with_mines(nb_mines);
	let mut screen = Output::new();
	screen.render_field(&field);
	let external_input = stdin();
	Self::event_loop(screen, external_input, field)
}

fn event_loop(
	mut screen: Output,
	external_input: Stdin,
	mut field: Field,
) -> Result<(), std::io::Error> {
	let mut mode = Mode::Normal;
	for e in external_input.events() {
		if e.is_err() {
			println!("{:?}", e);
			continue;
		}
		match (&mode, e.unwrap()) {
			(Mode::Normal, Event::Key(Key::Char('f'))) => {
				println!("Flag mode\r");
				mode = Mode::Flag
			}
			(Mode::Flag, Event::Key(Key::Char('f'))) => {
				println!("Normal mode\r");
				mode = Mode::Normal
			}
			(Mode::Flag, Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y))) => {
				let (y, x) = field.convert_coordinates((y as usize, x as usize));
				if field.position_is_valid(y, x) {
					let cell = &mut field.cells[y][x];
					cell.toggle_flag();
					screen.render_field(&field);
				}
			}
			(Mode::Normal, Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y))) => {
				let (y, x) = field.convert_coordinates((y as usize, x as usize));
				if field.position_is_valid(y, x) {
					field.show_cell(y, x);
					if field.cell_has_mine(y, x) {
						screen.render_field(&field);
						break;
					}
					screen.render_field(&field);
				}
			}
			(_, Event::Key(Key::Esc)) => return Ok(()),
			_ => {}
		}
		if field.is_revealed_entirely() {
			println!("You won, congrats!\r");
			break;
		}
	}
	Ok(())
}
}
