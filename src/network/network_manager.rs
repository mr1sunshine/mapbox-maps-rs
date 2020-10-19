use reqwest::{Client, ClientBuilder};
use eyre::Result;

const MAPBOX_API_ENDPOINT: &str = "https://api.mapbox.com";

pub(crate) struct NetworkManager {
    token: String,
    client: Client
}

impl NetworkManager {
    pub fn new(token: &str) -> Result<Self> {
        let client = ClientBuilder::new().gzip(true).build()?;
        Ok(Self {
            token: token.to_owned(),
            client
        })
    }

    pub async fn load_style(&self, uri: &str) -> Result<String> {
        let url = format!("{}/styles/v1/{}?access_token={}", MAPBOX_API_ENDPOINT, uri, self.token);
        let res = self.client.get(&url).send().await?;
        let body = res.text().await?;
        Ok(body)
    }
}