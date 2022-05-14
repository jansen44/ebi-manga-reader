use anyhow::Result;

use crate::errors::EbiError;
use crate::sources::chapter::{ChapterData, ChapterInfo, TChapter};

use super::client;

use super::{YABU_BASE_URL, YABU_SOURCE_IDENTIFIER};

#[derive(Default)]
pub struct YabuChapterBuilder {
    chapter: Option<usize>,
    title: Option<String>,
    manga_identifier: Option<String>,
    yabu_id: Option<usize>,
}

impl YabuChapterBuilder {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_chapter(mut self, chapter: usize) -> Self {
        self.chapter = Some(chapter);
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn with_yabu_id(mut self, id: usize) -> Self {
        self.yabu_id = Some(id);
        self
    }

    pub fn with_manga_identifier(mut self, manga_identifier: &str) -> Self {
        self.manga_identifier = Some(manga_identifier.to_owned());
        self
    }

    pub fn build(&self) -> YabuChapter {
        let url = format!("{}/?p={}", YABU_BASE_URL, self.yabu_id.unwrap_or_default());

        YabuChapter {
            url,
            chapter: self.chapter.unwrap_or_default(),
            title: self.title.clone().unwrap_or_default(),
            manga_identifier: self.manga_identifier.clone().unwrap_or_default(),
            yabu_id: self.yabu_id.unwrap_or_default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct YabuChapter {
    pub chapter: usize,
    pub title: String,
    pub url: String,
    pub manga_identifier: String,
    pub yabu_id: usize,
}

impl TChapter for YabuChapter {}

impl ChapterInfo for YabuChapter {
    fn chapter(&self) -> usize {
        self.chapter
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn url(&self) -> String {
        format!("{}/{}", YABU_BASE_URL, self.url)
    }

    fn manga_identifier(&self) -> String {
        self.manga_identifier.clone()
    }

    fn source_identifier(&self) -> String {
        YABU_SOURCE_IDENTIFIER.to_owned()
    }
}

#[async_trait::async_trait]
impl ChapterData for YabuChapter {
    async fn page_url_list(&self) -> Result<Vec<String>> {
        let page = client::yabu_html(self.url.as_str()).await?;
        let hatsuna = page
            .split("var hatsuna = ")
            .nth(1)
            .ok_or(EbiError::ParserError("hatsuna not set properly"))?
            .split(";")
            .nth(0)
            .ok_or(EbiError::ParserError("no comma after hatsuna"))?;

        let hatsuna: usize = hatsuna.parse()?;

        let api_url = format!(
            "{}/chapter.php?id={}&hatsuna={}&cachebuster=0",
            YABU_BASE_URL, self.yabu_id, hatsuna
        );

        let chapter_list = client::yabu_chapter_page_list(api_url.as_str()).await?;

        Ok(chapter_list)
    }
}
