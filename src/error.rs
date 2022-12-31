use thiserror::Error;

#[derive(Error, Debug)]
pub enum TogglError {
    #[error("library error: {0}")]
    LibraryError(String),

    #[cfg(feature = "client")]
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[cfg(feature = "client")]
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
}

pub type Result<T> = std::result::Result<T, TogglError>;
