use crate::Manga;
use crate::Result;
use crate::{Source, SourceData, SourceInfo};

#[derive(Debug)]
pub struct YabuSource {
    pub identifier: String,
    pub title: String,
    pub description: String,
    pub base_url: String,
}

impl YabuSource {
    pub fn default() -> Self {
        Self {
            base_url: String::from("https://mangayabu.top"),
            identifier: String::from("yabu"),
            title: String::from("Manga Yabu"),
            description: String::from("Manga Yabu! - Ler Mangás Online"),
        }
    }
}

impl SourceInfo for YabuSource {
    fn identifier(&self) -> String {
        self.identifier.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn base_url(&self) -> String {
        self.base_url.clone()
    }
}

#[async_trait::async_trait]
impl SourceData for YabuSource {
    async fn manga_list(&self) -> Result<Box<dyn Manga>> {
        todo!()
    }

    async fn latest_manga(&self) -> Result<Box<dyn Manga>> {
        todo!()
    }

    async fn popular_manga(&self) -> Result<Box<dyn Manga>> {
        todo!()
    }

    async fn hot_manga(&self) -> Result<Box<dyn Manga>> {
        todo!()
    }

    async fn search_manga(&self, _manga_title: &str) -> Result<Box<dyn Manga>> {
        todo!()
    }

    async fn get_manga(&self, _manga_identifier: &str) -> Result<Option<Box<dyn Manga>>> {
        todo!()
    }
}

impl Source for YabuSource {}

// use crate::errors::Result;
// use crate::MManga;
// use crate::SSource;

// pub mod source;

// mod client;
// mod parser;
