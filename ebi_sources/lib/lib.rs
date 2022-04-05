pub mod errors;

pub mod opex;
pub mod yabu;

use errors::Result;

pub trait ChapterInfo {
    fn chapter(&self) -> usize;
    fn title(&self) -> String;
    fn url(&self) -> String;
    fn manga_identifier(&self) -> String;
    fn source_identifier(&self) -> String;
}

#[async_trait::async_trait]
pub trait ChapterData {
    async fn page_list(&self) -> Result<Vec<String>>;
}

pub trait Chapter: ChapterInfo + ChapterData + std::fmt::Debug {}

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

pub trait SourceInfo {
    fn identifier(&self) -> String;
    fn title(&self) -> String;
    fn description(&self) -> String;
    fn base_url(&self) -> String;
}

#[async_trait::async_trait]
pub trait SourceData {
    async fn manga_list(&self) -> Result<Vec<Box<dyn Manga>>>;
    async fn latest_manga(&self) -> Result<Vec<Box<dyn Manga>>>;
    async fn popular_manga(&self) -> Result<Vec<Box<dyn Manga>>>;
    async fn hot_manga(&self) -> Result<Vec<Box<dyn Manga>>>;

    async fn search_manga(&self, manga_title: &str) -> Result<Vec<Box<dyn Manga>>>;
    async fn get_manga(&self, manga_identifier: &str) -> Result<Option<Box<dyn Manga>>>;
}

pub trait Source: SourceInfo + SourceData + std::fmt::Debug {}
