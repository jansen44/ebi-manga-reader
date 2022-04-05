pub mod errors;

pub mod manga;
pub mod source;

pub mod opex;
pub mod yabu;

pub use errors::Result;

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
