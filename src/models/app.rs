use opengl_graphics::{OpenGL, GlGraphics, GlyphCache, TextureSettings};
use piston::{window::Window, input::*, event_loop::{Events, EventSettings, EventLoop}};
use glutin_window::GlutinWindow;
use fps_counter::FPSCounter;

use models::{state::{GameState, GameCmd}, scene::Scene, menu::Menu};

const BLACK: [f32; 4] = [0., 0., 0., 1.];
const WHITE: [f32; 4] = [1., 1., 1., 1.];

const FPS_FONT_SIZE: u32 = 20;

pub struct App {
    pub window: GlutinWindow,
    pub gl: GlGraphics,
    pub game_state: GameState,
    pub menu: Menu,
    pub scene: Scene,
    pub scores: u32,
    pub speed: u64,
    pub events: Events,
    pub fps: FPSCounter,
}

impl App {
    pub fn new(window: GlutinWindow, opengl: OpenGL, height: i32) -> Self {
        App {
            window,
            gl: GlGraphics::new(opengl),
            scene: Scene::new(height),
            menu: Menu::new(),
            scores: 0,
            speed: 6,
            events: Events::new(EventSettings::new()).ups(6).swap_buffers(true),
            game_state: GameState::Menu,
            fps: FPSCounter::new(),
        }
    }

    pub fn run(&mut self) {
        while let Some(e) = self.events.next(&mut self.window) {
            self.event(e)
        }
    }

    pub fn event(&mut self, event: Event) {
        if self.game_state == GameState::Exit {
            self.window.set_should_close(true);
        }

        if let Some(r) = event.render_args() {
            self.render(&r);
        }

        if let Some(k) = event.press_args() {
            self.key_event(&k);
        }

        if let Some(u) = event.update_args() {
            self.update(&u);
        }
    }

    pub fn key_event(&mut self, args: &Button) {
        let fps = self.fps.tick();
        match self.game_state {
            GameState::Menu => {
                let new_state = self.menu.key_event(args);
                self.set_state(&new_state);
            }
            GameState::Play => {
                let new_state = self.scene.key_event(args, fps);
                self.set_state(&new_state);
            }
            GameState::Exit => {}
            GameState::None => {}
        }
    }

    pub fn set_state(&mut self, new_state: &GameState) {
        match new_state {
            GameState::Play => { self.game_state = GameState::Play; }
            GameState::Menu => { self.game_state = GameState::Menu; }
            GameState::Exit => { self.game_state = GameState::Exit; }
            GameState::None => {}
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let width = self.window.size().width;
        let height = self.window.size().height;

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let ref font = assets.join("FiraSans-Regular.ttf");
        let mut glyph_cache = GlyphCache::new(font, (), TextureSettings::new()).unwrap();

        let fps = self.fps.tick();
        self.gl.draw(args.viewport(), |_c, gl| {
            clear(BLACK, gl);
        });

        match self.game_state {
            GameState::Menu => {
                self.menu.render(args, &mut self.gl, (width as f64, height as f64));
            }
            GameState::Play => {
                self.scene.render(args, &mut self.gl, (width as f64, height as f64));
            }
            GameState::Exit => {}
            GameState::None => {}
        }

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(2., height as f64 - 10.);
            text::Text::new_color(WHITE, FPS_FONT_SIZE).draw(
                ("FPS: ".to_string() + &fps.to_string()).as_str(),
                &mut glyph_cache,
                &c.draw_state,
                transform,
                gl,
            ).unwrap();
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let fps = self.fps.tick();
        let width = self.window.size().width;
        let height = self.window.size().height;
        match self.game_state {
            GameState::Menu => {}
            GameState::Play => {
                let result = self.scene.update((width as f64, height as f64), fps);
                match result {
                    GameCmd::Default => {}
                    GameCmd::Ok => {
                        self.add_score();
                        self.add_speed();
                    }
                    GameCmd::Reset => {
                        self.reset_scores();
                        self.reset_speed();
                    }
                }
            }
            GameState::Exit => {}
            GameState::None => {}
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
        self.speed = 6;
        self.events.set_ups(self.speed)
    }
}