use crate::client::ClientErrors;
use crate::{Chapter, Manga, Source};

use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::Client;

const ACCEPT_HEADER: &str = "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9";
const REFERER_HEADER: &str = "https://onepieceex.net/";
const ACCEPT_LANGUAGE_HEADER: &str = "pt-BR,pt;q=0.9,en-US;q=0.8,en;q=0.7,es;q=0.6,gl;q=0.5";

pub struct OpexClient {
    client: Client,
    base_url: String,
}

impl OpexClient {
    pub fn new(source: Source) -> Result<Self, ClientErrors> {
        let headers = Self::build_default_headers()?;
        let client = Client::builder().default_headers(headers).build()?;
        Ok(Self {
            client,
            base_url: source.base_url.clone(),
        })
    }

    fn build_default_headers() -> Result<HeaderMap, ClientErrors> {
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT, ACCEPT_HEADER.parse()?);
        headers.insert(header::REFERER, REFERER_HEADER.parse()?);
        headers.insert(header::ACCEPT_LANGUAGE, ACCEPT_LANGUAGE_HEADER.parse()?);
        Ok(headers)
    }

    pub async fn get_manga_web_page(&self, manga: &Manga) -> Result<String, ClientErrors> {
        let url = format!("{}{}", self.base_url, manga.url);
        let body = self.client.get(url).send().await?.text().await?;
        Ok(body)
    }

    pub async fn get_chapter_web_page(&self, chapter: &Chapter) -> Result<String, ClientErrors> {
        let url = format!("{}{}", self.base_url, chapter.url);
        let body = self.client.get(url).send().await?.text().await?;
        Ok(body)
    }
}
