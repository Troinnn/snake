use models::{field::Field, snake::{Snake, Apple, Direction}};
use piston::input::{RenderArgs, Button, Key, UpdateArgs};
use opengl_graphics::GlGraphics;
use models::state::{GameState, GameCmd};
use piston::input::Button::Keyboard;
use std::collections::VecDeque;

pub struct Scene {
    pub field: Field,
    pub snake: Snake,
    pub apple: Apple,
}

impl Scene {
    pub fn new(height: i32) -> Self {
        Scene {
            field: Field::new(height),
            snake: Snake::new(),
            apple: Apple::new(height),
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, (w, h): (f64, f64)) {
        use graphics::*;

        const GREEN: [f32; 4] = [0., 1., 0., 0.7];
        const BLACK: [f32; 4] = [0., 0., 0., 1.];
        gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
        });

        self.field.render(gl, args);
        self.snake.render(gl, args);
        self.apple.render(gl, args);
    }
    pub fn key_event(&mut self, args: &Button, fps: usize) -> GameState {
        match args {
            Keyboard(k) => {
                match k {
                    Key::Up => {
                        self.snake.set_direct(Direction::Up);
                    }
                    Key::Down => {
                        self.snake.set_direct(Direction::Down);
                    }
                    Key::Left => {
                        self.snake.set_direct(Direction::Left);
                    }
                    Key::Right => {
                        self.snake.set_direct(Direction::Right);
                    }
                    Key::Escape => {
                        return GameState::Menu;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        GameState::None
    }

    pub fn update(&mut self, (w, h): (f64, f64), fps: usize) -> GameCmd {
        self.snake.update(self.field.scalar, (w, h), fps);
        self.check_coll()
    }

    pub fn check_coll(&mut self) -> GameCmd {
        let mut result = GameCmd::Default;
        if self.apple.x_pos == self.snake.body.front().expect("Косяк").clone().0 &&
            self.apple.y_pos == self.snake.body.front().expect("Косяк").clone().1 {
            self.apple.reroll(self.field.scalar);
            self.snake.body.push_back((self.apple.x_pos, self.apple.y_pos));
            result = GameCmd::Ok;
        }

        let mut squares: VecDeque<(i32, i32)> = VecDeque::new();

        for rect in self.snake.body.iter() {
            squares.push_back(*rect);
        }
        squares.pop_front().expect("Нет первого элемента!");
        for square in squares {
            if self.snake.body.front()
                .expect("Косяк").clone().0 == square.0 &&
                self.snake.body.front().expect("Косяк").clone().1 == square.1 {
                self.snake.body.clear();
                self.snake = Snake::new();
                self.apple = Apple::new(self.field.scalar);
                result = GameCmd::Reset
            }
        }
        result
    }
}