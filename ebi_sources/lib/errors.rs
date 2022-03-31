use std::error;
use std::fmt::Display;

use selectors::parser::SelectorParseErrorKind;

pub type ParseError<'a> = cssparser::ParseError<'a, SelectorParseErrorKind<'a>>;
pub type ParseResult<'a, T> = std::result::Result<T, ParseError<'a>>;

pub type Result<'a, T> = std::result::Result<T, SourceError<'a>>;

#[derive(Debug)]
pub enum SourceError<'a> {
    ClientError(client::ClientError),
    ParseError(ParseError<'a>),
}

impl<'a> Display for SourceError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SourceError::ClientError(ref e) => write!(f, "client_error: {e}"),
            SourceError::ParseError(ref e) => write!(f, "parsing_error: {:?}", e),
        }
    }
}

impl<'a> error::Error for SourceError<'a> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SourceError::ClientError(ref e) => e.source(),
            SourceError::ParseError(_) => None, // ToDo: Fix this later
        }
    }
}

impl<'a> From<client::ClientError> for SourceError<'a> {
    fn from(err: client::ClientError) -> Self {
        SourceError::ClientError(err)
    }
}

impl<'a> From<ParseError<'a>> for SourceError<'a> {
    fn from(err: ParseError<'a>) -> Self {
        SourceError::ParseError(err)
    }
}

pub mod client {
    use std::error;
    use std::fmt::Display;

    pub type ClientResult<T> = std::result::Result<T, ClientError>;

    #[derive(Debug)]
    pub enum ClientError {
        RequestError(reqwest::Error),
        RequestBodyError(reqwest::Error),
    }

    impl Display for ClientError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match *self {
                ClientError::RequestError(ref e) | ClientError::RequestBodyError(ref e) => {
                    write!(f, "{e}")
                }
            }
        }
    }

    impl error::Error for ClientError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match *self {
                ClientError::RequestError(ref e) | ClientError::RequestBodyError(ref e) => {
                    e.source()
                }
            }
        }
    }

    impl From<reqwest::Error> for ClientError {
        fn from(err: reqwest::Error) -> Self {
            if err.is_body() {
                return ClientError::RequestBodyError(err);
            }
            ClientError::RequestError(err)
        }
    }
}
