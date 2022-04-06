use crate::chapter::{Chapter, ChapterData, ChapterInfo};
use crate::Result;

use super::client;
use super::{OPEX_BASE_URL, OPEX_SOURCE_IDENTIFIER};

mod chapter_parser {
    use scraper::{Html, Selector};

    use crate::errors::parser::{ParserError, ParserResult};
    use crate::opex::OPEX_BASE_URL;

    pub fn chapter_page_list(chapter_page_body: &str) -> ParserResult<Vec<String>> {
        let page = Html::parse_document(chapter_page_body);

        let script_selector = Selector::parse("#leitor-opex > strong > script").unwrap();
        let script_elem = match page.select(&script_selector).next() {
            Some(el) => el.inner_html(),
            None => {
                return Err(ParserError::MissingElement(String::from(
                    "#leitor-opex > strong > script",
                )))
            }
        };

        let page_list_json = match script_elem.split("paginasLista = ").nth(1) {
            Some(content) => content,
            None => {
                return Err(ParserError::Other(String::from(
                    "\"paginasLista = \" identifier not found",
                )))
            }
        };

        let page_list_json = match page_list_json.split(";").next() {
            Some(content) => content,
            None => {
                return Err(ParserError::Other(String::from(
                    "\";\" identifier not found on split",
                )))
            }
        };

        let page_list_json = serde_json::from_str::<String>(page_list_json)?;
        let page_list_json: serde_json::Value = serde_json::from_str(page_list_json.as_str())?;
        let page_list_json = match page_list_json.as_object() {
            Some(page_list) => page_list,
            None => {
                return Err(ParserError::Other(String::from(
                    "could not get object from serde_json::Value",
                )));
            }
        };

        let mut page_list = page_list_json
            .iter()
            .map(|(key, value)| Ok((key.parse::<usize>()?, value.as_str().unwrap().to_owned())))
            .collect::<ParserResult<Vec<(usize, String)>>>()?;
        page_list.sort_by(|a, b| a.0.cmp(&b.0));

        Ok(page_list
            .iter()
            .map(|(_, value)| format!("{}/{}", OPEX_BASE_URL, value))
            .collect())
    }
}

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
        format!("{}/{}", OPEX_BASE_URL, self.url)
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
        let page = client::opex_html_page(self.url.as_str()).await?;
        let pages = chapter_parser::chapter_page_list(page.as_str())?;
        Ok(pages)
    }
}

impl Chapter for OpexChapter {}
