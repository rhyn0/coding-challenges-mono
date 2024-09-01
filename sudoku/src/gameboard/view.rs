//! Gameboard drawing logic.

use graphics::character::CharacterCache;
use graphics::types::Color;
use graphics::{Context, Graphics, Image, Line, Rectangle, Transformed};

use crate::position::{self, Position};

use super::controller::SudokuEventHandler;
use super::entity::GAME_SIZE;

/// Stores gameboard view settings.
#[derive(Debug)]
pub struct SudokuViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Selected cell background color.
    pub selected_cell_background_color: Color,
    /// Invalid cell background color.
    pub invalid_cell_background_color: Color,
    /// Invalid selected cell background color.
    pub invalid_selected_cell_background_color: Color,
    /// Text color
    pub text_color: Color,
    /// Selected cell background color.
    pub loaded_cell_background_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections.
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections.
    pub section_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
}

impl SudokuViewSettings {
    pub const fn new() -> Self {
        Self {
            position: [10.0; 2],
            size: 400.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
            text_color: [0.0, 0.0, 0.1, 1.0],
            loaded_cell_background_color: [1.0, 1.0, 1.0, 1.0],
            invalid_cell_background_color: [1.0, 0.0, 0.0, 1.0],
            invalid_selected_cell_background_color: [1.0, 0.0, 0.5, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
        }
    }
}

/// Actual view based on passed in settings, draws the updated view every render
#[derive(Debug)]
pub struct SudokuViewRenderer {
    /// render settings
    pub settings: SudokuViewSettings,
}

impl SudokuViewRenderer {
    pub const fn new(settings: SudokuViewSettings) -> Self {
        Self { settings }
    }
    #[allow(clippy::cast_precision_loss)]
    pub fn draw<G: Graphics, C>(
        &self,
        controller: &SudokuEventHandler,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
    ) where
        C: CharacterCache<Texture = G::Texture>,
    {
        let settings = &self.settings;
        let board_rect = [
            settings.position[0],
            settings.position[1],
            settings.size,
            settings.size,
        ];

        // Draw board background.
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);
        // Declare the format for cell and section lines.
        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);

        // Generate and draw the lines for the Sudoku Grid.
        for i in 0..GAME_SIZE {
            let x = settings.position[0] + (i as f64 * settings.size / 9.0); // this is GAME_SIZE as f64
            let y = settings.position[1] + (i as f64 * settings.size / 9.0);
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            let hline = [settings.position[0], y, x2, y];

            // Draw Section Lines instead of Cell Lines
            if (i % 3) == 0 {
                section_edge.draw(vline, &c.draw_state, c.transform, g);
                section_edge.draw(hline, &c.draw_state, c.transform, g);
            }
            // Draw the regular cell Lines
            else {
                cell_edge.draw(vline, &c.draw_state, c.transform, g);
                cell_edge.draw(hline, &c.draw_state, c.transform, g);
            }
        }
        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius).draw(
            board_rect,
            &c.draw_state,
            c.transform,
            g,
        );
        // Draw loaded cell background
        for i in 0..GAME_SIZE {
            for j in 0..GAME_SIZE {
                if controller.board.cells[i][j].loaded {
                    color_cell(
                        settings,
                        position::Position { x: j, y: i },
                        settings.loaded_cell_background_color,
                        c,
                        g,
                    );
                } else if !controller.board.cells[i][j].valid {
                    color_cell(
                        settings,
                        position::Position { x: j, y: i },
                        settings.invalid_cell_background_color,
                        c,
                        g,
                    );
                }
            }
        }
        // if there is a selected cell, highlight it
        if let Some(ind) = controller.selected_cell {
            let cell = controller.board.cells[ind.y][ind.x];
            let color = if cell.loaded {
                settings.loaded_cell_background_color
            } else if cell.valid {
                settings.selected_cell_background_color
            } else {
                settings.invalid_selected_cell_background_color
            };
            color_cell(settings, ind, color, c, g);
        };
        // Draw characters.
        let text_image = Image::new_color(settings.text_color);
        let cell_size = settings.size / 9.0;
        for row in 0..9 {
            for col in 0..9 {
                if let Some(ch) = controller.board.char(Position::new(col, row)) {
                    let pos_x = (col as f64).mul_add(cell_size, settings.position[0]) + 15.0;
                    let pos_y = (row as f64).mul_add(cell_size, settings.position[1]) + 34.0;
                    if let Ok(character) = glyphs.character(34, ch) {
                        let ch_x = pos_x + character.left();
                        // plane is normal cartesian x,y where positive y is upwards
                        // but we need to draw below the top left spot
                        let ch_y = pos_y - character.top();
                        let text_image = text_image.src_rect([
                            character.atlas_offset[0],
                            character.atlas_offset[1],
                            character.atlas_size[0],
                            character.atlas_size[1],
                        ]);
                        text_image.draw(
                            character.texture,
                            &c.draw_state,
                            c.transform.trans(ch_x, ch_y),
                            g,
                        );
                    }
                }
            }
        }
    }
}
#[allow(clippy::cast_precision_loss)]
/// Color an individual cell in the grid.
fn color_cell<G: Graphics>(
    settings: &SudokuViewSettings,
    ind: position::Position,
    color: [f32; 4],
    c: &Context,
    g: &mut G,
) {
    use graphics::Rectangle;

    let cell_size = settings.size / 9.0;
    let pos_x = ind.x as f64 * cell_size;
    let pos_y = ind.y as f64 * cell_size;
    let cell_rect = [
        settings.position[0] + pos_x,
        settings.position[1] + pos_y,
        cell_size,
        cell_size,
    ];
    Rectangle::new(color).draw(cell_rect, &c.draw_state, c.transform, g);
}
