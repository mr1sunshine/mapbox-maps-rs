use mapbox_maps::{Map, Config};
use std::env;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("MAPBOX_ACCESS_TOKEN").expect("Provide MAPBOX_ACCESS_TOKEN as env variable.");
    let map = Map::new(Config::new(&token))?;

    map.load_style("mapbox/streets-v11").await?;

    Ok(())
}