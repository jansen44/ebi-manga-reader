use std::error;
use std::fmt::Display;

use reqwest::header;

pub type SourceResult<T> = Result<T, ClientErrors>;

#[derive(Debug)]
pub enum ClientErrors {
    InvalidHeaderValue(header::InvalidHeaderValue),
    ReqwestError(reqwest::Error),
}

impl Display for ClientErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ClientErrors::InvalidHeaderValue(ref e) => write!(f, "error: {e}"),
            ClientErrors::ReqwestError(ref e) => write!(f, "error: {e}"),
        }
    }
}

impl error::Error for ClientErrors {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ClientErrors::InvalidHeaderValue(ref e) => Some(e),
            ClientErrors::ReqwestError(ref e) => Some(e),
        }
    }
}

impl From<header::InvalidHeaderValue> for ClientErrors {
    fn from(err: header::InvalidHeaderValue) -> Self {
        ClientErrors::InvalidHeaderValue(err)
    }
}

impl From<reqwest::Error> for ClientErrors {
    fn from(err: reqwest::Error) -> Self {
        ClientErrors::ReqwestError(err)
    }
}


#[derive(Debug)]
pub enum SourceErrors {
    ClientErrors(ClientErrors),
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

impl From<ClientErrors> for SourceErrors {
    fn from(err: ClientErrors) -> Self {
        SourceErrors::ClientErrors(err)
    }
}