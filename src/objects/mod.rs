use std::collections::LinkedList;

use piston::input::*;
use opengl_graphics::GlGraphics;
use std::collections::vec_deque::VecDeque;
use rand;
use rand::Rng;

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

    pub fn step_move(&mut self, scalar: i32) {

        let head = self.body.front().expect("Нет тела, нет дела!").clone();
        match self.direct {
            Direction::Left => {
                if (head.0 - 1) >= 0 {
                    self.body.push_front((head.0 - 1, head.1));
                } else {
                    self.body.push_front(((scalar / 20) - 1, head.1));
                }
            }
            Direction::Right => {
                if (head.0 + 1) <= (scalar / 20) - 1 {
                    self.body.push_front((head.0 + 1, head.1));
                } else {
                    self.body.push_front((0, head.1));
                }
            }
            Direction::Up => {
                if (head.1 - 1) >= 0 {
                    self.body.push_front((head.0, head.1 - 1));
                } else {
                    self.body.push_front((head.0, (scalar / 20) - 1));
                }
            }
            Direction::Down => {
                if (head.1 + 1) <= (scalar / 20) - 1 {
                    self.body.push_front((head.0, head.1 + 1));
                } else {
                    self.body.push_front((head.0, 0));
                }
            }
        }
        self.body.pop_back();
    }

    pub fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs) {
        use graphics;
        let half_green: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let mut squares: VecDeque<graphics::types::Rectangle> = VecDeque::new();

        for rect in self.body.iter() {
            let mut tt: graphics::types::Rectangle = graphics::rectangle::square((rect.0 * 20) as f64, (rect.1 * 20) as f64, 20_f64);
            squares.push_back(tt);
        }

        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;
            let head = squares.pop_front().expect("Змейка пустая");
            graphics::rectangle(green, head, transform, gl);
            for square in squares {
                graphics::rectangle(half_green, square, transform, gl);
            }
        });
    }
}

pub struct Field {
    pub scalar: i32,
}

impl Field {
    pub fn new(scalar: i32) -> Field {
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
    pub apple: Apple,
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
        self.apple.render(&mut self.gl, arg);
    }

    pub fn check_coll(&mut self) {
        if self.apple.x_pos == self.snake.body.front().expect("Косяк").clone().0 && self.apple.y_pos == self.snake.body.front().expect("Косяк").clone().1 {
            self.apple.reroll(self.field.scalar);
            self.snake.body.push_back((self.apple.x_pos, self.apple.y_pos));
        }
    }
}

pub struct Apple {
    x_pos: i32,
    y_pos: i32,
}

impl Apple {
    pub fn new(scalar: i32) -> Apple {
        let mut rng = rand::thread_rng();
        Apple {
            x_pos: rng.gen_range::<i32>(0, (scalar / 20) - 1),
            y_pos: rng.gen_range::<i32>(0, (scalar / 20) - 1),
        }
    }

    pub fn reroll(&mut self, scalar: i32) {
        let mut rng = rand::thread_rng();
        self.x_pos = rng.gen_range::<i32>(0, (scalar / 20) - 1);
        self.y_pos = rng.gen_range::<i32>(0, (scalar / 20) - 1);
    }

    pub fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs) {
        use graphics;
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let field: [f64; 4] = graphics::rectangle::square((self.x_pos * 20) as f64, (self.y_pos * 20) as f64, 20_f64);

        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(red, field, transform, gl);
        });
    }
}