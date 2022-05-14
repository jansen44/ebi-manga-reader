use anyhow::Result;

use crate::sources::chapter::TChapter;

use super::client;
use super::{OPEX_BASE_URL, OPEX_SOURCE_IDENTIFIER};

mod chapter_parser {
    use anyhow::Result;
    use scraper::{Html, Selector};

    use crate::errors::EbiError;
    use crate::sources::opex::OPEX_BASE_URL;

    pub fn chapter_page_list(chapter_page_body: &str) -> Result<Vec<String>> {
        let page = Html::parse_document(chapter_page_body);

        let script_selector = Selector::parse("#leitor-opex > strong > script").unwrap();
        let script_elem = page
            .select(&script_selector)
            .next()
            .ok_or(EbiError::ParserMissingElement("span"))?;

        let page_list_json = script_elem.inner_html();
        let page_list_json = page_list_json
            .split("paginasLista = ")
            .nth(1)
            .ok_or(EbiError::ParserError("\"paginasLista = \" identifier not found"))?;

        let page_list_json = page_list_json
            .split(";")
            .next()
            .ok_or(EbiError::ParserError("\";\" identifier not found on split"))?;

        let page_list_json = serde_json::from_str::<String>(page_list_json)?;
        let page_list_json: serde_json::Value = serde_json::from_str(page_list_json.as_str())?;
        let page_list_json = page_list_json
            .as_object()
            .ok_or(EbiError::ParserError("could not get object from serde_json::Value"))?;

        let mut page_list = page_list_json
            .iter()
            .map(|(key, value)| Ok((key.parse::<usize>()?, value.as_str().unwrap().to_owned())))
            .collect::<Result<Vec<(usize, String)>>>()?;
        page_list.sort_by(|a, b| a.0.cmp(&b.0));

        Ok(page_list
            .iter()
            .map(|(_, value)| format!("{}/{}", OPEX_BASE_URL, value))
            .collect())
    }
}

#[derive(Default, Debug)]
pub struct OpexChapter {
    pub chapter: usize,
    pub title: String,
    pub url: String,
    pub manga_identifier: String,
}

#[async_trait::async_trait]
impl TChapter for OpexChapter {
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

    async fn page_url_list(&self) -> Result<Vec<String>> {
        let page = client::opex_html_page(self.url.as_str()).await?;
        let pages = chapter_parser::chapter_page_list(page.as_str())?;
        Ok(pages)
    }
}
