use reqwest::Client;

use crate::client::ClientErrors;
use crate::Source;

pub struct YabuClient {
    client: Client,
    base_url: String,
}

impl YabuClient {
    pub fn new(source: Source) -> Result<Self, ClientErrors> {
        let client = Client::builder().build()?;
        Ok(Self {
            client,
            base_url: source.base_url.clone(),
        })
    }

    pub async fn get_manga_list(&self) -> Result<String, ClientErrors> {
        let url = self.base_url.clone();
        let body = self.client.get(url).send().await?.text().await?;
        Ok(body)
    }
}
