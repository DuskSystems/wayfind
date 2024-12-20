use crate::errors::EncodingError;
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to search for a match in a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum PathSearchError {
    /// A [`EncodingError`] that occurred during the search.
    EncodingError(EncodingError),
}

impl Error for PathSearchError {}

impl Display for PathSearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EncodingError(error) => error.fmt(f),
        }
    }
}

impl From<EncodingError> for PathSearchError {
    fn from(error: EncodingError) -> Self {
        Self::EncodingError(error)
    }
}
