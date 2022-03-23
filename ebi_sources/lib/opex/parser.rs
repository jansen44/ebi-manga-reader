use std::collections::HashMap;

use crate::{Chapter, Manga};
use scraper::{element_ref::Select, ElementRef, Html, Selector};

const MAIN_CHAPTER_LIST_SELECTOR: &str = "#volumes div.capitulos li.volume-capitulo";
const SBS_CHAPTER_LIST_SELECTOR: &str =
    "#conteudo #post > a.text-uppercase.sombra-clara.bnt-lista-horizontal";
const COVERS_CHAPTER_LIST_SELECTOR: &str =
    "#post div.volume.text-uppercase div.capitulos li.volume-capitulo";

pub struct Parser {
    chapter_list_selectors: HashMap<String, Selector>,
}

impl Parser {
    pub fn new() -> Self {
        let chapter_list_selectors = Self::chapter_list_selectors();
        Self {
            chapter_list_selectors,
        }
    }

    fn get_child_span_value(element: ElementRef) -> String {
        let selector = Selector::parse("span").unwrap();
        element.select(&selector).next().unwrap().inner_html()
    }

    fn href_from_child_anchor(element: ElementRef) -> String {
        element.value().attr("href").unwrap().to_owned()
    }

    fn chapter_list_selectors() -> HashMap<String, Selector> {
        let mut chapter_list_selectors = HashMap::new();
        chapter_list_selectors.insert(
            String::from("main"),
            Selector::parse(MAIN_CHAPTER_LIST_SELECTOR).unwrap(),
        );
        chapter_list_selectors.insert(
            String::from("sbs"),
            Selector::parse(SBS_CHAPTER_LIST_SELECTOR).unwrap(),
        );
        chapter_list_selectors.insert(
            String::from("covers"),
            Selector::parse(COVERS_CHAPTER_LIST_SELECTOR).unwrap(),
        );
        chapter_list_selectors
    }

    fn get_id_from_title(base_title: String) -> (usize, String) {
        let mut id = "";
        let mut title = String::new();

        let title_bytes = base_title.as_bytes();
        for (i, &item) in title_bytes.iter().enumerate() {
            if item == b'.' {
                id = &base_title[0..i];
                title = String::from(&base_title[i + 2..]);
                break;
            }
        }
        let id: usize = id.parse().unwrap();

        (id, title)
    }

    fn main_chapter_from_element(manga: &Manga, element: ElementRef) -> Chapter {
        let base_title = Self::get_child_span_value(element);
        let url = Self::href_from_child_anchor(
            element
                .select(&Selector::parse("a.online").unwrap())
                .next()
                .unwrap(),
        );
        let (id, title) = Self::get_id_from_title(base_title);

        Chapter {
            id,
            mangaId: manga.id,
            title,
            url,
            source_name: manga.source_name.clone(),
        }
    }

    fn chapter_from_element(manga: &Manga, element: ElementRef) -> Option<Chapter> {
        //    match manga.name.as_str() {
        //        "main" => Some(Chapter{
        //            id: 0,
        //            mangaId: manga.id,
        //            source_name: manga.source_name.clone(),
        //            title: String::from(element.select(&Selector::parse("span").unwrap()).next().),
        //            url: String::new(),
        //        }),
        //        _ => None,
        //    }

        let chapter = Self::main_chapter_from_element(manga, element);
        // println!("{:?}", chapter);

        Some(chapter)
    }

    pub fn get_chapter_list(&self, manga: &Manga, manga_page_body: String) -> Vec<Chapter> {
        let page = Html::parse_document(manga_page_body.as_str());

        let selector = self.chapter_list_selectors.get(&manga.name);
        if selector.is_none() {
            return vec![];
        }

        let mut chapters = vec![];
        for element in page.select(selector.unwrap()) {
            let chapter = Self::chapter_from_element(manga, element);
            if let Some(chapter) = chapter {
                chapters.push(chapter);
            }
        }
        chapters
    }
}
