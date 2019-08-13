use crate::{error::MineError, Result};

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
pub enum Mode {
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

	fn reset(&self) -> Game {
		let config = Config {
			height: self.field.height,
			width: self.field.width,
			nb_mines: self.nb_mines,
		};
		Self::new(config)
	}

	fn change_mode(&mut self, screen: &mut Output) -> Result<()> {
		match self.mode {
			Mode::Flag => {
				self.mode = Mode::Normal;
				screen.update_mode(&self.mode)?;
			}
			Mode::Normal => {
				self.mode = Mode::Flag;
				screen.update_mode(&self.mode)?;
			}
		}
		Ok(())
	}

	pub fn is_won(&self) -> bool {
		self.field.nb_of_unreveiled_cells() == self.nb_mines
	}

}

pub fn run(mut game: Game) -> Result<()> {
	'outer: loop {
		let input = stdin();
		let mut screen = Output::new(game.field.height, 4)?;
		screen.render_field(&game.field)?;
		screen.update_mode(&game.mode)?;
		screen.prompt_info()?;
		for e in input.events() {
			if e.is_err() {
				println!("{:?}", e);
				continue;
			}
			let event = e.map_err(|_| MineError::InputError)?;
			if let Event::Key(Key::Esc) = event {
				screen.reposition_cursor()?;
				return Ok(());
			} else if let Event::Key(Key::Char('r')) = event {
				game = game.reset();
				continue 'outer;
			}
			match &game.mode {
				Mode::Normal => match event {
					Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)) if y > 1 && x > 1 => {
						let (y, x) = screen.convert_coordinates((y as usize, x as usize));
						if game.field.position_is_valid(y, x) {
							game.field.show_cells(y, x);
							if game.field.cell_has_mine(y, x) {
								screen.render_field(&game.field)?;
								screen.prompt_end("Boom, you lost...\n")?;
								break 'outer;
							}
							screen.render_field(&game.field)?;
						}
					}
					Event::Key(Key::Char('f')) => game.change_mode(&mut screen)?,
					_ => {}
				},
				Mode::Flag => match event {
					Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)) if y > 1 && x > 1 => {
						let (y, x) = screen.convert_coordinates((y as usize, x as usize));
						if game.field.position_is_valid(y, x) {
							let cell = &mut game.field.cells[y][x];
							cell.toggle_flag();
							screen.render_field(&game.field)?;
						}
					}
					Event::Key(Key::Char('f')) => game.change_mode(&mut screen)?,
					_ => {}
				},
			}

			if game.is_won() {
				screen.prompt_end("You won, congrats!\n")?;
				break 'outer;
			}
		}

	}
	Ok(())
}
