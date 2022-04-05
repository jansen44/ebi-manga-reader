use crate::Chapter;
use crate::Result;
use crate::{Manga, MangaData, MangaInfo};

use super::OPEX_SOURCE_IDENTIFIER;

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
        todo!()
    }
    async fn get_chapter(&self, _chapter: usize) -> Result<Option<Box<dyn Chapter>>> {
        todo!()
    }
}

impl Manga for OpexManga {}
