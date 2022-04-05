// use std::collections::HashMap;

// use scraper::{ElementRef, Html, Selector};

// use crate::errors::parser::{ParserError, ParserResult};
// use crate::{CChapter, MManga};

// const MAIN_CHAPTER_LIST_SELECTOR: &str = "#volumes div.capitulos li.volume-capitulo";
// const SBS_CHAPTER_LIST_SELECTOR: &str =
//     "#conteudo #post > a.text-uppercase.sombra-clara.bnt-lista-horizontal";
// const COVERS_CHAPTER_LIST_SELECTOR: &str =
//     "#post div.volume.text-uppercase div.capitulos li.volume-capitulo";

// type ChapterInfo = (usize, String, String);

// pub struct Parser {
//     chapter_list_selectors: HashMap<String, Selector>,
// }

// impl<'i> Parser {
//     pub fn new() -> Self {
//         let chapter_list_selectors = Self::chapter_list_selectors();
//         Self {
//             chapter_list_selectors,
//         }
//     }

//     pub fn get_chapter_list(
//         &self,
//         manga: &MManga,
//         manga_page_body: &str,
//     ) -> ParserResult<'i, Vec<CChapter>> {
//         let page = Html::parse_document(manga_page_body);

//         let selector = self.chapter_list_selectors.get(&manga.identifier);
//         if selector.is_none() {
//             return Ok(vec![]);
//         }
//         let selector = selector.unwrap();

//         let mut chapters = vec![];
//         let mut current_id = 1;
//         for element in page.select(selector) {
//             let chapter = Self::chapter_from_element(manga, element)?;
//             if let Some(chapter) = chapter {
//                 let mut chapter = chapter;
//                 if chapter.id == 0 {
//                     chapter.id = current_id;
//                     current_id += 1;
//                 }
//                 chapters.push(chapter);
//             }
//         }
//         Ok(chapters)
//     }

//     pub fn get_page_list(&self, chapter_page_body: &str) -> ParserResult<'i, Vec<String>> {
//         let page = Html::parse_document(chapter_page_body);

//         let script_selector = Selector::parse("#leitor-opex > strong > script").unwrap();
//         let script_elem = match page.select(&script_selector).next() {
//             Some(el) => el.inner_html(),
//             None => {
//                 return Err(ParserError::MissingElement(
//                     "#leitor-opex > strong > script",
//                 ))
//             }
//         };

//         let page_list_json = match script_elem.split("paginasLista = ").nth(1) {
//             Some(content) => content,
//             None => {
//                 return Err(ParserError::Other(
//                     "\"paginasLisa = \" identifier not found",
//                 ))
//             }
//         };

//         let page_list_json = match page_list_json.split(";").next() {
//             Some(content) => content,
//             None => return Err(ParserError::Other("\";\" identifier not found on split")),
//         };

//         let page_list_json = serde_json::from_str::<String>(page_list_json)?;
//         let page_list_json = page_list_json.as_str();
//         let page_list_json: serde_json::Value = serde_json::from_str(page_list_json)?;
//         let page_list_json = match page_list_json.as_object() {
//             Some(page_list) => page_list,
//             None => {
//                 return Err(ParserError::Other(
//                     "could not get object from serde_json::Value",
//                 ));
//             }
//         };

//         let mut page_list = page_list_json
//             .iter()
//             .map(|(key, value)| Ok((key.parse::<usize>()?, value.as_str().unwrap().to_owned())))
//             .collect::<ParserResult<'i, Vec<(usize, String)>>>()?;
//         page_list.sort_by(|a, b| a.0.cmp(&b.0));

//         Ok(page_list.iter().map(|(_, value)| value.clone()).collect())
//     }

//     fn get_child_span_value(element: ElementRef) -> ParserResult<'i, String> {
//         let selector = Selector::parse("span")?;
//         match element.select(&selector).next() {
//             Some(span) => Ok(span.inner_html()),
//             None => Err(ParserError::MissingElement("span")),
//         }
//     }

