use super::EncodingError;
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to search for a match in a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
    /// A [`EncodingError`] that occurred during the decoding.
    EncodingError(EncodingError),
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EncodingError(error) => error.fmt(f),
        }
    }
}

impl From<EncodingError> for SearchError {
    fn from(error: EncodingError) -> Self {
        Self::EncodingError(error)
    }
}
