use crate::manga::Manga;
use crate::source::{Source, SourceData, SourceInfo};
use crate::Result;

pub mod chapter;
pub mod manga;

const OPEX_SOURCE_IDENTIFIER: &str = "opex";
const OPEX_BASE_URL: &str = "https://onepieceex.net";

#[derive(Debug, Clone)]
pub struct OpexSource {
    pub identifier: String,
    pub title: String,
    pub description: String,
    pub base_url: String,
}

impl OpexSource {
    pub fn default() -> Self {
        Self {
            base_url: OPEX_BASE_URL.to_owned(),
            identifier: OPEX_SOURCE_IDENTIFIER.to_owned(),
            title: String::from("One Piece Ex"),
            description: String::from("One Piece Ex | De fã para fã"),
        }
    }
}

impl SourceInfo for OpexSource {
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
impl SourceData for OpexSource {
    async fn manga_list(&self) -> Result<Vec<Box<dyn Manga>>> {
        let cover = manga::OpexMangaBuilder::new()
            .with_identifier("covers")
            .with_title("One Piece - Histórias de Capa")
            .with_cover("https://onepieceex.net/mangareader/mangas/428/00_c.jpg")
            .with_url("/historias-de-capa")
            .build();
        let cover: Box<dyn Manga> = Box::new(cover);

        let main = manga::OpexMangaBuilder::new()
            .with_identifier("main")
            .with_title("One Piece")
            .with_cover("https://onepieceex.net/mangareader/sbs/capa/preview/Volume_1.jpg")
            .with_url("/mangas")
            .build();
        let main: Box<dyn Manga> = Box::new(main);

        let sbs = manga::OpexMangaBuilder::new()
            .with_identifier("sbs")
            .with_title("One Piece - SBS")
            .with_cover("https://onepieceex.net/mangareader/sbs/capa/preview/nao.jpg")
            .with_url("/sbs")
            .build();
        let sbs: Box<dyn Manga> = Box::new(sbs);

        Ok(vec![cover, main, sbs])
    }

    async fn latest_manga(&self) -> Result<Vec<Box<dyn Manga>>> {
        self.manga_list().await
    }

    async fn popular_manga(&self) -> Result<Vec<Box<dyn Manga>>> {
        self.manga_list().await
    }

    async fn hot_manga(&self) -> Result<Vec<Box<dyn Manga>>> {
        self.manga_list().await
    }

    async fn search_manga(&self, _manga_title: &str) -> Result<Vec<Box<dyn Manga>>> {
        self.manga_list().await
    }

    async fn get_manga(&self, manga_identifier: &str) -> Result<Option<Box<dyn Manga>>> {
        let manga_list = self.manga_list().await?;
        let manga = manga_list
            .into_iter()
            .find(|m| m.identifier() == manga_identifier);
        Ok(manga)
    }
}

impl Source for OpexSource {}

// ==========================================================
// pub struct Opex {
//     client: client::OpexClient,
//     parser: parser::Parser,
// }

// impl<'i> Opex {
//     pub async fn chapters(&self, manga: &MManga) -> Result<'i, Vec<CChapter>> {
//         let page = self.client.get_manga_web_page(manga).await?;
//         let chapters = self.parser.get_chapter_list(manga, page.as_str())?;
//         Ok(chapters)
//     }

//     pub async fn chapter(&self, manga: &MManga, id: usize) -> Result<'i, Option<CChapter>> {
//         let chapters = self.chapters(manga).await?;
//         let mut chapters = chapters.iter();

//         let chapter = chapters.find(|chapter| chapter.id == id);
//         if chapter.is_none() {
//             return Ok(None);
//         }

//         let chapter = chapter.unwrap();
//         Ok(Some(chapter.to_owned()))
//     }

//     pub async fn pages(&self, chapter: &CChapter) -> Result<'i, Vec<String>> {
//         let page = self.client.get_chapter_web_page(chapter).await?;
//         let pages = self.parser.get_page_list(page.as_str())?;
//         Ok(pages)
//     }
// }
