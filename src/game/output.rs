use crate::game::{
	cell::{Aspect, Cell, Content},
	field::Field,
	Mode,
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

static TOP_RIGHT_CORNER: &str = "┐";
static TOP_LEFT_CORNER: &str = "┌";
static BOTTOM_LEFT_CORNER: &str = "└";
static BOTTOM_RIGHT_CORNER: &str = "┘";
static SIDE_BORDER: &str = "│";
static TOP_MODE_BORDER: &str =
	"┌──────── Mode ──────────────────┐";
static NORMAL_MODE: &str = "│ Normal mode                    │";
static FLAG_MODE: &str = "│ Flag mode                      │";
static BOTTOM_MODE_BORDER: &str =
	"└────────────────────────────────┘";
static USAGE: &str = "┌─────┬─ Usage ──────────────────┐\n\r\
                      │ f   │   change mode            │\n\r\
                      │ r   │   reset                  │\n\r\
                      │ esc │   quit                   │\n\r\
                      └─────┘──────────────────────────┘";

impl Output {
	pub fn new(height: usize, characters_width: usize) -> Self {
		let out = MouseTerminal::from(HideCursor::from(stdout().into_raw_mode().unwrap()));
		println!("{}", clear::All);
		Output {
			characters_width,
			out,
			mode_row: height as u16 + 3,
			prompt_row: height as u16 + 11,
		}
	}

	pub fn render_field(&mut self, field: &Field) {
		print!(
			"{}{}{}",
			cursor::Goto(1, self.mode_row - 2),
			clear::BeforeCursor,
			cursor::Goto(1, 1)
		);

		let mut buffer = String::with_capacity(
			(field.height + 5) * (field.width + 2) * self.characters_width * 4,
		);
		let horizontal_edge = "─".repeat(field.width * self.characters_width);
		write!(
			&mut buffer,
			"{}\n\r",
			String::from(TOP_LEFT_CORNER) + &horizontal_edge + TOP_RIGHT_CORNER
		)
		.unwrap();
		for line in field.cells.iter() {
			write!(&mut buffer, "{}", SIDE_BORDER).unwrap();
			for cell in line.iter() {
				write!(&mut buffer, "{}", self.render_cell(*cell)).unwrap();
			}
			write!(&mut buffer, "{}\n\r", SIDE_BORDER).unwrap();
		}
		write!(
			&mut buffer,
			"{}",
			String::from(BOTTOM_LEFT_CORNER) + &horizontal_edge + BOTTOM_RIGHT_CORNER
		)
		.unwrap();
		println!("{}\r", buffer);
	}

	fn render_cell(&self, cell: Cell) -> String {
		let color = match (cell.aspect, cell.content) {
			(Aspect::Hidden, _) => color::Fg(color::White).to_string(),
			(Aspect::Flagged, _) => color::Fg(color::Yellow).to_string(),
			(Aspect::Visible, Content::Mine) => color::Fg(color::Red).to_string(),
			(Aspect::Visible, Content::Empty) => color::Fg(color::Green).to_string(),
		};
		format!(
			"{}{:^width$}{}",
			color,
			cell.to_string(),
			color::Fg(color::Reset),
			width = self.characters_width
		)
	}

	pub fn convert_coordinates(&self, (mut y, mut x): (usize, usize)) -> (usize, usize) {
		y -= 2;
		x = (x - 2) / self.characters_width;
		(y, x)
	}

	pub fn update_mode(&mut self, mode: &Mode) {
		print!(
			"{}{}{}\r",
			cursor::Goto(1, self.mode_row + 1),
			clear::CurrentLine,
			color::Fg(color::White)
		);
		self.out.flush().unwrap();
		println!(
			"{}\r",
			match mode {
				Mode::Normal => NORMAL_MODE,
				Mode::Flag => FLAG_MODE,
			},
		);
	}

	pub fn prompt_info(&mut self) {
		let reset_all = format!("{}{}", color::Bg(color::Reset), color::Fg(color::Reset));
		print!("{}{}", reset_all, cursor::Goto(1, self.mode_row));
		print!(
			"{}\r\n{}\r\n{}\r\n",
			TOP_MODE_BORDER, NORMAL_MODE, BOTTOM_MODE_BORDER
		);
		self.out.flush().unwrap();
		println!("{}\r\n", USAGE);
	}

	pub fn prompt_end(&mut self, message: &str) {
		print!(
			"{}{}{}{}\r",
			cursor::Goto(1, self.prompt_row),
			clear::CurrentLine,
			color::Fg(color::White),
			message
		);
		self.out.flush().unwrap();
	}

	pub fn reposition_cursor(&mut self) {
		print!(
			"{}{}\r",
			cursor::Goto(1, self.prompt_row),
			clear::CurrentLine,
		);
		self.out.flush().unwrap();
	}

}
