use anyhow::Result;

use crate::sources::chapter::Chapter;
use crate::sources::manga::TManga;

use super::client;
use super::{YABU_BASE_URL, YABU_SOURCE_IDENTIFIER};

mod manga_parser {
    use anyhow::Result;
    use scraper::{Html, Selector};

    use crate::sources::chapter::Chapter;
    use crate::sources::yabu::chapter::YabuChapterBuilder;

    pub fn chapter_list(manga_identifier: &str, html_page: &str) -> Result<Vec<Chapter>> {
        let html = Html::parse_document(html_page);
        let chapter_list_json = Selector::parse("#manga-info").unwrap();
        let chapter_list_json = html.select(&chapter_list_json).next().unwrap().inner_html();

        let page_list_json = serde_json::from_str::<serde_json::Value>(chapter_list_json.as_str())?;
        let page_list_json = page_list_json.as_object().unwrap();

        let title = page_list_json.get("chapter_name").unwrap();
        let title = title.as_str().unwrap();

        let all_posts_json = page_list_json.get("allposts").unwrap();
        let all_posts_json = all_posts_json.as_array().unwrap();

        let chapters: Vec<Chapter> = all_posts_json
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, chapter)| {
                let chapter = chapter.as_object().unwrap();

                let id = chapter.get("id").unwrap();
                let id = id.as_u64().unwrap() as usize;

                let num = chapter.get("num").unwrap();
                let num = num.as_str().unwrap();

                let title = format!("{} - {}", title, num);

                YabuChapterBuilder::new()
                    .with_chapter(idx + 1)
                    .with_title(title.as_str())
                    .with_yabu_id(id)
                    .with_manga_identifier(manga_identifier)
                    .build()
                    .into()
            })
            .collect();

        Ok(chapters)
    }
}

#[derive(Default)]
pub struct YabuMangaBuilder {
    identifier: Option<String>,
    title: Option<String>,
    cover: Option<String>,
    genre: Option<String>,
}

impl YabuMangaBuilder {
    pub fn new() -> Self {
        Self { ..Default::default() }
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

    pub fn with_genre(mut self, genre: &str) -> Self {
        self.genre = Some(genre.to_owned());
        self
    }

    pub fn build(&self) -> YabuManga {
        let identifier = self.identifier.clone().unwrap_or_default();
        let url = format!("{}/manga/{}/", YABU_BASE_URL, identifier.clone());

        YabuManga {
            identifier: self.identifier.clone().unwrap_or_default(),
            title: self.title.clone().unwrap_or_default(),
            cover: self.cover.clone().unwrap_or_default(),
            url,
            genre: self.genre.clone(),
        }
    }
}

#[derive(Default, Debug)]
pub struct YabuManga {
    pub identifier: String,
    pub title: String,
    pub cover: String,
    pub url: String,
    pub genre: Option<String>,
}

#[async_trait::async_trait]
impl TManga for YabuManga {
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
        self.url.clone()
    }

    fn genre(&self) -> Option<String> {
        self.genre.clone()
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn source_identifier(&self) -> String {
        YABU_SOURCE_IDENTIFIER.to_owned()
    }

    async fn chapter_list(&self) -> Result<Vec<Chapter>> {
        let page = client::yabu_html(self.url.as_str()).await?;
        let chapters = manga_parser::chapter_list(self.identifier().as_str(), page.as_str())?;
        Ok(chapters)
    }

    async fn chapter(&self, chapter: usize) -> Result<Option<Chapter>> {
        let mut chapter_list = self.chapter_list().await?;

        let chapter = match chapter {
            chapter if chapter > chapter_list.len() => chapter_list.len(),
            chapter if chapter == 0 => 1,
            _ => chapter,
        };

        let chapter = chapter_list.swap_remove(chapter - 1); // necessary in order to return owned Box
        Ok(Some(chapter))
    }
}
