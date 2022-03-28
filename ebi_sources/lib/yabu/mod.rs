use crate::Source;
use crate::SourceErrors;

mod client;
mod parser;

pub struct Yabu {
    client: client::YabuClient,
    parser: parser::Parser,
}

impl Yabu {
    pub fn new() -> Result<Self, SourceErrors> {
        let client = client::YabuClient {};
        let parser = parser::Parser {};
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
}
