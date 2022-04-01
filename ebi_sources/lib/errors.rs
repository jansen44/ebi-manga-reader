use std::error;
use std::fmt::Display;

pub type Result<'a, T> = std::result::Result<T, SourceError<'a>>;

#[derive(Debug)]
pub enum SourceError<'a> {
    ClientError(client::ClientError),
    ParserError(parser::ParserError<'a>),
}

impl<'a> Display for SourceError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SourceError::ClientError(ref e) => write!(f, "client_error: {e}"),
            SourceError::ParserError(ref e) => write!(f, "parsing_error: {:?}", e),
        }
    }
}

impl<'a> error::Error for SourceError<'a> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SourceError::ClientError(ref e) => e.source(),
            SourceError::ParserError(_) => None,
        }
    }
}

impl<'a> From<client::ClientError> for SourceError<'a> {
    fn from(err: client::ClientError) -> Self {
        SourceError::ClientError(err)
    }
}

impl<'a> From<parser::ParserError<'a>> for SourceError<'a> {
    fn from(err: parser::ParserError<'a>) -> Self {
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

    use selectors::parser::SelectorParseErrorKind;

    pub type ParserResult<'a, T> = std::result::Result<T, ParserError<'a>>;

    #[derive(Debug)]
    pub enum ParserError<'a> {
        Other(&'a str),
        ParsingError(cssparser::ParseError<'a, SelectorParseErrorKind<'a>>),
        MissingElement(&'a str),
        FailedTypeConversion(String),
    }

    impl<'a> From<cssparser::ParseError<'a, SelectorParseErrorKind<'a>>> for ParserError<'a> {
        fn from(err: cssparser::ParseError<'a, SelectorParseErrorKind<'a>>) -> Self {
            ParserError::ParsingError(err)
        }
    }

    impl<'a> From<ParseIntError> for ParserError<'a> {
        fn from(err: ParseIntError) -> Self {
            ParserError::FailedTypeConversion(format!("{:?}", err.kind()))
        }
    }

    impl<'a> From<serde_json::error::Error> for ParserError<'a> {
        fn from(err: serde_json::error::Error) -> Self {
            let classification = err.classify();
            let line = err.line();
            let column = err.column();

            let message = format!("({}, {}) - {:?}", line, column, classification);

            ParserError::FailedTypeConversion(message)
        }
    }
}
