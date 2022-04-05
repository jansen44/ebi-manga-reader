use crate::Manga;
use crate::Result;
use crate::{Source, SourceData, SourceInfo};

pub mod manga;

const OPEX_SOURCE_IDENTIFIER: &str = "opex";

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
            base_url: String::from("https://onepieceex.net"),
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
    async fn manga_list(&self) -> Result<Box<dyn Manga>> {
        todo!()
    }

    async fn latest_manga(&self) -> Result<Box<dyn Manga>> {
        todo!()
    }

    async fn popular_manga(&self) -> Result<Box<dyn Manga>> {
        todo!()
    }

    async fn hot_manga(&self) -> Result<Box<dyn Manga>> {
        todo!()
    }

    async fn search_manga(&self, _manga_title: &str) -> Result<Box<dyn Manga>> {
        todo!()
    }

    async fn get_manga(&self, _manga_identifier: &str) -> Result<Option<Box<dyn Manga>>> {
        todo!()
    }
}

impl Source for OpexSource {}

// ==========================================================

// use crate::errors::Result;
// use crate::CChapter;
// use crate::MManga;
// use crate::SSource;

// pub mod source;
// pub mod manga;
// pub mod chapter;

// mod client;
// mod parser;

// pub struct Opex {
//     client: client::OpexClient,
//     parser: parser::Parser,
// }

// impl<'i> Opex {
//     pub fn new() -> Result<'i, Self> {
//         let client = client::OpexClient::new(Opex::source().base_url.clone());
//         let parser = parser::Parser::new();
//         Ok(Self { client, parser })
//     }

//     pub fn source() -> SSource {
//         SSource {
//             identifier: String::from("opex"),
//             title: String::from("One Piece Ex"),
//             description: String::from("One Piece Ex | De fã para fã"),
//             base_url: String::from("https://onepieceex.net"),
//         }
//     }

//     pub async fn mangas(&self) -> Result<'i, Vec<MManga>> {
//         Ok(vec![
//             MManga {
//                 identifier: String::from("covers"),
//                 title: String::from("One Piece - Histórias de Capa"),
//                 thumbnail: String::from("https://onepieceex.net/mangareader/mangas/428/00_c.jpg"),
//                 url: String::from("//historias-de-capa"),
//                 source_identifier: Opex::source().identifier,
//                 description: None,
//                 genre: None,
//             },
//             MManga {
//                 identifier: String::from("main"),
//                 title: String::from("One Piece"),
//                 thumbnail: String::from(
//                     "https://onepieceex.net/mangareader/sbs/capa/preview/Volume_1.jpg",
//                 ),
//                 url: String::from("/mangas"),
//                 source_identifier: Opex::source().identifier,
//                 description: None,
//                 genre: None,
//             },
//             MManga {
//                 identifier: String::from("sbs"),
//                 title: String::from("One Piece - SBS"),
//                 thumbnail: String::from(
//                     "https://onepieceex.net/mangareader/sbs/capa/preview/nao.jpg",
//                 ),
//                 url: String::from("/sbs"),
//                 source_identifier: Opex::source().identifier,
//                 description: None,
//                 genre: None,
//             },
//         ])
//     }

//     pub async fn manga(&self, identifier: &str) -> Result<'i, Option<MManga>> {
//         let mangas = self.mangas().await?;
//         let manga = mangas.iter().find(|el| el.identifier == identifier);
//         match manga {
//             Some(manga) => Ok(Some(manga.to_owned())),
//             None => Ok(None),
//         }
//     }

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