//     fn href_from_anchor(element: ElementRef) -> ParserResult<'i, String> {
//         match element.value().attr("href") {
//             Some(href) => Ok(href.to_owned()),
//             None => Err(ParserError::MissingElement("href")),
//         }
//     }

//     fn chapter_list_selectors() -> HashMap<String, Selector> {
//         let mut chapter_list_selectors = HashMap::new();
//         chapter_list_selectors.insert(
//             String::from("main"),
//             Selector::parse(MAIN_CHAPTER_LIST_SELECTOR).unwrap(),
//         );
//         chapter_list_selectors.insert(
//             String::from("sbs"),
//             Selector::parse(SBS_CHAPTER_LIST_SELECTOR).unwrap(),
//         );
//         chapter_list_selectors.insert(
//             String::from("covers"),
//             Selector::parse(COVERS_CHAPTER_LIST_SELECTOR).unwrap(),
//         );
//         chapter_list_selectors
//     }

//     fn get_id_from_title(base_title: &str) -> ParserResult<'i, (usize, String)> {
//         let mut id = "";
//         let mut title = String::new();
//         let base_title = base_title.to_owned();

//         let title_bytes = base_title.as_bytes();
//         for (i, &item) in title_bytes.iter().enumerate() {
//             if item == b'.' {
//                 id = &base_title[0..i];
//                 title = String::from(&base_title[i + 2..]);
//                 break;
//             }
//         }

//         let id = id.parse::<usize>()?;
//         Ok((id, title))
//     }

//     fn main_chapter_info_from_element(element: ElementRef) -> ParserResult<'i, ChapterInfo> {
//         let base_title = Self::get_child_span_value(element)?;

//         let anchor_selector = &Selector::parse("a.online").unwrap();
//         let anchor = match element.select(&anchor_selector).next() {
//             Some(el) => el,
//             None => return Err(ParserError::MissingElement("a.online")),
//         };
//         let url = Self::href_from_anchor(anchor)?;

//         let (id, title) = Self::get_id_from_title(base_title.as_str())?;
//         Ok((id, title, url))
//     }

//     fn sbs_chapter_info_from_element(element: ElementRef) -> ParserResult<'i, ChapterInfo> {
//         let title = Self::get_child_span_value(element)?;
//         let url = Self::href_from_anchor(element)?;
//         Ok((0, title, url))
//     }

//     fn covers_chapter_info_from_element(element: ElementRef) -> ParserResult<'i, ChapterInfo> {
//         let title = match element.text().next() {
//             Some(t) => t,
//             None => return Err(ParserError::MissingElement("element text()")),
//         };
//         let title = (&title[..title.len() - 1]).to_owned();

//         let anchor_selector = &Selector::parse("a.online").unwrap();
//         let anchor = match element.select(&anchor_selector).next() {
//             Some(el) => el,
//             None => return Err(ParserError::MissingElement("a.online")),
//         };
//         let url = Self::href_from_anchor(anchor)?;

//         Ok((0, title, url))
//     }

//     fn chapter_from_element(
//         manga: &MManga,
//         element: ElementRef,
//     ) -> ParserResult<'i, Option<CChapter>> {
//         let info = match manga.identifier.as_str() {
//             "main" => Some(Self::main_chapter_info_from_element(element)),
//             "sbs" => Some(Self::sbs_chapter_info_from_element(element)),
//             "covers" => Some(Self::covers_chapter_info_from_element(element)),
//             _ => None,
//         };

//         if info.is_none() {
//             return Ok(None);
//         }

//         let (id, title, url) = info.unwrap()?;
//         Ok(Some(CChapter {
//             id,
//             manga_identifier: manga.identifier.clone(),
//             title,
//             url,
//             source_identifier: manga.source_identifier.clone(),
//         }))
//     }
// }
