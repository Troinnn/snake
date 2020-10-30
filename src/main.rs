extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use opengl_graphics::{OpenGL, GlGraphics};
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;

mod models;

use models::app::App;

fn main() {
    let opengl: OpenGL = OpenGL::V2_1;
    let width: i32 = 800;
    let height: i32 = 600;
    let mut window: GlutinWindow = WindowSettings::new("SG", [width as u32, height as u32])
        .opengl(opengl)
        .srgb(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(window, opengl, height);
    app.run();
}
