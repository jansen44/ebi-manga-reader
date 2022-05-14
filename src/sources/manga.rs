use anyhow::Result;

use super::chapter::Chapter;

pub trait MangaInfo {
    fn identifier(&self) -> String;
    fn title(&self) -> String;
    fn cover(&self) -> String;
    fn url(&self) -> String;
    fn genre(&self) -> Option<String>;
    fn description(&self) -> Option<String>;
    fn source_identifier(&self) -> String;
}

#[async_trait::async_trait]
pub trait MangaData {
    async fn chapter_list(&self) -> Result<Vec<Chapter>>;
    async fn chapter(&self, chapter: usize) -> Result<Option<Chapter>>;
}

pub trait TManga: MangaInfo + MangaData + std::fmt::Debug {}

pub struct Manga {
    internal: Box<dyn TManga>,
}

impl Manga {
    pub fn identifier(&self) -> String {
        self.internal.identifier()
    }

    pub fn title(&self) -> String {
        self.internal.title()
    }

    pub fn cover(&self) -> String {
        self.internal.cover()
    }

    pub fn url(&self) -> String {
        self.internal.url()
    }

    pub fn genre(&self) -> Option<String> {
        self.internal.genre()
    }

    pub fn description(&self) -> Option<String> {
        self.internal.description()
    }

    pub fn source_identifier(&self) -> String {
        self.internal.source_identifier()
    }

    pub async fn chapter_list(&self) -> Result<Vec<Chapter>> {
        self.internal.chapter_list().await
    }

    pub async fn chapter(&self, chapter: usize) -> Result<Option<Chapter>> {
        self.internal.chapter(chapter).await
    }
}

impl std::fmt::Display for Manga {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.source_identifier();
        let title = self.title();
        let identifier = self.identifier();
        let url = self.url();

        write!(f, "[{source}] ({identifier}) \"{title}\" - {url}")
    }
}

impl<T> From<T> for Manga
where
    T: TManga + 'static,
{
    fn from(internal: T) -> Self {
        Self {
            internal: Box::new(internal),
        }
    }
}
