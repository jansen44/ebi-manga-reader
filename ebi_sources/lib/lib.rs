pub mod errors;

pub mod opex;
pub mod yabu;

use errors::Result;

#[derive(Clone, Debug)]
pub struct SSource {
    pub identifier: String,
    pub title: String,
    pub description: String,
    pub base_url: String,
}

#[derive(Clone, Debug)]
pub struct MManga {
    pub identifier: String,
    pub title: String,
    pub thumbnail: String,
    pub url: String,
    pub source_identifier: String,
    pub genre: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CChapter {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub manga_identifier: String,
    pub source_identifier: String,
}

pub trait ChapterInfo {
    fn chapter(&self) -> usize;
    fn title(&self) -> String;
    fn url(&self) -> String;
    fn manga_identifier(&self) -> String;
    fn source_identifier(&self) -> String;
}

#[async_trait::async_trait]
pub trait ChapterData<'t> {
    async fn page_list(&self) -> Result<'t, Vec<String>>;
}

pub trait Chapter<'t>: ChapterInfo + ChapterData<'t> + std::fmt::Display + std::fmt::Debug {}
pub type BoxedChapterList<'t> = Vec<Box<dyn Chapter<'t>>>;
pub type BoxedOptionalChapter<'t> = Option<Box<dyn Chapter<'t>>>;

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
pub trait MangaData<'t> {
    async fn chapter_list(&self) -> Result<'t, BoxedChapterList>;
    async fn get_chapter(&self, chapter: usize) -> Result<'t, BoxedOptionalChapter<'t>>;
}

pub trait Manga<'t>: MangaInfo + MangaData<'t> + std::fmt::Display + std::fmt::Debug {}
pub type BoxedMangaList<'t> = Vec<Box<dyn Manga<'t>>>;
pub type BoxedOptionalManga<'t> = Option<Box<dyn Manga<'t>>>;

pub trait SourceInfo {
    fn identifier(&self) -> String;
    fn title(&self) -> String;
    fn description(&self) -> String;
    fn base_url(&self) -> String;
}

#[async_trait::async_trait]
pub trait SourceData<'t> {
    async fn manga_list(&self) -> Result<'t, BoxedMangaList<'t>>;
    async fn latest_manga(&self) -> Result<'t, BoxedMangaList<'t>>;
    async fn popular_manga(&self) -> Result<'t, BoxedMangaList<'t>>;
    async fn hot_manga(&self) -> Result<'t, BoxedMangaList<'t>>;

    async fn search_manga(&self, manga_title: &str) -> Result<'t, BoxedMangaList<'t>>;
    async fn get_manga(&self, manga_identifier: &str) -> Result<'t, BoxedOptionalManga<'t>>;
}

pub trait Source<'t>: SourceInfo + SourceData<'t> + std::fmt::Display + std::fmt::Debug {}
