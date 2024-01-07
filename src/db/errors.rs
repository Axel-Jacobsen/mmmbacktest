use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RowParsingError {
    Generic(String),
    RusqliteError(rusqlite::Error),
    SerdeError(serde_json::Error),
}

impl fmt::Display for RowParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RowParsingError::Generic(e) => write!(f, "APIGeneric error: {}", e),
            RowParsingError::RusqliteError(e) => write!(f, "Rusqlite error: {}", e),
            RowParsingError::SerdeError(e) => write!(f, "Serde JSON error: {}", e),
        }
    }
}

impl Error for RowParsingError {}

impl From<String> for RowParsingError {
    fn from(error: String) -> Self {
        RowParsingError::Generic(error)
    }
}

impl From<rusqlite::Error> for RowParsingError {
    fn from(error: rusqlite::Error) -> Self {
        RowParsingError::RusqliteError(error)
    }
}

impl From<serde_json::Error> for RowParsingError {
    fn from(error: serde_json::Error) -> Self {
        RowParsingError::SerdeError(error)
    }
}
