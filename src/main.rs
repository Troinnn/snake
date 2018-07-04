extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

mod objects;

use objects::{Snake, Field, App, Direction, Apple};
use opengl_graphics::OpenGL;
use glutin_window::GlutinWindow;
use opengl_graphics::GlGraphics;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings, EventLoop};
use piston::input::*;


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

    let mut app = App {
        gl: GlGraphics::new(opengl),
        field: Field::new(height),
        snake: Snake::new(),
        apple: Apple::new(height),
        scores: 0,
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(_u) = e.update_args() {
            app.snake.step_move(app.field.scalar);
            app.check_coll();
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Left => {
                    app.snake.set_direct(Direction::Left);
                }
                Key::Right => {
                    app.snake.set_direct(Direction::Right);
                }
                Key::Up => {
                    app.snake.set_direct(Direction::Up);
                }
                Key::Down => {
                    app.snake.set_direct(Direction::Down);
                }
                _ => {}
            }
        };
    }
}
