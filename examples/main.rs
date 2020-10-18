use mapbox_maps::{Map, Config};
use std::env;

fn main() {
    let token = env::var("MAPBOX_ACCESS_TOKEN").expect("Provide MAPBOX_ACCESS_TOKEN as env variable.");
    let map = Map::new(Config::new(&token));
}