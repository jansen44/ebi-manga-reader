use std::error;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, SourceError>;

#[derive(Debug)]
pub enum SourceError {
    ClientError(client::ClientError),
    ParserError(parser::ParserError),
}

impl Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SourceError::ClientError(ref e) => write!(f, "client_error: {e}"),
            SourceError::ParserError(ref e) => write!(f, "parsing_error: {:?}", e),
        }
    }
}

impl error::Error for SourceError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SourceError::ClientError(ref e) => e.source(),
            SourceError::ParserError(_) => None,
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
