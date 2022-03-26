use crate::{client::ClientResult, Chapter, Manga, Source, SourceErrors};

mod client;
mod parser;

pub struct Opex {
    client: client::OpexClient,
    parser: parser::Parser,
}

impl Opex {
    pub fn new() -> Result<Self, SourceErrors> {
        let client = client::OpexClient::new(Opex::source())?;
        let parser = parser::Parser::new();
        Ok(Self { client, parser })
    }

    pub fn source() -> Source {
        Source {
            name: String::from("opex"),
            title: String::from("One Piece Ex"),
            description: String::from("One Piece Ex | De fã para fã"),
            base_url: String::from("https://onepieceex.net"),
        }
    }

    pub async fn mangas(&self) -> ClientResult<Vec<Manga>> {
        Ok(vec![
            Manga {
                id: 1,
                title: String::from("One Piece"),
                name: String::from("main"),
                thumbnail: String::from(
                    "https://onepieceex.net/mangareader/sbs/capa/preview/Volume_1.jpg",
                ),
                url: String::from("/mangas"),
                source_name: Opex::source().name,
            },
            Manga {
                id: 2,
                title: String::from("SBS"),
                name: String::from("sbs"),
                thumbnail: String::from(
                    "https://onepieceex.net/mangareader/sbs/capa/preview/nao.jpg",
                ),
                url: String::from("/sbs"),
                source_name: Opex::source().name,
            },
            Manga {
                id: 3,
                title: String::from("Histórias de Capa"),
                name: String::from("covers"),
                thumbnail: String::from("https://onepieceex.net/mangareader/mangas/428/00_c.jpg"),
                url: String::from("//historias-de-capa"),
                source_name: Opex::source().name,
            },
        ])
    }

    pub async fn manga(&self, id: usize) -> ClientResult<Option<Manga>> {
        let mangas = self.mangas().await?;
        if id > mangas.len() || id == 0 {
            return Ok(None);
        }
        Ok(Some(mangas[id - 1].clone()))
    }

    pub async fn chapters(&self, manga: &Manga) -> ClientResult<Vec<Chapter>> {
        let page = self.client.get_manga_web_page(manga).await?;
        let chapters = self.parser.get_chapter_list(manga, page.as_str());
        Ok(chapters)
    }

    pub async fn chapter(&self, manga: &Manga, id: usize) -> ClientResult<Option<Chapter>> {
        let chapters = self.chapters(manga).await?;
        let mut chapters = chapters.iter();

        let chapter = chapters.find(|chapter| chapter.id == id);
        if chapter.is_none() {
            return Ok(None);
        }

        let chapter = chapter.unwrap();
        Ok(Some(chapter.to_owned()))
    }

    pub async fn pages(&self, chapter: &Chapter) -> ClientResult<Vec<String>> {
        let page = self.client.get_chapter_web_page(chapter).await?;
        let pages = self.parser.get_page_list(page.as_str());
        Ok(pages)
    }
}
