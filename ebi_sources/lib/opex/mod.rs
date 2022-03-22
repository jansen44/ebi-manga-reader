use crate::{Manga, Source, SourceErrors};

mod client;
mod parser;

pub struct Opex {
    client: client::OpexClient,
}

impl Opex {
    pub fn new() -> Result<Self, SourceErrors> {
        let client = client::OpexClient::new(Opex::source())?;
        Ok(Self { client })
    }

    pub fn source() -> Source {
        Source {
            name: String::from("opex"),
            title: String::from("One Piece Ex"),
            description: String::from("One Piece Ex | De fã para fã"),
            base_url: String::from("https://onepieceex.net"),
        }
    }

    pub async fn mangas(&self) -> Vec<Manga> {
        vec![
            Manga {
                id: 0,
                title: String::from("One Piece"),
                name: String::from("main"),
                thumbnail: String::from(
                    "https://onepieceex.net/mangareader/sbs/capa/preview/Volume_1.jpg",
                ),
                url: String::from("/mangas"),
                source_name: Opex::source().name,
            },
            Manga {
                id: 1,
                title: String::from("SBS"),
                name: String::from("sbs"),
                thumbnail: String::from(
                    "https://onepieceex.net/mangareader/sbs/capa/preview/nao.jpg",
                ),
                url: String::from("/sbs"),
                source_name: Opex::source().name,
            },
            Manga {
                id: 2,
                title: String::from("Histórias de Capa"),
                name: String::from("covers"),
                thumbnail: String::from("https://onepieceex.net/mangareader/mangas/428/00_c.jpg"),
                url: String::from("//historias-de-capa"),
                source_name: Opex::source().name,
            },
        ]
    }

    pub async fn manga(&self, id: usize) -> Option<Manga> {
        let mangas = self.mangas().await;
        if id >= mangas.len() {
            return None;
        }
        Some(mangas[id].clone())
    }
}
