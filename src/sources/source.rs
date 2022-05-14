use anyhow::Result;

use super::manga;

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
    async fn get_manga(&self, manga_identifier: &str) -> Result<Box<dyn manga::Manga>>;
}

pub trait TSource: SourceInfo + SourceData + std::fmt::Debug {}

pub struct Source {
    pub internal: Box<dyn TSource>,
}

impl Source {
    pub fn identifier(&self) -> String {
        self.internal.identifier()
    }

    pub fn title(&self) -> String {
        self.internal.title()
    }

    pub fn description(&self) -> String {
        self.internal.description()
    }

    pub fn base_url(&self) -> String {
        self.internal.base_url()
    }

    pub async fn _manga_list(&self) -> Result<Vec<Box<dyn manga::Manga>>> {
        self.internal.manga_list().await
    }

    pub async fn _latest_manga(&self) -> Result<Vec<Box<dyn manga::Manga>>> {
        self.internal.latest_manga().await
    }

    pub async fn _popular_manga(&self) -> Result<Vec<Box<dyn manga::Manga>>> {
        self.internal.popular_manga().await
    }

    pub async fn _hot_manga(&self) -> Result<Vec<Box<dyn manga::Manga>>> {
        self.internal.hot_manga().await
    }

    pub async fn _search_manga(&self, manga_title: &str) -> Result<Vec<Box<dyn manga::Manga>>> {
        self.internal.search_manga(manga_title).await
    }

    pub async fn get_manga(&self, manga_identifier: &str) -> Result<Box<dyn manga::Manga>> {
        self.internal.get_manga(manga_identifier).await
    }
}

impl<T> From<T> for Source
where
    T: TSource + 'static,
{
    fn from(internal: T) -> Self {
        Self {
            internal: Box::new(internal),
        }
    }
}
