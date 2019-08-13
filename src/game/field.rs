use crate::game::cell::{Aspect, Cell, Content};

use rand::random;

use std::cmp::min;

#[derive(Debug)]
pub struct Field {
    pub cells: Vec<Vec<Cell>>,
    pub nb_cells: usize,
    pub height: usize,
    pub width: usize,
    pub nb_unreveiled_cells: usize,
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
            height,
            width,
            nb_unreveiled_cells: nb_cells,
        }
    }

    pub fn populate_with_mines(mut self, mut nb_mines: usize) -> Self {
        while nb_mines > 0 {
            let random_cell_nb = random::<usize>() % self.nb_cells;
            let random_height = random_cell_nb / self.width;
            let random_width = random_cell_nb % self.width;
            let random_cell = &self.cells[random_height][random_width];
            if random_cell.content == Content::Empty {
                self.place_mine(random_height, random_width);
                nb_mines -= 1;
            }
        }
        self
    }

    fn place_mine(&mut self, mine_row: usize, mine_col: usize) {
        self.cells[mine_row][mine_col].set_mine();
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

    pub fn cell_has_mine(&self, y: usize, x: usize) -> bool {
        self.cells[y][x].has_mine()
    }

    pub fn show_cells(&mut self, y: usize, x: usize) {
        let cell = &mut self.cells[y][x];
        if cell.is_visible() {
            return;
        }
        cell.set_visible();
        self.nb_unreveiled_cells -= 1;
        if !cell.has_mine() && !cell.has_adjacent_mine() {
            let adjacent_positions = self.get_adjacent_positions(y, x);
            for (row, col) in adjacent_positions {
                self.show_cells(row, col);
            }
        }
    }
}
