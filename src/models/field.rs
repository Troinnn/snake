use opengl_graphics::GlGraphics;
use piston::input::*;

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
        let dark_blue: [f32; 4] = [0.0, 0.0, 0.3, 1.0];
        let field: [f64; 4] = graphics::rectangle::square(0 as f64, 0 as f64, self.scalar as f64);

        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(dark_blue, field, transform, gl);
        });
    }
}