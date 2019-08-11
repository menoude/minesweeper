use crate::game::field::Field;

use std::{
	fmt::Write,
	io::{stdout, Stdout, Write as Print},
};

use termion::{
	clear, cursor,
	cursor::HideCursor,
	input::MouseTerminal,
	raw::{IntoRawMode, RawTerminal},
};

pub struct Output {
	pub characters_width: usize,
	out: MouseTerminal<RawTerminal<Stdout>>,
	mode_row: u16,
	prompt_row: u16,
	cursor_row: usize,
	cursor_col: usize,
}

impl Output {
	pub fn new(height: usize, characters_width: usize) -> Self {
		let out = MouseTerminal::from(stdout().into_raw_mode().unwrap());
		HideCursor::from(stdout());
		println!("{}", clear::All);
		Output {
			characters_width,
			out,
			cursor_col: 1,
			cursor_row: 1,
			mode_row: height as u16 + 2,
			prompt_row: height as u16 + 3,
		}
	}

	pub fn render_field(&mut self, field: &Field) {
		print!("{}{}{}", cursor::Goto(1, self.mode_row - 2), clear::BeforeCursor, cursor::Goto(1, 1));

		let mut buffer =
			String::with_capacity(field.nb_cells * self.characters_width + field.height * 2);
		for line in field.cells.iter() {
			for cell in line.iter() {
				write!(&mut buffer, "{0:1$}", cell, self.characters_width).unwrap();
			}
			write!(&mut buffer, "\n\r").unwrap();
		}
		println!("{}", buffer);
	}

	pub fn update_mode_prompt(&mut self, message: &str) {
		print!("{}{}", cursor::Goto(1, self.mode_row), clear::CurrentLine);
		print!("{}\r", message);
		self.out.flush().unwrap();
	}

	pub fn prompt(&mut self, message: &str) {
		print!("{}{}", cursor::Goto(1, self.prompt_row), clear::CurrentLine);
		print!("{}\r", message);
		self.out.flush().unwrap();
	}
}
