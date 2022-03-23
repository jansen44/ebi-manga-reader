use std::error;
use std::fmt::Display;

pub mod client;
pub mod opex;

#[derive(Clone, Debug)]
pub struct Source {
    pub name: String,
    pub title: String,
    pub description: String,
    pub base_url: String,
}

#[derive(Clone, Debug)]
pub struct Manga {
    pub id: usize,
    pub name: String,
    pub title: String,
    pub thumbnail: String,
    pub url: String,
    pub source_name: String,
}

#[derive(Clone, Debug)]
pub struct Chapter {
    pub id: usize,
    pub manga_id: usize,
    pub title: String,
    pub url: String,
    pub source_name: String,
}

#[derive(Debug)]
pub enum SourceErrors {
    ClientErrors(client::ClientErrors),
}

impl Display for SourceErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SourceErrors::ClientErrors(ref e) => write!(f, "{e}"),
        }
    }
}

impl error::Error for SourceErrors {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SourceErrors::ClientErrors(ref e) => Some(e),
        }
    }
}

impl From<client::ClientErrors> for SourceErrors {
    fn from(err: client::ClientErrors) -> Self {
        SourceErrors::ClientErrors(err)
    }
}
