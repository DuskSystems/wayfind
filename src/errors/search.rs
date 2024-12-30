use std::{error::Error, fmt::Display};

/// Errors relating to attempting to search for a match in a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
    Authority(wayfind_authority::errors::SearchError),
    Path(wayfind_path::errors::SearchError),
    Method(wayfind_method::errors::SearchError),
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

impl From<wayfind_authority::errors::SearchError> for SearchError {
    fn from(error: wayfind_authority::errors::SearchError) -> Self {
        Self::Authority(error)
    }
}

impl From<wayfind_path::errors::SearchError> for SearchError {
    fn from(error: wayfind_path::errors::SearchError) -> Self {
        Self::Path(error)
    }
}

impl From<wayfind_method::errors::SearchError> for SearchError {
    fn from(error: wayfind_method::errors::SearchError) -> Self {
        Self::Method(error)
    }
}
