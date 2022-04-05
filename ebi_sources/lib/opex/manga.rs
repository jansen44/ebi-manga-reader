use crate::{Manga, MangaData, MangaInfo};

#[derive(Default)]
pub struct OpexMangaBuilder {
    identifier: Option<String>,
    title: Option<String>,
    cover: Option<String>,
    url: Option<String>,
    source_identifier: Option<String>,
}

impl OpexMangaBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_identifier(mut self, identifier: &str) -> Self {
        self.identifier = Some(identifier.to_owned());
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn with_cover(mut self, cover: &str) -> Self {
        self.cover = Some(cover.to_owned());
        self
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }

    pub fn with_source(mut self, source: &str) -> Self {
        self.source_identifier = Some(source.to_owned());
        self
    }

    pub fn build(&self) -> OpexManga {
        OpexManga {
            identifier: self.identifier.clone().unwrap_or_default(),
            title: self.title.clone().unwrap_or_default(),
            cover: self.cover.clone().unwrap_or_default(),
            url: self.url.clone().unwrap_or_default(),
            source_identifier: self.source_identifier.clone().unwrap_or_default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct OpexManga {
    pub identifier: String,
    pub title: String,
    pub cover: String,
    pub url: String,
    pub source_identifier: String,
}

// impl Manga {
//     pub fn builder() -> Self {
//         Self { ..Default::default() }
//     }

//     pub fn with_identifier(mut self, identifier: &str) -> Self {
//         self.identifier = identifier.to_owned();
//         self
//     }

//     pub fn with_title(mut self, title: &str) -> Self {
//         self.title = title.to_owned();
//         self
//     }

//     pub fn with_cover(mut self, cover: &str) -> Self {
//         self.cover = cover.to_owned();
//         self
//     }

//     pub fn with_url(mut self, url: &str) -> Self {
//         self.url = url.to_owned();
//         self
//     }

//     pub fn with_genre(mut self, genre: &str) -> Self {
//         self.genre = Some(genre.to_owned());
//         self
//     }

//     pub fn with_description(mut self, description: &str) -> Self {
//         self.description = Some(description.to_owned());
//         self
//     }

//     pub fn with_source(mut self, source_identifier: &str) -> Self {
//         self.source_identifier = source_identifier.to_owned();
//         self
//     }
// }
