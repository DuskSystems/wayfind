use crate::errors::EncodingError;
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to search for a match in a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum AuthoritySearchError {
    /// A [`EncodingError`] that occurred during the search.
    EncodingError(EncodingError),
}

impl Error for AuthoritySearchError {}

impl Display for AuthoritySearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EncodingError(error) => error.fmt(f),
        }
    }
}

impl From<EncodingError> for AuthoritySearchError {
    fn from(error: EncodingError) -> Self {
        Self::EncodingError(error)
    }
}
