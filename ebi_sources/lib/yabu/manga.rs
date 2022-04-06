use crate::chapter::Chapter;
use crate::manga::{Manga, MangaData, MangaInfo};
use crate::Result;

use super::{YABU_SOURCE_IDENTIFIER, YABU_BASE_URL};

#[derive(Default)]
pub struct YabuMangaBuilder {
    identifier: Option<String>,
    title: Option<String>,
    cover: Option<String>,
    url: Option<String>,
}

impl YabuMangaBuilder {
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

    pub fn build(&self) -> YabuManga {
        YabuManga {
            identifier: self.identifier.clone().unwrap_or_default(),
            title: self.title.clone().unwrap_or_default(),
            cover: self.cover.clone().unwrap_or_default(),
            url: self.url.clone().unwrap_or_default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct YabuManga {
    pub identifier: String,
    pub title: String,
    pub cover: String,
    pub url: String,
}

impl MangaInfo for YabuManga {
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
        format!("{}{}", YABU_BASE_URL, self.url)
    }

    fn genre(&self) -> Option<String> {
        None
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn source_identifier(&self) -> String {
        YABU_SOURCE_IDENTIFIER.to_owned()
    }
}

#[async_trait::async_trait]
impl MangaData for YabuManga {
    async fn chapter_list(&self) -> Result<Vec<Box<dyn Chapter>>> {
        todo!()
    }

    async fn chapter(&self, _chapter: usize) -> Result<Option<Box<dyn Chapter>>> {
        todo!()
    }
}

impl Manga for YabuManga {}
