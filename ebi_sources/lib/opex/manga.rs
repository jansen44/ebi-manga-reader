use crate::chapter::Chapter;
use crate::manga::{Manga, MangaData, MangaInfo};
use crate::Result;

use super::OPEX_SOURCE_IDENTIFIER;

mod manga_client {
    use reqwest::header;
    use reqwest::header::HeaderMap;
    use reqwest::Client;

    use crate::errors::client::ClientResult;
    use crate::opex::OPEX_BASE_URL;

    const ACCEPT_HEADER: &str = "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9";
    const REFERER_HEADER: &str = "https://onepieceex.net/";
    const ACCEPT_LANGUAGE_HEADER: &str = "pt-BR,pt;q=0.9,en-US;q=0.8,en;q=0.7,es;q=0.6,gl;q=0.5";

    pub async fn manga_html_page(manga: &super::OpexManga) -> ClientResult<String> {
        let url = format!("{}{}", OPEX_BASE_URL, manga.url.clone());

        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT, ACCEPT_HEADER.parse().unwrap());
        headers.insert(header::REFERER, REFERER_HEADER.parse().unwrap());
        headers.insert(
            header::ACCEPT_LANGUAGE,
            ACCEPT_LANGUAGE_HEADER.parse().unwrap(),
        );

        let client = Client::builder().default_headers(headers).build().unwrap();
        let body = client.get(url).send().await?.text().await?;
        Ok(body)
    }
}

#[derive(Default)]
pub struct OpexMangaBuilder {
    identifier: Option<String>,
    title: Option<String>,
    cover: Option<String>,
    url: Option<String>,
}

impl OpexMangaBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_identifier(mut self, identifier: &str) -> Self {
        self.identifier = Some(identifier.to_owned());
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn with_cover(mut self, cover: &str) -> Self {
        self.cover = Some(cover.to_owned());
        self
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }

    pub fn build(&self) -> OpexManga {
        OpexManga {
            identifier: self.identifier.clone().unwrap_or_default(),
            title: self.title.clone().unwrap_or_default(),
            cover: self.cover.clone().unwrap_or_default(),
            url: self.url.clone().unwrap_or_default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct OpexManga {
    pub identifier: String,
    pub title: String,
    pub cover: String,
    pub url: String,
}

impl MangaInfo for OpexManga {
    fn identifier(&self) -> String {
        self.identifier.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn cover(&self) -> String {
        self.cover.clone()
    }

    fn url(&self) -> String {
        self.url.clone()
    }

    fn genre(&self) -> Option<String> {
        None
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn source_identifier(&self) -> String {
        OPEX_SOURCE_IDENTIFIER.to_owned()
    }
}

#[async_trait::async_trait]
impl MangaData for OpexManga {
    async fn chapter_list(&self) -> Result<Vec<Box<dyn Chapter>>> {
        let page = manga_client::manga_html_page(self).await?;
        println!("{}", page);

        Ok(vec![])
    }
    async fn get_chapter(&self, _chapter: usize) -> Result<Option<Box<dyn Chapter>>> {
        todo!()
    }
}

impl Manga for OpexManga {}
