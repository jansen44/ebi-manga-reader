use crate::chapter::Chapter;
use crate::manga::{Manga, MangaData, MangaInfo};
use crate::Result;

use super::{OPEX_SOURCE_IDENTIFIER, OPEX_BASE_URL};
use super::client;

mod manga_parser {
    use scraper::{ElementRef, Html, Selector};

    use crate::chapter::Chapter;
    use crate::errors::parser::{ParserError, ParserResult};
    use crate::opex::chapter::OpexChapter;

    type ChapterInfo = (Option<usize>, String, String);

    fn chapter_list_selectors(manga_identifier: &str) -> Option<Selector> {
        let selector = match manga_identifier {
            "main" => Some("#volumes div.capitulos li.volume-capitulo"),
            "sbs" => Some("#conteudo #post > a.text-uppercase.sombra-clara.bnt-lista-horizontal"),
            "covers" => Some("#post div.volume.text-uppercase div.capitulos li.volume-capitulo"),
            _ => None,
        }?;

        let selector = Selector::parse(selector).unwrap();
        Some(selector)
    }

    fn get_child_span_value(element: ElementRef) -> ParserResult<String> {
        let selector = Selector::parse("span")?;
        match element.select(&selector).next() {
            Some(span) => Ok(span.inner_html()),
            None => Err(ParserError::MissingElement(String::from("span"))),
        }
    }

    fn href_from_anchor(element: ElementRef) -> ParserResult<String> {
        match element.value().attr("href") {
            Some(href) => Ok(href.to_owned()),
            None => Err(ParserError::MissingElement(String::from("href"))),
        }
    }

    fn chapter_and_title_from_raw_title(base_title: &str) -> ParserResult<(usize, &str)> {
        let mut id = "";
        let mut title = "";

        for (i, &item) in base_title.as_bytes().iter().enumerate() {
            if item == b'.' {
                id = &base_title[0..i];
                title = &base_title[i + 2..];
                break;
            }
        }

        let id = id.parse::<usize>()?;
        Ok((id, title))
    }

    fn main_chapter_info_from_element(element: ElementRef) -> ParserResult<ChapterInfo> {
        let base_title = get_child_span_value(element)?;

        let anchor_selector = &Selector::parse("a.online").unwrap();
        let url = match element.select(&anchor_selector).next() {
            Some(el) => href_from_anchor(el),
            None => return Err(ParserError::MissingElement(String::from("a.online"))),
        }?;

        let (id, title) = chapter_and_title_from_raw_title(base_title.as_str())?;
        Ok((Some(id), String::from(title), url))
    }

    fn sbs_chapter_info_from_element(element: ElementRef) -> ParserResult<ChapterInfo> {
        let title = get_child_span_value(element)?;
        let url = href_from_anchor(element)?;
        Ok((None, title, url))
    }

    fn covers_chapter_info_from_element(element: ElementRef) -> ParserResult<ChapterInfo> {
        let title = match element.text().next() {
            Some(t) => t,
            None => return Err(ParserError::MissingElement(String::from("element text()"))),
        };
        let title = (&title[..title.len() - 1]).to_owned();

        let anchor_selector = &Selector::parse("a.online").unwrap();
        let anchor = match element.select(&anchor_selector).next() {
            Some(el) => el,
            None => return Err(ParserError::MissingElement(String::from("a.online"))),
        };
        let url = href_from_anchor(anchor)?;

        Ok((None, title, url))
    }

    fn chapter_from_element(
        manga_identifier: &str,
        element: ElementRef,
        idx: usize,
    ) -> ParserResult<OpexChapter> {
        let info = match manga_identifier {
            "main" => Ok(main_chapter_info_from_element(element)),
            "sbs" => Ok(sbs_chapter_info_from_element(element)),
            "covers" => Ok(covers_chapter_info_from_element(element)),
            _ => Err(ParserError::InvalidParsingTarget(format!(
                "manga - {}",
                manga_identifier
            ))),
        }?;

        let (id, title, url) = info?;
        let chapter = if let Some(id) = id { id } else { idx };

        Ok(OpexChapter {
            chapter,
            title,
            url,
            manga_identifier: String::from(manga_identifier),
        })
    }

    pub fn chapter_list(
        manga_identifier: &str,
        html_page: &str,
    ) -> ParserResult<Vec<Box<dyn Chapter>>> {
        let scraper_html_page = Html::parse_document(html_page);

        let selector = chapter_list_selectors(manga_identifier);
        if selector.is_none() {
            return Ok(vec![]);
        }
        let selector = selector.unwrap();

        scraper_html_page
            .select(&selector)
            .into_iter()
            .enumerate()
            .map(
                |(i, el)| match chapter_from_element(manga_identifier, el, i) {
                    Ok(chapter) => Ok(Box::new(chapter) as Box<dyn Chapter>),
                    Err(err) => Err(err),
                },
            )
            .collect()
    }
}

#[derive(Default)]
pub struct OpexMangaBuilder {
    identifier: Option<String>,
    title: Option<String>,
    cover: Option<String>,
    url: Option<String>,
}

impl OpexMangaBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_identifier(mut self, identifier: &str) -> Self {
        self.identifier = Some(identifier.to_owned());
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn with_cover(mut self, cover: &str) -> Self {
        self.cover = Some(cover.to_owned());
        self
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }

    pub fn build(&self) -> OpexManga {
        OpexManga {
            identifier: self.identifier.clone().unwrap_or_default(),
            title: self.title.clone().unwrap_or_default(),
            cover: self.cover.clone().unwrap_or_default(),
            url: self.url.clone().unwrap_or_default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct OpexManga {
    pub identifier: String,
    pub title: String,
    pub cover: String,
    pub url: String,
}

impl MangaInfo for OpexManga {
    fn identifier(&self) -> String {
        self.identifier.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn cover(&self) -> String {
        self.cover.clone()
    }

    fn url(&self) -> String {
        format!("{}{}", OPEX_BASE_URL, self.url)
    }

    fn genre(&self) -> Option<String> {
        None
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn source_identifier(&self) -> String {
        OPEX_SOURCE_IDENTIFIER.to_owned()
    }
}

#[async_trait::async_trait]
impl MangaData for OpexManga {
    async fn chapter_list(&self) -> Result<Vec<Box<dyn Chapter>>> {
        let page = client::opex_html_page(self.url.as_str()).await?;
        let chapters = manga_parser::chapter_list(self.identifier.as_str(), page.as_str())?;
        Ok(chapters)
    }

    async fn chapter(&self, chapter: usize) -> Result<Option<Box<dyn Chapter>>> {
        let chapters = self.chapter_list().await?;
        let chapter = chapters.into_iter().find(|c| c.chapter() == chapter);
        Ok(chapter)
    }
}

impl Manga for OpexManga {}
