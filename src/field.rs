use crate::{
    cell::{Aspect, Cell},
    content::Content,
};

use rand::random;

use std::{
    cmp::min,
    fmt,
    fmt::Write,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct Field {
    pub cells: Vec<Vec<Cell>>,
    nb_cells: usize,
    nb_revealed_cells: usize,
    nb_of_mines: usize,
    pub height: usize,
    pub width: usize,
    characters_height: usize,
    characters_width: usize,
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
    pub fn new(height: usize, width: usize) -> Self {
        let mut line = Vec::with_capacity(width as usize);
        for _ in 0..width {
            line.push(Cell {
                content: Content::Empty,
                adjacent_mines: 0,
                aspect: Aspect::Hidden,
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
            nb_revealed_cells: 0,
            nb_of_mines: 0,
            height,
            width,
            characters_height: 1,
            characters_width: 2,
        }
    }

    pub fn populate_with_mines(mut self, nb_mines: usize) -> Self {
        self.nb_of_mines = nb_mines;
        let mut remaining_mines = nb_mines;
        while remaining_mines > 0 {
            let random_cell_nb = random::<usize>() % self.nb_cells;
            let random_height = random_cell_nb / self.width;
            let random_width = random_cell_nb % self.width;
            let random_cell = &self.cells[random_height][random_width].content;
            if *random_cell == Content::Empty {
                self.place_mine(random_height, random_width);
                remaining_mines -= 1;
            }
        }
        self
    }

    fn place_mine(&mut self, mine_row: usize, mine_col: usize) {
        self.cells[mine_row][mine_col].content = Content::Mine;

        let adjacent_positions = self.get_adjacent_positions(mine_row, mine_col);
        for (row, col) in adjacent_positions {
            self.cells[row][col].adjacent_mines += 1;
        }
    }

    fn get_adjacent_positions(&self, center_row: usize, center_col: usize) -> Vec<(usize, usize)> {
        let upper_left = (center_row.saturating_sub(1), center_col.saturating_sub(1));
        let lower_right = (
            min(center_row + 1, self.height - 1),
            min(center_col + 1, self.width - 1),
        );
        let mut positions = Vec::new();
        for row in upper_left.0..=lower_right.0 {
            for col in upper_left.1..=lower_right.1 {
                if (row, col) == (center_row, center_col) {
                    continue;
                }
                positions.push((row, col));
            }
        }
        positions
    }

    pub fn position_is_valid(&self, y: usize, x: usize) -> bool {
        y < self.height && x < self.width
    }

    pub fn position_has_mine(&self, y: usize, x: usize) -> bool {
        self.cells[y][x].content == Content::Mine
    }

    pub fn convert_coordinates(&self, (mut y, mut x): (usize, usize)) -> (usize, usize) {
        y = (y - 1) / self.characters_height;
        x = (x - 1) / self.characters_width;
        println!("Position clicked: ({}, {})\r", y, x);
        (y, x)
    }

    pub fn show_cell(&mut self, y: usize, x: usize) {
        let cell = &mut self.cells[y][x];
        if cell.is_visible() {
            return;
        }
        cell.set_visible();
        self.nb_revealed_cells += 1;
        if !cell.has_mine() && !cell.has_adjacent_mine() {
            let adjacent_positions = self.get_adjacent_positions(y, x);
            for (row, col) in adjacent_positions {
                self.show_cell(row, col);
            }
        }
    }

    pub fn is_revealed_entirely(&self) -> bool {
        self.nb_revealed_cells + self.nb_of_mines == self.nb_cells
    }
}
