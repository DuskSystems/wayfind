use std::{error::Error, fmt::Display};

use super::EncodingError;

/// Errors relating to attempting to search for a path in a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
    /// A [`EncodingError`] that occurred during the search.
    Encoding(EncodingError),
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encoding(error) => error.fmt(f),
        }
    }
}

impl From<EncodingError> for SearchError {
    fn from(error: EncodingError) -> Self {
        Self::Encoding(error)
    }
}
