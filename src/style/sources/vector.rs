use crate::network::NetworkManager;
use crate::style_spec;
use eyre::Result;
use std::rc::Rc;
use tilejson::TileJson;

#[derive(Debug)]
pub(crate) struct Vector {
    nm: Rc<NetworkManager>,
    name: String,
    tilejson: Option<TileJson>,
}

impl Vector {
    pub async fn new(
        nm: Rc<NetworkManager>,
        name: &str,
        source: &style_spec::Vector,
    ) -> Result<Self> {
        let tilejson = match &source.url {
            Some(url) => {
                let tilejson = nm.load_tilejson(&url).await?;
                let tilejson = serde_json::from_str::<TileJson>(&tilejson)?;
                Some(tilejson)
            }
            None => None,
        };
        Ok(Self {
            nm,
            name: name.to_owned(),
            tilejson,
        })
    }
}
