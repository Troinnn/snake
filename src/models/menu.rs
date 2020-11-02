use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use std::path::PathBuf;

use models::state::GameState;
use piston::input::{Key, Button, RenderArgs};
use piston::input::Button::Keyboard;

#[derive(Debug)]
pub struct Item {
    pub title: String,
}

impl Item {
    pub fn new(title: String) -> Self {
        Item {
            title
        }
    }
}

#[derive(Debug)]
pub struct Menu {
    pub pos: usize,
    pub items: Vec<Item>,
    pub font_size: f64,
    pub font: PathBuf,
}

impl Menu {
    pub fn new() -> Self {
        //let titles: Vec<&str> = vec!["Start", "Options", "Exit"];
        let titles: Vec<&str> = vec!["Start", "Exit"];
        let items = titles.iter().map(|x| {
            Item::new(x.to_string())
        }).collect::<Vec<Item>>();
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let font = assets.join("FiraSans-Regular.ttf");
        Menu {
            pos: 0,
            items,
            font_size: 50.,
            font,
        }
    }

    pub fn next(&mut self) {
        if self.pos == self.items.len() - 1 {
            self.pos = 0;
        } else {
            self.pos += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.pos == 0 {
            self.pos = self.items.len() - 1;
        } else {
            self.pos -= 1;
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, (w, h): (f64, f64)) {
        use graphics::*;

        const WHITE: [f32; 4] = [1., 1., 1., 1.];
        let mut glyph_cache = GlyphCache::new(&self.font, (), TextureSettings::new()).unwrap();
        gl.draw(args.viewport(), |c, gl| {
            for (i, item) in self.items.iter().enumerate() {
                let transform = c.transform.trans(
                    w / 2. - self.font_size * 2.,
                    h / 2. + self.font_size * i as f64,
                );

                let title;
                if self.pos == i {
                    title = "-> ".to_string() + &item.title;
                } else {
                    title = "   ".to_string() + &item.title;
                }

                text::Text::new_color(WHITE, self.font_size as u32).draw(
                    title.as_str(),
                    &mut glyph_cache,
                    &c.draw_state,
                    transform,
                    gl,
                ).unwrap();
            }
        });
    }

    pub fn key_event(&mut self, args: &Button) -> GameState {
        match args {
            Keyboard(k) => {
                match k {
                    Key::Up => {
                        self.prev();
                    }
                    Key::Down => {
                        self.next();
                    }
                    Key::Return => {
                        match self.pos {
                            0 => {
                                return GameState::Play;
                            }
                            /*1 => {
                                return GameState::None;
                            }*/
                            1 => {
                                return GameState::Exit;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        GameState::None
    }
}