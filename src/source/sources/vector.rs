use crate::network::NetworkManager;
use crate::source::tile::Tile;
use crate::style_spec;
use eyre::Result;
use std::rc::Rc;
use tilejson::TileJson;

#[derive(Debug)]
pub(crate) struct Vector {
    nm: Rc<NetworkManager>,
    name: String,
    tilejson: Option<TileJson>,
    options: style_spec::Vector,
}

impl Vector {
    pub fn new(nm: Rc<NetworkManager>, name: &str, options: &style_spec::Vector) -> Self {
        Self {
            nm,
            name: name.to_owned(),
            tilejson: None,
            options: options.clone(),
        }
    }

    pub async fn load(&mut self) -> Result<()> {
        self.tilejson = match &self.options.url {
            Some(url) => {
                let tilejson = self.nm.load_tilejson(&url).await?;
                let tilejson = serde_json::from_str::<TileJson>(&tilejson)?;
                Some(tilejson)
            }
            None => None,
        };
        println!("{:#?}", self.tilejson);
        Ok(())
    }

    pub async fn load_tile(&self, tile: &mut Tile) {
        let tilejson = match &self.tilejson {
            Some(t) => t,
            None => return,
        };
        let url = tile
            .tile_id()
            .canonical()
            .url(&tilejson.tiles, Some(self.options.scheme.clone()));

        println!("url for tile loading: {}", url);
    }
}
