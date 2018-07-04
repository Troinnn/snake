use std::collections::LinkedList;

use piston::input::*;
use opengl_graphics::GlGraphics;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Snake {
    body: LinkedList<(i32, i32)>,
    direct: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        let mut body: LinkedList<(i32, i32)> = LinkedList::new();
        body.push_front((2, 0));
        body.push_front((1, 0));
        body.push_front((0, 0));
        Snake {
            body,
            direct: (Direction::Right),
        }
    }

    pub fn set_direct(&mut self, direct: Direction) {
        if self.direct == Direction::Right && direct != Direction::Left {
            self.direct = direct;
            return;
        }
        if self.direct == Direction::Left && direct != Direction::Right {
            self.direct = direct;
            return;
        }
        if self.direct == Direction::Up && direct != Direction::Down {
            self.direct = direct;
            return;
        }
        if self.direct == Direction::Down && direct != Direction::Up {
            self.direct = direct;
            return;
        }
    }

    pub fn step_move(&mut self) {
        let head = self.body.front().expect("Нет тела, нет дела!").clone();
        match self.direct {
            Direction::Left => {
                self.body.push_front((head.0 - 1, head.1));
            }
            Direction::Right => {
                self.body.push_front((head.0 + 1, head.1));
            }
            Direction::Up => {
                self.body.push_front((head.0, head.1 - 1));
            }
            Direction::Down => {
                self.body.push_front((head.0, head.1 + 1));
            }
        }
        self.body.pop_back();
    }

    pub fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs) {
        use graphics;
        let half_green: [f32; 4] = [0.0, 0.5, 0.0, 1.0];

        let mut squares: Vec<graphics::types::Rectangle>  = Vec::new();

        for rect in self.body.iter() {
            let mut tt: graphics::types::Rectangle = graphics::rectangle::square((rect.0 * 20) as f64, (rect.1 * 20) as f64, 20_f64);
            squares.push(tt);
        }

        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;
            for square in squares {
                graphics::rectangle(half_green, square, transform, gl);
            }
        });
    }
}

pub struct Field {
    scalar: u32,
}

impl Field {
    pub fn new(scalar: u32) -> Field {
        Field {
            scalar
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs) {
        use graphics;
        let dark_blue: [f32; 4] = [0.0, 0.0, 0.3, 1.0];
        let field: [f64; 4] = graphics::rectangle::square(0 as f64, 0 as f64, self.scalar as f64);

        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(dark_blue, field, transform, gl);
        });
    }
}

pub struct App {
    pub gl: GlGraphics,
    pub field: Field,
    pub snake: Snake,
}

impl App {
    pub fn render(&mut self, arg: &RenderArgs) {
        use graphics;
        let blue: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(blue, gl);
        });

        self.field.render(&mut self.gl, arg);
        self.snake.render(&mut self.gl, arg);
    }
}