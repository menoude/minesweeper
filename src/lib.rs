use rand::random;

use std::{
    cmp::min,
    fmt,
    fmt::Write,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Content {
    Mine,
    Empty,
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let character = match self {
            Content::Mine => 'X',
            Content::Empty => '.',
        };
        write!(f, "{}", character)
    }
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    content: Content,
    adjacent_mines: u16,
    visible: bool,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Cell {
                content: Content::Empty,
                adjacent_mines,
                ..
            } => write!(f, "{} ", adjacent_mines),
            Cell { content, .. } => write!(f, "{} ", content),
        }
    }
}

#[derive(Debug)]
pub struct Field {
    cells: Vec<Vec<Cell>>,
    nb_cells: usize,
    height: usize,
    width: usize,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut buffer = String::with_capacity(self.cells.len() * self.cells[0].len() * 4);
        for line in self.cells.iter() {
            for cell in line.iter() {
                write!(&mut buffer, "{}", cell).unwrap();
            }
            writeln!(&mut buffer).unwrap();
        }
        write!(f, "{}", buffer)
    }
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        if width > 100 || height > 100 {
            panic!("Size too large, limit is set to 100x100.")
        }

        let mut line = Vec::with_capacity(width as usize);
        for _ in 0..width {
            line.push(Cell {
                content: Content::Empty,
                adjacent_mines: 0,
                visible: false,
            });
        }
        let mut board = Vec::with_capacity(height as usize);
        for _ in 0..height {
            board.push(line.clone());
        }
        let nb_cells = board.len() * line.len();
        Field {
            cells: board,
            nb_cells,
            height,
            width,
        }
    }

    pub fn populate_with_mines(mut self, nb_mines: usize) -> Self {
        let mut remaining_mines = nb_mines;
        while remaining_mines > 0 {
            let random_cell_nb = random::<usize>() % self.nb_cells;
            let random_height = random_cell_nb / self.height;
            let random_width = random_cell_nb % self.height;
            let random_cell = &mut self.cells[random_height][random_width].content;
            if *random_cell == Content::Empty {
                self.place_mine(random_height, random_width);
                remaining_mines -= 1;
            }
        }
        self
    }

    fn place_mine(&mut self, mine_row: usize, mine_col: usize) {
        self.cells[mine_row][mine_col].content = Content::Mine;

        let upper_left = (mine_row.saturating_sub(1), mine_col.saturating_sub(1));
        let lower_right = (
            min(mine_row + 1, self.height - 1),
            min(mine_col + 1, self.width - 1),
        );
        for row in upper_left.0..=lower_right.0 {
            for col in upper_left.1..=lower_right.1 {
                if (row, col) == (mine_row, mine_col) {
                    continue;
                }
                self.cells[row][col].adjacent_mines += 1;
            }
        }
    }
}
