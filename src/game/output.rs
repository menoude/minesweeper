use crate::{
	game::{
		cell::{Aspect, Cell, Content},
		field::Field,
		Mode,
	},
	Result,
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
static HORIZONTAL_BORDER: &str = "─";
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

enum Side {
	Top,
	Bottom,
}

impl Output {
	pub fn new(height: usize, characters_width: usize) -> Result<Self> {
		let out = MouseTerminal::from(HideCursor::from(stdout().into_raw_mode()?));
		println!("{}", clear::All);
		let result = Output {
			characters_width,
			out,
			mode_row: height as u16 + 3,
			prompt_row: height as u16 + 11,
		};
		Ok(result)
	}

	fn clear_field_instructions(&mut self) -> String {
		format!(
			"{}{}{}",
			cursor::Goto(1, self.mode_row - 2),
			clear::BeforeCursor,
			cursor::Goto(1, 1)
		)
	}

	pub fn render_field(&mut self, field: &Field) -> Result<()> {
		let mut buffer = String::with_capacity(
			(field.height + 5) * (field.width + 2) * self.characters_width * 4,
		);
		write!(buffer, "{}", self.clear_field_instructions())?;
		let horizontal_edge = HORIZONTAL_BORDER.repeat(field.width * self.characters_width);
		self.render_edge(&mut buffer, &horizontal_edge, Side::Top)?;
		for line in field.cells.iter() {
			self.render_line(&mut buffer, line)?;
		}
		self.render_edge(&mut buffer, &horizontal_edge, Side::Bottom)?;
		println!("{}", buffer);
		Ok(())
	}

	fn render_edge(&self, buffer: &mut String, horizontal_edge: &str, side: Side) -> Result<()> {
		let line = match side {
			Side::Top => String::from(TOP_LEFT_CORNER) + horizontal_edge + TOP_RIGHT_CORNER,
			Side::Bottom => {
				String::from(BOTTOM_LEFT_CORNER) + horizontal_edge + BOTTOM_RIGHT_CORNER
			}
		};
		writeln!(buffer, "{}\r", line)?;
		Ok(())
	}

	fn render_line(&self, buffer: &mut String, line: &[Cell]) -> Result<()> {
		write!(buffer, "{}", SIDE_BORDER)?;
		for &cell in line.iter() {
			self.render_cell(buffer, cell)?;
		}
		write!(buffer, "{}\n\r", SIDE_BORDER)?;
		Ok(())
	}

	fn render_cell(&self, buffer: &mut String, cell: Cell) -> Result<()> {
		let color = match (cell.aspect, cell.content) {
			(Aspect::Hidden, _) => color::Fg(color::White).to_string(),
			(Aspect::Flagged, _) => color::Fg(color::Yellow).to_string(),
			(Aspect::Visible, Content::Mine) => color::Fg(color::Red).to_string(),
			(Aspect::Visible, Content::Empty) => color::Fg(color::Green).to_string(),
		};
		write!(
			buffer,
			"{}{:^width$}{}",
			color,
			cell.to_string(),
			color::Fg(color::Reset),
			width = self.characters_width
		)?;
		Ok(())
	}

	fn clear_line_instructions(&mut self, line: u16) -> String {
		format!("{}{}\r", cursor::Goto(1, line), clear::CurrentLine)
	}

	pub fn update_mode(&mut self, mode: &Mode) {
		println!(
			"{}{}{}\r",
			self.clear_line_instructions(self.mode_row + 1),
			color::Fg(color::White),
			match mode {
				Mode::Normal => NORMAL_MODE,
				Mode::Flag => FLAG_MODE,
			},
		);
	}

	pub fn prompt_info(&mut self) -> Result<()> {
		print!(
			"{}{}",
			color::Fg(color::Reset),
			cursor::Goto(1, self.mode_row)
		);
		print!(
			"{}\r\n{}\r\n{}\r\n",
			TOP_MODE_BORDER, NORMAL_MODE, BOTTOM_MODE_BORDER
		);
		println!("{}\r\n", USAGE);
		Ok(())
	}

	pub fn prompt_end_message(&mut self, message: &str) {
		println!(
			"{}{}{}\r",
			self.clear_line_instructions(self.prompt_row),
			color::Fg(color::White),
			message
		);
	}

	pub fn reposition_cursor(&mut self) -> Result<()> {
		print!(
			"{}{}\r",
			cursor::Goto(1, self.prompt_row),
			clear::CurrentLine,
		);
		self.out.flush()?;
		Ok(())
	}

	pub fn convert_coordinates(&self, (mut y, mut x): (usize, usize)) -> (usize, usize) {
		y -= 2;
		x = (x - 2) / self.characters_width;
		(y, x)
	}
}
