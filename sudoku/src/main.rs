extern crate glutin_window;
mod gameboard;
mod position;
mod solver;

use std::path::PathBuf;

use gameboard::prelude::*;

use glutin_window::GlutinWindow;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::{EventLoop, RenderEvent, WindowSettings};

fn get_assets_path() -> PathBuf {
    let cwd = std::env::current_dir().expect("Could not access current directory.");
    let mut possible_dir = cwd;
    possible_dir.push("assets");
    if !possible_dir.exists() {
        possible_dir.pop();
        possible_dir.push("sudoku/assets");
        assert!(possible_dir.exists(), "Can't find `assets` folder.");
    }
    possible_dir
}

fn main() {
    // graphics rendering settings
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", (640, 480))
        .exit_on_esc(true)
        .graphics_api(opengl);
    // .vsync(true);

    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    // initialize Board
    let sudoku_board = SudokuBoard::new();
    let mut sudoku_board_controller = SudokuEventHandler::new(sudoku_board);
    let sudoku_board_view_settings = SudokuViewSettings::new();
    let sudoku_view = SudokuViewRenderer::new(sudoku_board_view_settings);

    // load font settings for text
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let mut assets_path = get_assets_path();
    assets_path.push("FiraSans-Regular.ttf");
    let glyphs = &mut GlyphCache::new(assets_path.as_path(), (), texture_settings)
        .expect("Could not load font");

    while let Some(e) = events.next(&mut window) {
        // pass event into our board controller
        sudoku_board_controller.handle_event(
            sudoku_view.settings.position,
            sudoku_view.settings.size,
            &e,
        );
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                // redraw the current board
                sudoku_view.draw(&sudoku_board_controller, glyphs, &c, g);
            });
        }
    }
}
