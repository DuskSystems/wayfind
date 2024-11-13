use super::EncodingError;
use core::{error::Error, fmt::Display};

/// Errors relating to attempting to search for a match in a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
    /// A [`EncodingError`] that occurred during the search.
    EncodingError(EncodingError),
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
