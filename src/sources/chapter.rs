use anyhow::Result;

#[async_trait::async_trait]
pub trait TChapter: std::marker::Send + std::marker::Sync {
    fn chapter(&self) -> usize;
    fn title(&self) -> String;
    fn url(&self) -> String;
    fn manga_identifier(&self) -> String;
    fn source_identifier(&self) -> String;
    async fn page_url_list(&self) -> Result<Vec<String>>;
}

pub struct Chapter {
    internal: Box<dyn TChapter>,
}

impl Chapter {
    pub fn chapter(&self) -> usize {
        self.internal.chapter()
    }

    pub fn title(&self) -> String {
        self.internal.title()
    }

    pub fn url(&self) -> String {
        self.internal.url()
    }

    pub fn manga_identifier(&self) -> String {
        self.internal.manga_identifier()
    }

    pub fn source_identifier(&self) -> String {
        self.internal.source_identifier()
    }

    pub async fn page_url_list(&self) -> Result<Vec<String>> {
        self.internal.page_url_list().await
    }
}

impl std::fmt::Display for Chapter {
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

impl<T> From<T> for Chapter
where
    T: TChapter + 'static,
{
    fn from(internal: T) -> Self {
        Self {
            internal: Box::new(internal),
        }
    }
}
