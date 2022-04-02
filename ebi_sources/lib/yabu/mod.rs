use crate::errors::Result;
use crate::MManga;
use crate::SSource;

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

    pub fn source() -> SSource {
        SSource {
            identifier: String::from("yabu"),
            title: String::from("Manga Yabu"),
            description: String::from("Manga Yabu! - Ler Mangás Online"),
            base_url: String::from("https://mangayabu.top"),
        }
    }

    pub async fn mangas(&self) -> Result<'i, Vec<MManga>> {
        let body = self.client.get_manga_list().await?;
        Ok(body)
    }
}
