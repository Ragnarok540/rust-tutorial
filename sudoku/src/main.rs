#![deny(missing_docs)]

//! A sudoku game.

pub use crate::gameboard::Gameboard;
pub use crate::gameboard_controller::GameboardController;
pub use crate::gameboard_view::{GameboardView, GameboardViewSettings};

mod gameboard;
mod gameboard_controller;
mod gameboard_view;

use piston::{WindowSettings, EventLoop, RenderEvent};
use piston::event_loop::{EventSettings, Events};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};


fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", (640, 480))
        .exit_on_esc(true)
        .graphics_api(opengl);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new()).lazy(true);
    let mut gl = GlGraphics::new(opengl);

    //let gameboard = Gameboard::load_sdm("static/puzzle.sdm"); //Gameboard::new();
    let gameboard = Gameboard::load_sdm("static/puzzle2.sdm"); //Gameboard::new();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(gameboard_view.settings.position, gameboard_view.settings.size, &e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, glyphs, &c, g);
            });
        }
    }

    println!("{}", settings.get_exit_on_esc());
}
