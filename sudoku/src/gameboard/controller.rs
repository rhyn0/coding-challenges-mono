//! Controller to parse events and pass relevant info to the object

use super::entity;
use crate::position;
use piston::input::{Button, Key, MouseButton};
use piston::GenericEvent;

/// Controller object to handle Event logic
#[derive(Debug)]
pub struct SudokuEventHandler {
    /// wrapped main object
    pub board: entity::SudokuBoard,
    /// selected cell that user wants to edit
    pub selected_cell: Option<position::Position>,
    /// Stores last mouse cursor position.
    cursor_pos: [f64; 2],
}

impl SudokuEventHandler {
    pub const fn new(board: entity::SudokuBoard) -> Self {
        Self {
            board,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }
    /// Return that given x,y coordinate is on the board of square `size`.
    ///
    /// Assumes that x,y position have already been converted to be relative to board edge
    fn position_on_board(x: f64, y: f64, size: f64) -> bool {
        x >= 0.0 && x < size && y >= 0.0 && y < size
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn handle_event<E: GenericEvent>(
        &mut self,
        board_position: [f64; 2],
        size: f64,
        event: &E,
    ) {
        if let Some(pos) = event.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        // calculate where the mouse click is on mouse click events
        // and only on left button clicks
        if Some(Button::Mouse(MouseButton::Left)) == event.press_args() {
            // board position is the offset of board top/left corner
            // to window top left corner. Find the relative cursor position on board
            let x = self.cursor_pos[0] - board_position[0];
            let y = self.cursor_pos[1] - board_position[1];

            // make sure calculated relative position is on board
            if Self::position_on_board(x, y, size) {
                // turn pixels into row column
                let cell_x = (x / size * 9.0) as usize;
                let cell_y = (y / size * 9.0) as usize;
                self.selected_cell = Some(position::Position::new(cell_x, cell_y));
            }
        }
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if let Some(pos) = self.selected_cell {
                // Set cell value.
                match key {
                    Key::D1 => self.board.set(pos, 1),
                    Key::D2 => self.board.set(pos, 2),
                    Key::D3 => self.board.set(pos, 3),
                    Key::D4 => self.board.set(pos, 4),
                    Key::D5 => self.board.set(pos, 5),
                    Key::D6 => self.board.set(pos, 6),
                    Key::D7 => self.board.set(pos, 7),
                    Key::D8 => self.board.set(pos, 8),
                    Key::D9 => self.board.set(pos, 9),
                    _ => {}
                }
            }
        }
    }
}
