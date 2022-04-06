use crate::Result;

pub trait ChapterInfo {
    fn chapter(&self) -> usize;
    fn title(&self) -> String;
    fn url(&self) -> String;
    fn manga_identifier(&self) -> String;
    fn source_identifier(&self) -> String;
}

#[async_trait::async_trait]
pub trait ChapterData {
    async fn page_url_list(&self) -> Result<Vec<String>>;
}

pub trait Chapter: ChapterInfo + ChapterData + std::fmt::Debug {}

impl std::fmt::Display for dyn Chapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chapter = self.chapter();
        let title = self.title();
        let url = self.url();
        let manga_identifier = self.manga_identifier();
        let source = self.source_identifier();

        write!(f, "[{source}::{manga_identifier}] {chapter} - \"{title}\" - {url}")?;
        Ok(())
    }
}
