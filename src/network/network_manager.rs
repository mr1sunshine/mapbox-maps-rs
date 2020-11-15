use eyre::Result;
use mvt::{decode, FeatureWithCoordinates, Tile};
use reqwest::{Client, ClientBuilder};

const MAPBOX_API_ENDPOINT: &str = "https://api.mapbox.com";

#[derive(Debug)]
pub(crate) struct NetworkManager {
    token: String,
    client: Client,
}

impl NetworkManager {
    pub fn new(token: &str) -> Result<Self> {
        let client = ClientBuilder::new().gzip(true).build()?;
        Ok(Self {
            token: token.to_owned(),
            client,
        })
    }

    pub async fn load_style(&self, uri: &str) -> Result<String> {
        let url = format!(
            "{}/styles/v1/{}?access_token={}",
            MAPBOX_API_ENDPOINT, uri, self.token
        );
        let res = self.client.get(&url).send().await?;
        let body = res.text().await?;
        Ok(body)
    }

    pub async fn load_tilejson(&self, uri: &str) -> Result<String> {
        let url = format!(
            "{}/v4/{}.json?access_token={}",
            MAPBOX_API_ENDPOINT,
            uri.to_string().split_off("mapbox://".len()),
            self.token
        );
        let res = self.client.get(&url).send().await?;
        let body = res.text().await?;
        Ok(body)
    }

    pub async fn load_vector_tile(&self, uri: &str) -> Result<(mvt::Tile<FeatureWithCoordinates>)> {
        let url = uri
            .to_string()
            .replace("http", "https")
            .replace("a.tiles", "api")
            .replace("b.tiles", "api");
        let res = self.client.get(&url).send().await?;
        let body = res.bytes().await?;
        let tile: Tile<FeatureWithCoordinates> = decode(&body)?;
        Ok((tile))
    }
}
