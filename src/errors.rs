use std::error;
use std::fmt::Display;

#[derive(Debug)]
pub enum EbiError<'a> {
    InvalidSourceIdentifier,
    InvalidMangaIdentifier,

    ClientInvalidRequestBody(&'a str),

    ParserError(&'a str),
    ParserMissingElement(&'a str),
}

impl<'a> Display for EbiError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EbiError::InvalidSourceIdentifier => {
                write!(f, "error: Invalid source identifier")
            }
            EbiError::InvalidMangaIdentifier => {
                write!(f, "error: Invalid manga identifier")
            }
            EbiError::ClientInvalidRequestBody(err) => {
                write!(f, "error: Invalid body: {err}")
            }
            EbiError::ParserError(err) => {
                write!(f, "error: Error parsing html: {err}")
            }
            EbiError::ParserMissingElement(el) => {
                write!(f, "error: Error parsing html: Missing element {el}")
            }
        }
    }
}

impl<'a> error::Error for EbiError<'a> {}
