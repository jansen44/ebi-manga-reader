use std::error;
use std::fmt::Display;

pub type Result<T, K = ArchiveError> = std::result::Result<T, K>;

#[derive(Debug)]
pub enum ArchiveError {
    SourceError(ebi_sources::errors::SourceError),
    DownloadError(download::DownloadError),

    RequestError(reqwest::Error),
    RequestBodyError(reqwest::Error),
}

impl Display for ArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ArchiveError::RequestError(ref e) | ArchiveError::RequestBodyError(ref e) => {
                write!(f, "download_error: {e}")
            }
            ArchiveError::DownloadError(ref e) => write!(f, "download_error: {e}"),
            ArchiveError::SourceError(ref e) => write!(f, "source_error: {:?}", e),
        }
    }
}

impl error::Error for ArchiveError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ArchiveError::RequestError(ref e) | ArchiveError::RequestBodyError(ref e) => e.source(),
            ArchiveError::DownloadError(ref e) => e.source(),
            ArchiveError::SourceError(ref e) => e.source(),
        }
    }
}

impl From<ebi_sources::errors::SourceError> for ArchiveError {
    fn from(err: ebi_sources::errors::SourceError) -> Self {
        ArchiveError::SourceError(err)
    }
}

impl From<reqwest::Error> for ArchiveError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_body() {
            return ArchiveError::RequestBodyError(err);
        }
        ArchiveError::RequestError(err)
    }
}

pub mod download {
    use std::error;
    use std::fmt::Display;

    pub type ClientResult<T> = std::result::Result<T, DownloadError>;

    #[derive(Debug)]
    pub enum DownloadError {
        RequestError(reqwest::Error),
        RequestBodyError(reqwest::Error),
        GenericError(String),
    }

    impl Display for DownloadError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match *self {
                DownloadError::GenericError(ref e) => write!(f, "{e}"),
                DownloadError::RequestError(ref e) | DownloadError::RequestBodyError(ref e) => {
                    write!(f, "{e}")
                }
            }
        }
    }

    impl error::Error for DownloadError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match *self {
                DownloadError::GenericError(ref e) => Err(e).ok(),
                DownloadError::RequestError(ref e) | DownloadError::RequestBodyError(ref e) => {
                    e.source()
                }
            }
        }
    }

    impl From<reqwest::Error> for DownloadError {
        fn from(err: reqwest::Error) -> Self {
            if err.is_body() {
                return DownloadError::RequestBodyError(err);
            }
            DownloadError::RequestError(err)
        }
    }
}
