use super::decode::DecodeError;
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
    DecodeError(DecodeError),
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DecodeError(error) => error.fmt(f),
        }
    }
}

impl From<DecodeError> for SearchError {
    fn from(error: DecodeError) -> Self {
        Self::DecodeError(error)
    }
}
