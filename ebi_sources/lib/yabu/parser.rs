use scraper::{ElementRef, Html, Selector};

use crate::Manga;

use super::Yabu;

pub struct Parser {
    manga_card_selector: Selector,
    popular_manga_list_container_selector: Selector,
}

impl Parser {
    pub fn new() -> Self {
        let manga_card_selector = Selector::parse(".manga-card").unwrap();

        let popular_manga_list_container_selector =
            Selector::parse("#main > .row > .carousel.s12").unwrap();

        Self {
            manga_card_selector,
            popular_manga_list_container_selector,
        }
    }

    pub fn popular_manga_from_page(&self, page_body: &str) -> Vec<Manga> {
        let page = Html::parse_document(page_body);

        let manga_list_container = page
            .select(&self.popular_manga_list_container_selector)
            .next()
            .unwrap();

        let manga_list: Vec<Manga> = manga_list_container
            .select(&self.manga_card_selector)
            .map(Self::manga_from_card)
            .collect();

        manga_list
    }

    fn image_from_manga_card(el: ElementRef) -> String {
        let selector = Selector::parse(".image > img").unwrap();
        let img = el.select(&selector).next().unwrap().value();

        match img.attr("src") {
            Some(src) => src.to_owned(),
            None => img.attr("data-cfsrc").unwrap().to_owned(),
        }
    }

    fn title_url_from_manga_card(el: ElementRef) -> (String, String) {
        let selector = Selector::parse(".info-bottom > a").unwrap();
        let anchor = el.select(&selector).next().unwrap();

        let url = anchor.value().attr("href").unwrap().to_owned();
        let title = anchor.inner_html();

        (url, title)
    }

    fn manga_from_card(el: ElementRef) -> Manga {
        let thumbnail = Self::image_from_manga_card(el);
        let (url, title) = Self::title_url_from_manga_card(el);

        let preffix = format!("{}/manga/", Yabu::source().base_url.clone());
        let identifier = url.clone();
        let identifier = identifier.split(preffix.as_str()).nth(1).unwrap();
        let identifier = identifier.split("/").next().unwrap().to_owned();

        Manga {
            identifier,
            url,
            title,
            thumbnail,
            source_identifier: Yabu::source().identifier.clone(),
        }
    }
}
