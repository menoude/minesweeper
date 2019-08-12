pub mod cell;
pub mod field;
pub mod output;

use field::Field;
use output::Output;

use std::io::stdin;

use termion::{
	event::{Event, Key, MouseButton, MouseEvent},
	input::TermRead,
};

#[derive(PartialEq)]
enum Mode {
	Flag,
	Normal,
}

pub struct Config {
	pub height: usize,
	pub width: usize,
	pub nb_mines: usize,
}

pub struct Game {
	nb_mines: usize,
	field: Field,
	mode: Mode,
}

impl Game {
	pub fn new(
		Config {
			height,
			width,
			nb_mines,
		}: Config,
	) -> Self {
		let field = Field::new(height, width).populate_with_mines(nb_mines);
		Game {
			nb_mines,
			field,
			mode: Mode::Normal,
		}
	}

	pub fn run(mut self) -> Result<(), std::io::Error> {
		let input = stdin();
		let mut screen = Output::new(self.field.height, 4);
		screen.render_field(&self.field);
		screen.prompt_mode("Normal mode");
		for e in input.events() {
			if e.is_err() {
				println!("{:?}", e);
				continue;
			}
			let event = e.unwrap();
			if let Event::Key(Key::Esc) = event {
				return Ok(());
			}
			match &self.mode {
				Mode::Normal => match event {
					Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)) => {
						let (y, x) = screen.convert_coordinates((y as usize, x as usize));
						if self.field.position_is_valid(y, x) {
							self.field.show_cells(y, x);
							if self.field.cell_has_mine(y, x) {
								screen.render_field(&self.field);
								screen.prompt_message("Boom, you lost...\n");
								break;
							}
							screen.render_field(&self.field);
						} else {
							screen
								.prompt_message(&format!("Out of bound position: ({}, {})", y, x));
						}

					}
					Event::Key(Key::Char('f')) => self.change_mode(&mut screen),
					_ => {}
				},
				Mode::Flag => match event {
					Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)) => {
						let (y, x) = screen.convert_coordinates((y as usize, x as usize));
						if self.field.position_is_valid(y, x) {
							let cell = &mut self.field.cells[y][x];
							cell.toggle_flag();
							screen.render_field(&self.field);
						} else {
							screen
								.prompt_message(&format!("Out of bound position: ({}, {})", y, x));
						}
					}
					Event::Key(Key::Char('f')) => self.change_mode(&mut screen),
					_ => {}
				},
			}

			if self.field.nb_of_unreveiled_cells() == self.nb_mines {
				screen.prompt_message("You won, congrats!\n");
				break;
			}
		}
		Ok(())
	}

	fn change_mode(&mut self, screen: &mut Output) {
		match self.mode {
			Mode::Flag => {
				self.mode = Mode::Normal;
				screen.prompt_mode("Normal mode");
			}
			Mode::Normal => {
				self.mode = Mode::Flag;
				screen.prompt_mode("Flag mode");
			}
		}
	}
}
