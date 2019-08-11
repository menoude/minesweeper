use crate::game::field::Field;

use std::{
	fmt::Write,
	io::{stdout, Stdout},
};

use termion::{
	clear, cursor,
	cursor::HideCursor,
	input::MouseTerminal,
	raw::{IntoRawMode, RawTerminal},
};

pub struct Output {
	out: MouseTerminal<RawTerminal<Stdout>>,
}

impl Output {
	pub fn new() -> Self {
		let out = MouseTerminal::from(stdout().into_raw_mode().unwrap());
		HideCursor::from(stdout());
		println!("{}", clear::All);
		Output { out }
	}

	pub fn render_field(&mut self, field: &Field) {
		print!("{}{}", clear::All, cursor::Goto(1, 1));

		let mut buffer = String::with_capacity(field.height * (field.width + 2));
		for line in field.cells.iter() {
			for cell in line.iter() {
				write!(&mut buffer, "{}", cell).unwrap();
			}
			write!(&mut buffer, "\n\r").unwrap();
		}
		println!("{}", buffer);
	}
}
