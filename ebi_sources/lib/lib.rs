pub mod client;
pub mod opex;

pub struct Source {
    name: String,
    title: String,
    description: String,
    base_url: String,
}

pub struct Manga {
    id: usize,
    name: String,
    title: String,
    thumbnail: String,
    url: String,
    source_name: String,
}

pub struct Chapter {
    id: usize,
    mangaId: usize,
    title: String,
    url: String,
    source_name: String,
}
