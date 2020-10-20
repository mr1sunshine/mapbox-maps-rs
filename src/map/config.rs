use std::rc::Rc;
use winit::window::Window;

pub struct Config {
    token: String,
    window: Rc<Window>,
}

impl<'a> Config {
    pub fn new(token: &str, window: Rc<Window>) -> Self {
        Self {
            token: token.to_owned(),
            window,
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn window(&self) -> &Window {
        self.window.as_ref()
    }
}
