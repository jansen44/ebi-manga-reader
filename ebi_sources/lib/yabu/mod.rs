use crate::manga::Manga;
use crate::Result;
use crate::source::{Source, SourceData, SourceInfo};

pub mod manga;

const YABU_SOURCE_IDENTIFIER: &str = "yabu";
const YABU_BASE_URL: &str = "https://mangayabu.top";

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
            identifier: String::from(YABU_SOURCE_IDENTIFIER),
            base_url: String::from(YABU_BASE_URL),
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
    async fn manga_list(&self) -> Result<Vec<Box<dyn Manga>>> {
        todo!()
    }

    async fn latest_manga(&self) -> Result<Vec<Box<dyn Manga>>> {
        todo!()
    }

    async fn popular_manga(&self) -> Result<Vec<Box<dyn Manga>>> {
        todo!()
    }

    async fn hot_manga(&self) -> Result<Vec<Box<dyn Manga>>> {
        todo!()
    }

    async fn search_manga(&self, _manga_title: &str) -> Result<Vec<Box<dyn Manga>>> {
        todo!()
    }

    async fn get_manga(&self, _manga_identifier: &str) -> Result<Option<Box<dyn Manga>>> {
        todo!()
    }
}

impl Source for YabuSource {}
