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
}
