use std::{error::Error, fmt::Display, str::Utf8Error};

#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
    Utf8Error(Utf8Error),
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Utf8Error(error) => error.fmt(f),
        }
    }
}

impl From<Utf8Error> for SearchError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8Error(error)
    }
}
