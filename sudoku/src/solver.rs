use crate::gameboard::prelude::SudokuBoard;
use crate::{position, Cell};

#[allow(dead_code)]
fn find_empty(board: &SudokuBoard) -> Option<position::Position> {
    board.cells.iter().enumerate().find_map(|(i, row)| {
        row.iter()
            .position(Cell::is_empty)
            .map(|x| position::Position::new(x, i))
    })
}
#[allow(dead_code)]
fn backtrack_helper(board: &mut SudokuBoard) -> bool {
    let empty_cell = find_empty(board);
    if empty_cell.is_none() {
        return true;
    }
    let pos = empty_cell.unwrap();

    for test_val in 1..=9 {
        board.set(pos, test_val);
        if board.get(pos).valid && backtrack_helper(board) {
            return true;
        }
        board.set(pos, 0);
    }
    false
}

/// Return a solved board if there is one.
///
/// Methodology is to use recursive backtracking.
///
/// Initial input is not modified.
#[allow(dead_code)]
pub fn backtrack_solve(board: &SudokuBoard) -> Option<SudokuBoard> {
    // clone the board so that we can edit it, solve it and then return a different board
    let mut our_board = *board;
    let _ = backtrack_helper(&mut our_board);
    if our_board.completed {
        Some(our_board)
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wildcat_load() {
        let board = SudokuBoard::load_sdm("static/wildcat.sdm");
        let expected = SudokuBoard::from_cells([
            [0, 0, 0, 2, 6, 0, 7, 0, 1],
            [6, 8, 0, 0, 7, 0, 0, 9, 0],
            [1, 9, 0, 0, 0, 4, 5, 0, 0],
            [8, 2, 0, 1, 0, 0, 0, 4, 0],
            [0, 0, 4, 6, 0, 2, 9, 0, 0],
            [0, 5, 0, 0, 0, 3, 0, 2, 8],
            [0, 0, 0, 3, 0, 0, 0, 7, 4],
            [0, 4, 0, 0, 5, 0, 0, 3, 6],
            [7, 0, 3, 0, 1, 8, 0, 0, 0],
        ]);
        assert_eq!(board, expected);
    }
    #[test]
    fn test_solve() {
        let board = SudokuBoard::load_sdm("static/wildcat.sdm");
        let result = backtrack_solve(&board);
        assert!(result.is_some());
        let solved_board = result.unwrap();
        assert!(solved_board.completed);
        assert!(solved_board
            .cells
            .iter()
            .all(|row| row.iter().all(|c| c.valid)));
    }
}
