use crate::errors::Result;
use crate::Manga;
use crate::Source;

mod client;
mod parser;

pub struct Yabu {
    client: client::YabuClient,
    parser: parser::Parser,
}

impl<'i> Yabu {
    pub fn new() -> Result<'i, Self> {
        let client = client::YabuClient::new(Self::source())?;
        let parser = parser::Parser::new();
        Ok(Self { client, parser })
    }

    pub fn source() -> Source {
        Source {
            name: String::from("yabu"),
            title: String::from("Manga Yabu"),
            description: String::from("Manga Yabu! - Ler Mangás Online"),
            base_url: String::from("https://mangayabu.top"),
        }
    }

    pub async fn mangas(&self) -> Result<'i, Vec<Manga>> {
        let body = self.client.get_manga_list().await?;
        Ok(body)
    }
}
