// #[derive(Default)]
// pub struct Manga {
//     pub identifier: String,
//     pub title: String,
//     pub cover: String,
//     pub url: String,
//     pub genre: Option<String>,
//     pub description: Option<String>,
//     pub source_identifier: String,
// }

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

