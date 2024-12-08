use super::PathSearchError;
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to search for a match in a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
    /// A [`PathSearchError`] occurred.
    Path(PathSearchError),
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path(error) => error.fmt(f),
        }
    }
}

impl From<PathSearchError> for SearchError {
    fn from(error: PathSearchError) -> Self {
        Self::Path(error)
    }
}
