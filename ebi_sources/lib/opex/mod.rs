use crate::errors::Result;
use crate::Chapter;
use crate::Manga;
use crate::Source;

mod client;
mod parser;

pub struct Opex {
    client: client::OpexClient,
    parser: parser::Parser,
}

impl<'i> Opex {
    pub fn new() -> Result<'i, Self> {
        let client = client::OpexClient::new(Opex::source());
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

    pub async fn mangas(&self) -> Result<'i, Vec<Manga>> {
        Ok(vec![
            Manga {
                identifier: String::from("main"),
                title: String::from("One Piece"),
                thumbnail: String::from(
                    "https://onepieceex.net/mangareader/sbs/capa/preview/Volume_1.jpg",
                ),
                url: String::from("/mangas"),
                source_name: Opex::source().name,
            },
            Manga {
                identifier: String::from("sbs"),
                title: String::from("SBS"),
                thumbnail: String::from(
                    "https://onepieceex.net/mangareader/sbs/capa/preview/nao.jpg",
                ),
                url: String::from("/sbs"),
                source_name: Opex::source().name,
            },
            Manga {
                identifier: String::from("covers"),
                title: String::from("Histórias de Capa"),
                thumbnail: String::from("https://onepieceex.net/mangareader/mangas/428/00_c.jpg"),
                url: String::from("//historias-de-capa"),
                source_name: Opex::source().name,
            },
        ])
    }

    pub async fn manga(&self, identifier: &str) -> Result<'i, Option<Manga>> {
        let mangas = self.mangas().await?;
        let manga = mangas.iter().find(|el| el.identifier == identifier);
        match manga {
            Some(manga) => Ok(Some(manga.to_owned())),
            None => Ok(None),
        }
    }

    pub async fn chapters(&self, manga: &Manga) -> Result<'i, Vec<Chapter>> {
        let page = self.client.get_manga_web_page(manga).await?;
        let chapters = self.parser.get_chapter_list(manga, page.as_str());
        Ok(chapters)
    }

    pub async fn chapter(&self, manga: &Manga, id: usize) -> Result<'i, Option<Chapter>> {
        let chapters = self.chapters(manga).await?;
        let mut chapters = chapters.iter();

        let chapter = chapters.find(|chapter| chapter.id == id);
        if chapter.is_none() {
            return Ok(None);
        }

        let chapter = chapter.unwrap();
        Ok(Some(chapter.to_owned()))
    }

    pub async fn pages(&self, chapter: &Chapter) -> Result<'i, Vec<String>> {
        let page = self.client.get_chapter_web_page(chapter).await?;
        let pages = self.parser.get_page_list(page.as_str());
        Ok(pages)
    }
}
