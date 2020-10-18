pub struct Config {
    token: String
}

impl Config {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_owned()
        }
    }
}