use super::{AuthoritySearchError, MethodSearchError, PathSearchError};
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to search for a match in a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
    Authority(AuthoritySearchError),
    Path(PathSearchError),
    Method(MethodSearchError),
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authority(error) => error.fmt(f),
            Self::Path(error) => error.fmt(f),
            Self::Method(error) => error.fmt(f),
        }
    }
}

impl From<AuthoritySearchError> for SearchError {
    fn from(error: AuthoritySearchError) -> Self {
        Self::Authority(error)
    }
}

impl From<PathSearchError> for SearchError {
    fn from(error: PathSearchError) -> Self {
        Self::Path(error)
    }
}

impl From<MethodSearchError> for SearchError {
    fn from(error: MethodSearchError) -> Self {
        Self::Method(error)
    }
}
