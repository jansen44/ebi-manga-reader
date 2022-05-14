use anyhow::Result;

use crate::errors::EbiError;
use crate::sources::manga::Manga;
use crate::sources::source::{Source, SourceData, SourceInfo};

mod client;

pub mod chapter;
pub mod manga;

const YABU_SOURCE_IDENTIFIER: &str = "yabu";
const YABU_BASE_URL: &str = "https://mangayabu.top";

mod source_parser {
    use anyhow::Result;
    use scraper::{ElementRef, Html, Selector};

    use crate::errors::EbiError;
    use crate::sources::manga::Manga;
    use crate::sources::yabu::manga::{YabuManga, YabuMangaBuilder};

    const LATEST_MANGA_CAROUSEL_POSITION: usize = 0;
    const POPULAR_MANGA_CAROUSEL_POSITION: usize = 1;

    fn manga_card_cover(el: ElementRef) -> Result<String> {
        let cover_selector = Selector::parse(".image img").unwrap();

        let img = el
            .select(&cover_selector)
            .next()
            .ok_or(EbiError::ParserMissingElement(".image img"))?;

        if img.value().attr("src").is_some() {
            let img_url = img.value().attr("src").unwrap();
            return Ok(img_url.to_owned());
        }

        let img = img
            .value()
            .attr("data-cfsrc")
            .ok_or(EbiError::ParserMissingElement(".image img[data-cfsrc]"))?;

        Ok(img.to_owned())
    }

    fn manga_card_title(el: ElementRef) -> Result<String> {
        let link_selector = Selector::parse(".info-bottom a").unwrap();
        let anchor = el
            .select(&link_selector)
            .next()
            .ok_or(EbiError::ParserMissingElement(
                ".info-bottom a -- manga_card_title",
            ))?;
        Ok(anchor.inner_html())
    }

    fn manga_card_identifier(el: ElementRef) -> Result<String> {
        let link_selector = Selector::parse(".info-bottom a").unwrap();
        let href = el
            .select(&link_selector)
            .next()
            .ok_or(EbiError::ParserMissingElement(
                ".info-bottom a -- manga_card_identifier",
            ))?
            .value()
            .attr("href")
            .ok_or(EbiError::ParserMissingElement(
                ".info-bottom a[href] -- manga_card_identifier",
            ))?;

        let mut href = href.split('/').collect::<Vec<&str>>();
        href.pop();

        let identifier = *href.iter().last().unwrap();
        Ok(identifier.to_owned())
    }

    fn manga_from_card(el: ElementRef) -> Result<YabuManga> {
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

    fn manga_list_from_carousel(element: ElementRef) -> Result<Vec<Box<dyn Manga>>> {
        let manga_card_selector = Selector::parse(".manga-card").unwrap();
        let manga_card_iter = element.select(&manga_card_selector);

        let manga_list = manga_card_iter
            .map(|card| match manga_from_card(card) {
                Ok(manga) => Ok(Box::new(manga) as Box<dyn Manga>),
                Err(err) => Err(err),
            })
            .collect::<Result<Vec<Box<dyn Manga>>>>();

        manga_list
    }

    fn manga_list_from_carousel_at(position: usize, html: Html) -> Result<Vec<Box<dyn Manga>>> {
        let selector = Selector::parse("#main .carousel").unwrap();
        let carousel = html
            .select(&selector)
            .nth(position)
            .ok_or(EbiError::ParserMissingElement("#main .carousel"))?;
        manga_list_from_carousel(carousel)
    }

    pub fn latest_manga_list(yabu_homepage_html: &str) -> Result<Vec<Box<dyn Manga>>> {
        let html = Html::parse_document(yabu_homepage_html);
        manga_list_from_carousel_at(LATEST_MANGA_CAROUSEL_POSITION, html)
    }

    pub fn popular_manga_list(yabu_homepage_html: &str) -> Result<Vec<Box<dyn Manga>>> {
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

    async fn get_manga(&self, manga_identifier: &str) -> Result<Box<dyn Manga>> {
        let full_list = self.manga_list().await?;
        let manga = full_list
            .into_iter()
            .find(|manga| manga.identifier() == manga_identifier)
            .ok_or(EbiError::InvalidMangaIdentifier)?;
        Ok(manga)
    }
}

impl Source for YabuSource {}
