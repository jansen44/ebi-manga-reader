use std::error;
use std::fmt::Display;

pub mod client;
pub mod opex;

#[derive(Clone, Debug)]
pub struct Source {
    name: String,
    title: String,
    description: String,
    base_url: String,
}

#[derive(Clone, Debug)]
pub struct Manga {
    id: usize,
    name: String,
    title: String,
    thumbnail: String,
    url: String,
    source_name: String,
}

#[derive(Clone, Debug)]
pub struct Chapter {
    id: usize,
    mangaId: usize,
    title: String,
    url: String,
    source_name: String,
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
