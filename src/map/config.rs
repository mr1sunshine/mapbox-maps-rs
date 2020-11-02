use std::rc::Rc;
use winit::window::Window;

pub struct Config {
    token: String,
    window: Rc<Window>,
    min_zoom: f32,
    max_zoom: f32,
    min_pitch: f32,
    max_pitch: f32,
    render_world_copies: bool,
}

impl<'a> Config {
    pub fn new(
        token: &str,
        window: Rc<Window>,
        min_zoom: f32,
        max_zoom: f32,
        min_pitch: f32,
        max_pitch: f32,
        render_world_copies: bool,
    ) -> Self {
        Self {
            token: token.to_owned(),
            window,
            min_zoom,
            max_zoom,
            min_pitch,
            max_pitch,
            render_world_copies,
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn window(&self) -> &Window {
        self.window.as_ref()
    }

    pub fn min_zoom(&self) -> f32 {
        self.min_zoom
    }

    pub fn max_zoom(&self) -> f32 {
        self.max_zoom
    }

    pub fn min_pitch(&self) -> f32 {
        self.min_pitch
    }

    pub fn max_pitch(&self) -> f32 {
        self.max_pitch
    }

    pub fn render_world_copies(&self) -> bool {
        self.render_world_copies
    }
}
