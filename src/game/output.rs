use crate::game::{
	cell::{Aspect, Content},
	field::Field,
};

use std::{
	fmt::Write,
	io::{stdout, Stdout, Write as Print},
};

use termion::{
	clear, color, cursor,
	cursor::HideCursor,
	input::MouseTerminal,
	raw::{IntoRawMode, RawTerminal},
};

pub struct Output {
	pub characters_width: usize,
	out: MouseTerminal<HideCursor<RawTerminal<Stdout>>>,
	mode_row: u16,
	prompt_row: u16,
}

impl Output {
	pub fn new(height: usize, characters_width: usize) -> Self {
		let out = MouseTerminal::from(HideCursor::from(stdout().into_raw_mode().unwrap()));
		println!("{}", clear::All);
		Output {
			characters_width,
			out,
			mode_row: height as u16 + 3,
			prompt_row: height as u16 + 4,
		}
	}

	pub fn render_field(&mut self, field: &Field) {
		print!(
			"{}{}{}",
			cursor::Goto(1, self.mode_row - 2),
			clear::BeforeCursor,
			cursor::Goto(1, 1)
		);

		let mut buffer =
			String::with_capacity(field.nb_cells * self.characters_width + (field.height + 2) * 2);
		let horizontal_edge = "=".repeat(field.width * self.characters_width + 2);
		write!(&mut buffer, "{}\n\r", horizontal_edge).unwrap();
		for line in field.cells.iter() {
			write!(&mut buffer, "+").unwrap();
			for cell in line.iter() {
				let color = match (cell.aspect, cell.content) {
					(Aspect::Hidden, _) => color::Fg(color::White).to_string(),
					(Aspect::Flagged, _) => color::Fg(color::Yellow).to_string(),
					(Aspect::Visible, Content::Mine) => color::Fg(color::Red).to_string(),
					(Aspect::Visible, Content::Empty) => color::Fg(color::Green).to_string(),
				};
				write!(
					&mut buffer,
					"{}{:^2$}{reset}",
					color,
					cell.to_string(),
					self.characters_width,
					reset = color::Fg(color::Reset)
				)
				.unwrap();
			}
			write!(&mut buffer, "+\n\r").unwrap();
		}
		write!(&mut buffer, "{}", horizontal_edge).unwrap();
		println!("{}", buffer);
	}

	pub fn prompt_mode(&mut self, message: &str) {
		print!("{}{}", cursor::Goto(1, self.mode_row), clear::CurrentLine);
		print!("{}{}\r", color::Fg(color::White), message);
		self.out.flush().unwrap();
	}

	pub fn prompt_message(&mut self, message: &str) {
		print!("{}{}", cursor::Goto(1, self.prompt_row), clear::CurrentLine);
		print!("{}{}\r", color::Fg(color::White), message);
		self.out.flush().unwrap();
	}

	pub fn convert_coordinates(&self, (mut y, mut x): (usize, usize)) -> (usize, usize) {
		y -= 2;
		x = (x - 2) / self.characters_width;
		(y, x)
	}
}
