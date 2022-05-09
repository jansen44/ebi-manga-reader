use std::error;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, SourceError>;

#[derive(Debug)]
pub enum SourceError {
    InvalidSourceIdentifier,
    InvalidSourceData(String),
    ClientError(client::ClientError),
    ParserError(parser::ParserError),
}

impl Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SourceError::InvalidSourceIdentifier => write!(f, "source_error: Invalid source identifier"),
            SourceError::InvalidSourceData(ref e) => write!(f, "source_error: Invalid source data: {e}"),
            SourceError::ClientError(ref e) => write!(f, "{e}"),
            SourceError::ParserError(ref e) => write!(f, "{e}"),
        }
    }
}

impl error::Error for SourceError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SourceError::ClientError(ref e) => e.source(),
            _ => None,
        }
    }
}

impl From<client::ClientError> for SourceError {
    fn from(err: client::ClientError) -> Self {
        SourceError::ClientError(err)
    }
}

impl From<parser::ParserError> for SourceError {
    fn from(err: parser::ParserError) -> Self {
        SourceError::ParserError(err)
    }
}

pub mod client {
    use std::error;
    use std::fmt::Display;

    pub type ClientResult<T> = std::result::Result<T, ClientError>;

    #[derive(Debug)]
    pub enum ClientError {
        InvalidRequestBody(String),
        RequestError(reqwest::Error),
        RequestBodyError(reqwest::Error),
    }

    impl Display for ClientError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match *self {
                ClientError::InvalidRequestBody(ref message) => write!(f, "client_error: {message}"),
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
                _ => None,
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

pub mod parser {
    use std::num::ParseIntError;

    use cssparser::SourceLocation;
    use selectors::parser::SelectorParseErrorKind;

    pub type ParserResult<T> = std::result::Result<T, ParserError>;

    #[derive(Debug)]
    pub enum ParserError {
        Other(String),
        ParsingError(String, SourceLocation),
        MissingElement(String),
        FailedTypeConversion(String),
        InvalidParsingTarget(String),
    }

    impl std::fmt::Display for ParserError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match *self {
                ParserError::Other(ref e) => write!(f, "parser_error: {e}"),
                ParserError::ParsingError(ref e, location) => write!(
                    f,
                    "error parsing at position ({}, {}): {e}",
                    location.line, location.column
                ),
                ParserError::MissingElement(ref e) => write!(f, "parser_error: {e}"),
                ParserError::FailedTypeConversion(ref e) => write!(f, "parser_error: {e}"),
                ParserError::InvalidParsingTarget(ref e) => write!(f, "parser_error: {e}"),
            }
        }
    }

    impl<'a> From<cssparser::ParseError<'a, SelectorParseErrorKind<'a>>> for ParserError {
        fn from(err: cssparser::ParseError<'a, SelectorParseErrorKind>) -> Self {
            let location = err.location.clone();
            let err = format!("{:?}", err.kind);
            ParserError::ParsingError(err, location)
        }
    }

    impl From<ParseIntError> for ParserError {
        fn from(err: ParseIntError) -> Self {
            ParserError::FailedTypeConversion(format!("{:?}", err.kind()))
        }
    }

    impl From<serde_json::error::Error> for ParserError {
        fn from(err: serde_json::error::Error) -> Self {
            let classification = err.classify();
            let line = err.line();
            let column = err.column();

            let message = format!("({}, {}) - {:?}", line, column, classification);

            ParserError::FailedTypeConversion(message)
        }
    }
}
