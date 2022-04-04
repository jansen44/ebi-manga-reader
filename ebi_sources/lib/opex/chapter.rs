#[derive(Default)]
pub struct Chapter {
    pub chapter: usize,
    pub title: String,
    pub url: String,
    pub manga_identifier: String,
    pub source_identifier: String,
}


impl Chapter {
    pub fn builder() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_chapter(mut self, chapter: usize) -> Self {
        self.chapter = chapter;
        self
    }
    
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }
    
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = url.to_owned();
        self
    }

    pub fn with_manga(mut self, manga_identifier: &str) -> Self {
        self.manga_identifier = manga_identifier.to_owned();
        self
    }

    pub fn with_source(mut self, source_identifier: &str) -> Self {
        self.source_identifier = source_identifier.to_owned();
        self
    }
}
