use reqwest::Client;

use crate::errors::client::ClientResult;
use crate::Source;

pub struct YabuClient {
    client: Client,
    base_url: String,
}

impl YabuClient {
    pub fn new(source: Source) -> ClientResult<Self> {
        let client = Client::builder().build().unwrap();

        Ok(Self {
            client,
            base_url: source.base_url.clone(),
        })
    }

    pub async fn get_manga_list(&self) -> ClientResult<String> {
        let url = self.base_url.clone();
        let body = self.client.get(url).send().await?.text().await?;
        Ok(body)
    }
}
