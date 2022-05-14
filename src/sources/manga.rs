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
    async fn chapter_list(&self) -> Result<Vec<Box<dyn Chapter>>>;
    async fn chapter(&self, chapter: usize) -> Result<Option<Box<dyn Chapter>>>;
}

pub trait Manga: MangaInfo + MangaData + std::fmt::Debug {}

impl std::fmt::Display for dyn Manga {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.source_identifier();
        let title = self.title();
        let identifier = self.identifier();
        let url = self.url();

        write!(f, "[{source}] ({identifier}) \"{title}\" - {url}")
    }
}
