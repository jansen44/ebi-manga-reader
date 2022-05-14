use anyhow::Result;

use crate::errors::EbiError;
use crate::sources::manga::Manga;
use crate::sources::source::TSource;

mod client;

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

#[async_trait::async_trait]
impl TSource for OpexSource {
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

    async fn manga_list(&self) -> Result<Vec<Manga>> {
        let cover = manga::OpexMangaBuilder::new()
            .with_identifier("covers")
            .with_title("One Piece - Histórias de Capa")
            .with_cover("https://onepieceex.net/mangareader/mangas/428/00_c.jpg")
            .with_url("/historias-de-capa")
            .build();

        let main = manga::OpexMangaBuilder::new()
            .with_identifier("main")
            .with_title("One Piece")
            .with_cover("https://onepieceex.net/mangareader/sbs/capa/preview/Volume_1.jpg")
            .with_url("/mangas")
            .build();

        let sbs = manga::OpexMangaBuilder::new()
            .with_identifier("sbs")
            .with_title("One Piece - SBS")
            .with_cover("https://onepieceex.net/mangareader/sbs/capa/preview/nao.jpg")
            .with_url("/sbs")
            .build();

        Ok(vec![cover.into(), main.into(), sbs.into()])
    }

    async fn latest_manga(&self) -> Result<Vec<Manga>> {
        self.manga_list().await
    }

    async fn popular_manga(&self) -> Result<Vec<Manga>> {
        self.manga_list().await
    }

    async fn hot_manga(&self) -> Result<Vec<Manga>> {
        self.manga_list().await
    }

    async fn search_manga(&self, _manga_title: &str) -> Result<Vec<Manga>> {
        self.manga_list().await
    }

    async fn get_manga(&self, manga_identifier: &str) -> Result<Manga> {
        let manga_list = self.manga_list().await?;
        let manga = manga_list
            .into_iter()
            .find(|m| m.identifier() == manga_identifier)
            .ok_or(EbiError::InvalidMangaIdentifier)?;
        Ok(manga)
    }
}
