use super::PathSearchError;
use core::{error::Error, fmt::Display};

/// Errors relating to attempting to search for a match in a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
    /// A [`PathSearchError`] occurred.
    PathSearchError(PathSearchError),
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PathSearchError(error) => error.fmt(f),
        }
    }
}

impl From<PathSearchError> for SearchError {
    fn from(error: PathSearchError) -> Self {
        Self::PathSearchError(error)
    }
}
