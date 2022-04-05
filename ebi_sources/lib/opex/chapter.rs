use crate::chapter::{Chapter, ChapterData, ChapterInfo};
use crate::Result;

use super::OPEX_SOURCE_IDENTIFIER;

#[derive(Default)]
pub struct OpexChapterBuilder {
    chapter: Option<usize>,
    title: Option<String>,
    url: Option<String>,
    manga_identifier: Option<String>,
}

impl OpexChapterBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_chapter(mut self, chapter: usize) -> Self {
        self.chapter = Some(chapter);
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }

    pub fn with_manga_identifier(mut self, manga_identifier: &str) -> Self {
        self.manga_identifier = Some(manga_identifier.to_owned());
        self
    }

    pub fn build(&self) -> OpexChapter {
        OpexChapter {
            chapter: self.chapter.unwrap_or_default(),
            title: self.title.clone().unwrap_or_default(),
            url: self.url.clone().unwrap_or_default(),
            manga_identifier: self.manga_identifier.clone().unwrap_or_default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct OpexChapter {
    pub chapter: usize,
    pub title: String,
    pub url: String,
    pub manga_identifier: String,
}

impl ChapterInfo for OpexChapter {
    fn chapter(&self) -> usize {
        self.chapter
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn url(&self) -> String {
        self.url.clone()
    }

    fn manga_identifier(&self) -> String {
        self.manga_identifier.clone()
    }

    fn source_identifier(&self) -> String {
        OPEX_SOURCE_IDENTIFIER.to_owned()
    }
}

#[async_trait::async_trait]
impl ChapterData for OpexChapter {
    async fn page_url_list(&self) -> Result<Vec<String>> {
        todo!()
    }
}

impl Chapter for OpexChapter {}
