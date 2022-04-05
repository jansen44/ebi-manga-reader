pub mod errors;

pub mod manga;

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

pub trait SourceInfo {
    fn identifier(&self) -> String;
    fn title(&self) -> String;
    fn description(&self) -> String;
    fn base_url(&self) -> String;
}

#[async_trait::async_trait]
pub trait SourceData {
    async fn manga_list(&self) -> Result<Vec<Box<dyn manga::Manga>>>;
    async fn latest_manga(&self) -> Result<Vec<Box<dyn manga::Manga>>>;
    async fn popular_manga(&self) -> Result<Vec<Box<dyn manga::Manga>>>;
    async fn hot_manga(&self) -> Result<Vec<Box<dyn manga::Manga>>>;

    async fn search_manga(&self, manga_title: &str) -> Result<Vec<Box<dyn manga::Manga>>>;
    async fn get_manga(&self, manga_identifier: &str) -> Result<Option<Box<dyn manga::Manga>>>;
}

pub trait Source: SourceInfo + SourceData + std::fmt::Debug {}
