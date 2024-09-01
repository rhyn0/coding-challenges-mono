//! Gameboard object that implements business logic for the game.

use std::{
    borrow::Borrow,
    fmt::{Debug, Display, Write},
};

use crate::position;

pub const GAME_SIZE: usize = 9;

/// Per cell info and logic
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// value in the cell
    value: u8,
    /// loaded from problem
    pub loaded: bool,
    /// whether the value of this cell is valid
    /// Note that if loaded is true, this value must be true
    pub valid: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            value: 0,
            loaded: false,
            valid: true,
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SC")
            .field("value", &self.value)
            .field("loaded", &self.loaded)
            .field("valid", &self.valid)
            .finish()
    }
}

impl Cell {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub const fn is_empty(&self) -> bool {
        self.value == 0
    }
}

/// Store board info
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct SudokuBoard {
    /// cell chars for the board
    pub cells: [[Cell; GAME_SIZE]; GAME_SIZE],
    /// Whether the game is over or not
    pub completed: bool,
}

impl Display for SudokuBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "[");
        for row in self.cells {
            let _ = writeln!(
                f,
                "[{}],",
                row.iter().fold(String::new(), |mut acc, cell| {
                    let _ = write!(&mut acc, "{}", cell.value);
                    acc
                })
            );
        }

        writeln!(f, "]")
    }
}

impl SudokuBoard {
    pub fn new() -> Self {
        Self::default()
    }
    /// Given a Position, get the cell at the appropriate location
    pub fn get(&self, pos: position::Position) -> &Cell {
        self.cells[pos.y][pos.x].borrow()
    }
    /// Gets the character at cell location.
    pub fn char(&self, pos: position::Position) -> Option<char> {
        Some(match self.get(pos).value {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => return None,
        })
    }
    pub fn check_completion(&self) -> bool {
        // check for completion
        self.cells.iter().flatten().all(|c| c.valid && c.value != 0)
    }
    /// Set cell value.
    pub fn set(&mut self, pos: position::Position, val: u8) {
        if !self.get(pos).loaded {
            self.validate(pos, val);
            self.cells[pos.y][pos.x].value = val;
        }
        self.completed = self.check_completion();
    }
    /// Update single Cell according to rules of the game
    fn validate(&mut self, pos: position::Position, val: u8) {
        // check that set value does not already exist in the row
        // early exit if invalid
        for i in 0..GAME_SIZE {
            if i == pos.y {
                continue;
            }
            if self.cells[i][pos.x].value == val {
                self.cells[pos.y][pos.x].valid = false;
                return;
            }
        }
        // check that set value does not already exist in the column
        for j in 0..GAME_SIZE {
            if j == pos.x {
                continue;
            }
            if self.cells[pos.y][j].value == val {
                self.cells[pos.y][pos.x].valid = false;
                return;
            }
        }
        // check that set value not already exist in the box
        let (box_row, box_col) = (pos.y / 3, pos.x / 3);
        for i in 3 * box_row..3 * box_row + 3 {
            for j in 3 * box_col..3 * box_col + 3 {
                if i == pos.y && j == pos.x {
                    continue;
                }
                if self.cells[i][j].value == val {
                    self.cells[pos.y][pos.x].valid = false;
                    return;
                }
            }
        }

        // if no prior rule break, then its valid
        self.cells[pos.y][pos.x].valid = true;
    }
    #[allow(dead_code)]
    /// Load a new game board from the SDM file in `filename`
    pub fn load_sdm(filename: &str) -> Self {
        let data = std::fs::read_to_string(filename).expect("failed to read SDM file");
        let mut cells = [[Cell::default(); GAME_SIZE]; GAME_SIZE];
        let mut row = 0;
        let mut col = 0;
        for c in data.trim_end().chars() {
            if col == GAME_SIZE {
                col = 0;
                row += 1;
            }
            let value = u8::try_from(c.to_digit(10).unwrap()).unwrap();
            cells[row][col] = Cell {
                value,
                loaded: value != 0,
                valid: value != 0,
            };
            col += 1;
        }
        Self {
            cells,
            ..Default::default()
        }
    }
    #[allow(dead_code)]
    pub fn from_cells(cells: [[u8; GAME_SIZE]; GAME_SIZE]) -> Self {
        let mut ret = Self::new();
        for (i, row) in cells.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                ret.cells[i][j] = Cell {
                    value: col,
                    loaded: col != 0,
                    valid: col != 0,
                };
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use position::Position;

    use super::*;
    use std::env;

    #[test]
    fn test_cell_new() {
        let cell = Cell::default();
        assert_eq!(cell.loaded, false);
        assert_eq!(cell.value, 0);
    }
    #[test]
    fn test_board_new() {
        let board = SudokuBoard::new();
        assert_eq!(board.cells[0], [Cell::default(); GAME_SIZE]);
    }
    #[test]
    fn test_load_sdm() {
        let cwd = env::current_dir().expect("Couldn't read current directory");
        let sdm_path: &str;
        if cwd.ends_with("sudoku") {
            sdm_path = "static/example.sdm"
        } else {
            sdm_path = "sudoku/static/example.sdm"
        }
        let board = SudokuBoard::load_sdm(sdm_path);
        let expected = SudokuBoard::from_cells([
            [0, 1, 6, 4, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 9, 0, 0, 0],
            [4, 0, 0, 0, 0, 0, 0, 6, 2],
            [0, 7, 0, 2, 3, 0, 1, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 3],
            [0, 0, 3, 0, 8, 7, 0, 4, 0],
            [9, 6, 0, 0, 0, 0, 0, 0, 5],
            [0, 0, 0, 8, 0, 0, 0, 0, 7],
            [0, 0, 0, 0, 0, 6, 8, 2, 0],
        ]);
        for (row, expected_row) in board.cells.iter().zip(expected.cells.iter()) {
            assert_eq!(row, expected_row);
        }
    }
    #[test]
    fn test_invalid_set_row() {
        let mut board = SudokuBoard::new();
        let valid_position = Position::new(0, 1);
        let invalid_position = Position::new(5, 1);
        board.set(valid_position, 2);
        // in the same row, set another entry to be 0
        board.set(invalid_position, 2);
        assert_eq!(board.get(valid_position).valid, true);
        assert_eq!(board.get(invalid_position).valid, false);
    }
    #[test]
    fn test_invalid_set_col() {
        let mut board = SudokuBoard::new();
        let valid_position = Position::new(5, 0);
        let invalid_position = Position::new(5, 1);
        board.set(valid_position, 2);
        // in the same row, set another entry to be 0
        board.set(invalid_position, 2);
        assert_eq!(board.get(valid_position).valid, true);
        assert_eq!(board.get(invalid_position).valid, false);
    }
    #[test]
    fn test_invalid_set_square() {
        let mut board = SudokuBoard::new();
        let valid_position = Position::new(0, 1);
        let invalid_position = Position::new(2, 1);
        board.set(valid_position, 2);
        // in the same row, set another entry to be 0
        board.set(invalid_position, 2);
        assert_eq!(board.get(valid_position).valid, true);
        assert_eq!(board.get(invalid_position).valid, false);
    }
    #[test]
    fn test_initial_board_is_not_complete() {
        let mut board = SudokuBoard::new();
        assert_eq!(board.completed, false);
        // random set to trigger validation
        board.set(Position::new(1, 0), 2);
        assert_eq!(board.completed, false);
    }
}
