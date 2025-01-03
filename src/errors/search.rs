use std::{error::Error, fmt::Display, str::Utf8Error};

use wayfind_method::errors::MethodSearchError;

/// Errors relating to attempting to search for a match in a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
    Encoding(Utf8Error),
    Method(MethodSearchError),
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encoding(error) => error.fmt(f),
            Self::Method(error) => error.fmt(f),
        }
    }
}

impl From<Utf8Error> for SearchError {
    fn from(error: Utf8Error) -> Self {
        Self::Encoding(error)
    }
}

impl From<MethodSearchError> for SearchError {
    fn from(error: MethodSearchError) -> Self {
        Self::Method(error)
    }
}
