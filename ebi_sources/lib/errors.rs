use std::error;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, SourceError>;

#[derive(Debug)]
pub enum SourceError {
    ClientError(client::ClientError),
}

impl Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SourceError::ClientError(ref e) => write!(f, "client_error: {e}"),
        }
    }
}

impl error::Error for SourceError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SourceError::ClientError(ref e) => e.source(),
        }
    }
}

impl From<client::ClientError> for SourceError {
    fn from(err: client::ClientError) -> Self {
        SourceError::ClientError(err)
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
