use crate::client::ClientErrors;

use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::Client;

const BASE_URL: &str = "https://onepieceex.net";
const ACCEPT_HEADER: &str = "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9";
const REFERER_HEADER: &str = "https://onepieceex.net/";
const ACCEPT_LANGUAGE_HEADER: &str = "pt-BR,pt;q=0.9,en-US;q=0.8,en;q=0.7,es;q=0.6,gl;q=0.5";

pub struct OpexClient {
    client: Client,
}

impl OpexClient {
    pub fn new() -> Result<Self, ClientErrors> {
        let headers = Self::build_default_headers()?;
        let client = Client::builder().default_headers(headers).build()?;
        Ok(Self { client })
    }

    fn build_default_headers() -> Result<HeaderMap, ClientErrors> {
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT, ACCEPT_HEADER.parse()?);
        headers.insert(header::REFERER, REFERER_HEADER.parse()?);
        headers.insert(header::ACCEPT_LANGUAGE, ACCEPT_LANGUAGE_HEADER.parse()?);
        Ok(headers)
    }

    // ToDo: Receive manga as param and request to right URL
    pub async fn get_manga_page(&self) -> Result<String, ClientErrors> {
        let body = self.client.get(BASE_URL).send().await?.text().await?;
        Ok(body)
    }

    // ToDo: Receive chapter as param and request to right URL
    pub async fn get_manga_chapter(&self) -> Result<String, ClientErrors> {
        let body = self.client.get(BASE_URL).send().await?.text().await?;
        Ok(body)
    }
}
