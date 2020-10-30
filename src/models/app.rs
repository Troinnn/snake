use opengl_graphics::{OpenGL, GlGraphics};
use piston::{window::WindowSettings, input::*, event_loop::{Events, EventSettings, EventLoop}};
use std::collections::vec_deque::VecDeque;
use glutin_window::GlutinWindow;

use models::{field::Field, snake::{Snake, Apple, Direction}};

pub struct App {
    pub window: GlutinWindow,
    pub gl: GlGraphics,
    pub field: Field,
    pub snake: Snake,
    pub apple: Apple,
    pub scores: u32,
    pub speed: u64,
    pub events: Events,
}

impl App {
    pub fn new(window: GlutinWindow, opengl: OpenGL, height: i32) -> Self {
        App {
            window,
            gl: GlGraphics::new(opengl),
            field: Field::new(height),
            snake: Snake::new(),
            apple: Apple::new(height),
            scores: 0,
            speed: 6,
            events: Events::new(EventSettings::new()).ups(6),
        }
    }

    pub fn run(&mut self) {
        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }
            if let Some(_u) = e.update_args() {
                self.snake.step_move(self.field.scalar);
                self.check_coll();
            }
            if let Some(Button::Keyboard(key)) = e.press_args() {
                match key {
                    Key::Left => {
                        self.snake.set_direct(Direction::Left);
                    }
                    Key::Right => {
                        self.snake.set_direct(Direction::Right);
                    }
                    Key::Up => {
                        self.snake.set_direct(Direction::Up);
                    }
                    Key::Down => {
                        self.snake.set_direct(Direction::Down);
                    }
                    _ => {}
                }
            };
        }
    }

    pub fn render(&mut self, arg: &RenderArgs) {
        use graphics;
        let black: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(black, gl);
        });

        self.field.render(&mut self.gl, arg);
        self.snake.render(&mut self.gl, arg);
        self.apple.render(&mut self.gl, arg);
    }

    pub fn check_coll(&mut self) {
        if self.apple.x_pos == self.snake.body.front().expect("Косяк").clone().0 && self.apple.y_pos == self.snake.body.front().expect("Косяк").clone().1 {
            self.apple.reroll(self.field.scalar);
            self.snake.body.push_back((self.apple.x_pos, self.apple.y_pos));
            self.add_score();
            self.add_speed();
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
                self.reset_scores();
                self.reset_speed();
            }
        }
    }

    fn add_score(&mut self) {
        self.scores += 1;
        println!("Очков: {}", self.scores);
    }

    fn add_speed(&mut self) {
        self.speed += 1;
        self.events.set_ups(self.speed)
    }

    fn reset_scores(&mut self) {
        self.scores = 0;
        println!("Азаза проиграл. Очки сброшены!");
    }

    fn reset_speed(&mut self) {
        self.speed = 0;
        self.events.set_ups(self.speed)
    }
}