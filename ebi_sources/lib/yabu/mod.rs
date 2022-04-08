use crate::manga::Manga;
use crate::source::{Source, SourceData, SourceInfo};
use crate::Result;

mod client;

pub mod manga;

const YABU_SOURCE_IDENTIFIER: &str = "yabu";
const YABU_BASE_URL: &str = "https://mangayabu.top";

mod source_parser {
    use scraper::{ElementRef, Html, Selector};

    use crate::errors::parser::{ParserError, ParserResult};
    use crate::manga::Manga;

    use super::manga::{YabuManga, YabuMangaBuilder};

    const LATEST_MANGA_CAROUSEL_POSITION: usize = 0;
    const POPULAR_MANGA_CAROUSEL_POSITION: usize = 1;

    fn manga_card_cover(el: ElementRef) -> ParserResult<String> {
        let cover_selector = Selector::parse(".image img").unwrap();
        let img = match el.select(&cover_selector).next() {
            Some(img) => img,
            None => {
                return Err(ParserError::MissingElement(String::from(
                    "could not find manga cover elemtn",
                )))
            }
        };

        match img.value().attr("src") {
            Some(image_url) => Ok(image_url.to_owned()),
            None => match img.value().attr("data-cfsrc") {
                Some(image_url) => Ok(image_url.to_owned()),
                None => Err(ParserError::MissingElement(String::from(
                    "could not find manga cover",
                ))),
            },
        }
    }

    fn manga_card_title(el: ElementRef) -> ParserResult<String> {
        let link_selector = Selector::parse(".info-bottom a").unwrap();
        match el.select(&link_selector).next() {
            Some(anchor_el) => Ok(anchor_el.inner_html()),
            None => {
                return Err(ParserError::MissingElement(String::from(
                    "could not find manga card 'a' element",
                )))
            }
        }
    }

    fn manga_card_identifier(el: ElementRef) -> ParserResult<String> {
        let link_selector = Selector::parse(".info-bottom a").unwrap();
        let href = match el.select(&link_selector).next() {
            Some(href) => href.value().attr("href"),
            None => {
                return Err(ParserError::MissingElement(String::from(
                    "could not find manga card 'a' element",
                )))
            }
        };

        match href {
            Some(href) => {
                let mut href = href.split('/').collect::<Vec<&str>>();
                href.pop();
                let identifier = *href.iter().last().unwrap();
                Ok(identifier.to_owned())
            }
            None => Err(ParserError::MissingElement(String::from(
                "could not find manga card 'a' element href attr",
            ))),
        }
    }

    fn manga_from_card(el: ElementRef) -> ParserResult<YabuManga> {
        let cover_url = manga_card_cover(el)?;
        let identifier = manga_card_identifier(el)?;
        let title = manga_card_title(el)?;

        let manga = YabuMangaBuilder::new()
            .with_identifier(identifier.as_str())
            .with_cover(cover_url.as_str())
            .with_title(title.as_str())
            .build();

        Ok(manga)
    }

    fn manga_list_from_carousel(element: ElementRef) -> ParserResult<Vec<Box<dyn Manga>>> {
        let manga_card_selector = Selector::parse(".manga-card").unwrap();
        let manga_card_iter = element.select(&manga_card_selector);

        let manga_list = manga_card_iter
            .map(|card| match manga_from_card(card) {
                Ok(manga) => Ok(Box::new(manga) as Box<dyn Manga>),
                Err(err) => Err(err),
            })
            .collect::<ParserResult<Vec<Box<dyn Manga>>>>();

        manga_list
    }

    fn manga_list_from_carousel_at(
        position: usize,
        html: Html,
    ) -> ParserResult<Vec<Box<dyn Manga>>> {
        let selector = Selector::parse("#main .carousel").unwrap();

        let carousel = html.select(&selector).nth(position);

        match carousel {
            Some(carousel) => manga_list_from_carousel(carousel),
            None => Err(ParserError::MissingElement(String::from(
                "could not find \"latest_manga\" carousel",
            ))),
        }
    }

    pub fn latest_manga_list(yabu_homepage_html: &str) -> ParserResult<Vec<Box<dyn Manga>>> {
        let html = Html::parse_document(yabu_homepage_html);
        manga_list_from_carousel_at(LATEST_MANGA_CAROUSEL_POSITION, html)
    }

    pub fn popular_manga_list(yabu_homepage_html: &str) -> ParserResult<Vec<Box<dyn Manga>>> {
        let html = Html::parse_document(yabu_homepage_html);
        manga_list_from_carousel_at(POPULAR_MANGA_CAROUSEL_POSITION, html)
    }
}

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
        let manga_list = client::yabu_manga_list().await?;
        Ok(manga_list)
    }

    async fn latest_manga(&self) -> Result<Vec<Box<dyn Manga>>> {
        let html_page = client::yabu_homepage_html().await?;
        let manga_list = source_parser::latest_manga_list(html_page.as_str())?;
        Ok(manga_list)
    }

    async fn popular_manga(&self) -> Result<Vec<Box<dyn Manga>>> {
        let html_page = client::yabu_homepage_html().await?;
        let manga_list = source_parser::popular_manga_list(html_page.as_str())?;
        Ok(manga_list)
    }

    async fn hot_manga(&self) -> Result<Vec<Box<dyn Manga>>> {
        self.popular_manga().await
    }

    async fn search_manga(&self, manga_title: &str) -> Result<Vec<Box<dyn Manga>>> {
        let full_list = self.manga_list().await?;
        let reg = regex::Regex::new(manga_title.to_uppercase().as_str()).unwrap();

        Ok(full_list
            .into_iter()
            .filter(|manga| reg.is_match(manga.title().to_uppercase().as_str()))
            .collect())
    }

    async fn get_manga(&self, manga_identifier: &str) -> Result<Option<Box<dyn Manga>>> {
        let full_list = self.manga_list().await?;
        let manga = full_list.into_iter().find(|manga| manga.identifier() == manga_identifier);
        Ok(manga)
    }
}

impl Source for YabuSource {}
