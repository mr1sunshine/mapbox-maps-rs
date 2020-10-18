mod config;

pub use config::Config;

pub struct Map {
    config: Config
}

impl Map {
    pub fn new(config: Config) -> Self {
        Self {
            config
        }
    }
}