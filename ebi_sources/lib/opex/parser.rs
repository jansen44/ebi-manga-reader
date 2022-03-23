use std::collections::HashMap;

use crate::{Chapter, Manga};
use scraper::{ElementRef, Html, Selector};

const MAIN_CHAPTER_LIST_SELECTOR: &str = "#volumes div.capitulos li.volume-capitulo";
const SBS_CHAPTER_LIST_SELECTOR: &str =
    "#conteudo #post > a.text-uppercase.sombra-clara.bnt-lista-horizontal";
const COVERS_CHAPTER_LIST_SELECTOR: &str =
    "#post div.volume.text-uppercase div.capitulos li.volume-capitulo";

type ChapterInfo = (usize, String, String);

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

    pub fn get_chapter_list(&self, manga: &Manga, manga_page_body: String) -> Vec<Chapter> {
        let page = Html::parse_document(manga_page_body.as_str());

        let selector = self.chapter_list_selectors.get(&manga.name);
        if selector.is_none() {
            return vec![];
        }

        let mut chapters = vec![];
        let mut current_id = 1;
        for element in page.select(selector.unwrap()) {
            let chapter = Self::chapter_from_element(manga, element);
            if let Some(chapter) = chapter {
                let mut chapter = chapter;
                if chapter.id == 0 {
                    chapter.id = current_id;
                    current_id += 1;
                }
                chapters.push(chapter);
            }
        }
        chapters
    }

    pub fn get_page_list(&self, chapter_page_body: String) -> Vec<String> {
        let page = Html::parse_document(chapter_page_body.as_str());
        let script_selector = Selector::parse("#leitor-opex > strong > script").unwrap();

        let script_elem = page.select(&script_selector).next().unwrap().inner_html();
        let script_elem = script_elem.split("paginasLista = ").nth(1).unwrap();
        let script_elem = script_elem.split(";").next().unwrap();

        let raw_json_object: serde_json::Value = serde_json::from_str(script_elem).unwrap();
        let str_raw_json_object = raw_json_object.as_str().unwrap();
        let raw_json_object: serde_json::Value = serde_json::from_str(str_raw_json_object).unwrap();
        let raw_json_object = raw_json_object.as_object().unwrap();

        let page_list: Vec<(&String, &serde_json::Value)> = raw_json_object.iter().collect();
        let mut page_list = page_list
            .iter()
            .map(|(key, value)| {
                (
                    key.parse::<usize>().unwrap(),
                    value.as_str().unwrap().to_owned(),
                )
            })
            .collect::<Vec<(usize, String)>>();
        page_list.sort_by(|a, b| a.0.cmp(&b.0));

        page_list.iter().map(|(_, value)| value.clone()).collect()
    }

    fn get_child_span_value(element: ElementRef) -> String {
        let selector = Selector::parse("span").unwrap();
        element.select(&selector).next().unwrap().inner_html()
    }

    fn href_from_anchor(element: ElementRef) -> String {
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

    fn main_chapter_info_from_element(element: ElementRef) -> ChapterInfo {
        let base_title = Self::get_child_span_value(element);
        let url = Self::href_from_anchor(
            element
                .select(&Selector::parse("a.online").unwrap())
                .next()
                .unwrap(),
        );
        let (id, title) = Self::get_id_from_title(base_title);
        (id, title, url)
    }

    fn sbs_chapter_info_from_element(element: ElementRef) -> ChapterInfo {
        let title = Self::get_child_span_value(element);
        let url = Self::href_from_anchor(element);
        (0, title, url)
    }

    fn covers_chapter_info_from_element(element: ElementRef) -> ChapterInfo {
        let title = element.text().next().unwrap();
        let title = &title[..title.len() - 1];
        let title = title.to_owned();
        let url = Self::href_from_anchor(
            element
                .select(&Selector::parse("a.online").unwrap())
                .next()
                .unwrap(),
        );
        (0, title, url)
    }

    fn chapter_from_element(manga: &Manga, element: ElementRef) -> Option<Chapter> {
        let info = match manga.name.as_str() {
            "main" => Some(Self::main_chapter_info_from_element(element)),
            "sbs" => Some(Self::sbs_chapter_info_from_element(element)),
            "covers" => Some(Self::covers_chapter_info_from_element(element)),
            _ => None,
        };

        match info {
            Some((id, title, url)) => Some(Chapter {
                id,
                manga_id: manga.id,
                title,
                url,
                source_name: manga.source_name.clone(),
            }),
            None => None,
        }
    }
}
