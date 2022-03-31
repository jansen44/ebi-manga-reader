pub mod errors;

pub mod opex;
pub mod yabu;

#[derive(Clone, Debug)]
pub struct Source {
    pub name: String,
    pub title: String,
    pub description: String,
    pub base_url: String,
}

#[derive(Clone, Debug)]
pub struct Manga {
    pub identifier: String,
    pub title: String,
    pub thumbnail: String,
    pub url: String,
    pub source_name: String,
}

#[derive(Clone, Debug)]
pub struct Chapter {
    pub id: usize,
    pub manga_identifier: String,
    pub title: String,
    pub url: String,
    pub source_name: String,
}

