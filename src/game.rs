pub mod cell;
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
	output: Output,
	field: Field,
	input: Stdin,
	mode: Mode,
}

impl Game {
	pub fn new((height, width): (usize, usize), nb_mines: usize) -> Self {
		let field = Field::new(height, width).populate_with_mines(nb_mines);
		let mut screen = Output::new(height, 3);
		screen.render_field(&field);
		let external_input = stdin();
		Game {
			nb_mines,
			revealed_cells: 0,
			output: screen,
			input: external_input,
			field,
			mode: Mode::Normal,
		}
	}

	pub fn run(mut self) -> Result<(), std::io::Error> {
		for e in self.input.events() {
			if e.is_err() {
				println!("{:?}", e);
				continue;
			}
			match (&self.mode, e.unwrap()) {
				(Mode::Normal, Event::Key(Key::Char('f'))) => {
					self.output.update_mode_prompt("Flag mode");
					self.mode = Mode::Flag
				}
				(Mode::Flag, Event::Key(Key::Char('f'))) => {
					self.output.update_mode_prompt("Normal mode");
					self.mode = Mode::Normal
				}
				(Mode::Flag, Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y))) => {
					let (y, x) = self.field.convert_coordinates(
						(y as usize, x as usize),
						self.output.characters_width,
					);
					if self.field.position_is_valid(y, x) {
						let cell = &mut self.field.cells[y][x];
						cell.toggle_flag();
						self.output.render_field(&self.field);
						self.mode = Mode::Normal;
						self.output.update_mode_prompt("Normal mode");
					} else {
						self.output
							.prompt(&format!("Out of bound position: ({}, {})", y, x));
					}
				}
				(Mode::Normal, Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y))) => {
					let (y, x) = self.field.convert_coordinates(
						(y as usize, x as usize),
						self.output.characters_width,
					);
					if self.field.position_is_valid(y, x) {
						self.field.show_cell(y, x);
						self.revealed_cells += 1;
						if self.field.cell_has_mine(y, x) {
							self.output.render_field(&self.field);
							self.output.prompt("Boom, you lost...\n");
							break;
						}
						self.output.render_field(&self.field);
					} else {
						self.output
							.prompt(&format!("Out of bound position: ({}, {})", y, x));
					}
				}
				(_, Event::Key(Key::Esc)) => return Ok(()),
				_ => {}
			}

			if self.revealed_cells + self.nb_mines == self.field.nb_cells {
				self.output.prompt("You won, congrats!\n");
				break;
			}
		}
		Ok(())
	}
}
