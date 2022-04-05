use crate::Result;
use crate::chapter::Chapter;

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
    async fn get_chapter(&self, chapter: usize) -> Result<Option<Box<dyn Chapter>>>;
}

pub trait Manga: MangaInfo + MangaData + std::fmt::Debug {}

impl std::fmt::Display for dyn Manga {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.source_identifier();
        let title = self.title();
        let identifier = self.identifier();
        let url = self.url();
        let cover = self.cover();
        let genre = self.genre();
        let description = self.description();

        write!(f, "[{source}] ({identifier}) \"{title}\" - {url} - {cover}")?;
        if genre.is_some() {
            write!(f, " - {}", genre.unwrap())?;
        }
        if description.is_some() {
            write!(f, " - {}", description.unwrap())?;
        }
        Ok(())
    }
}
