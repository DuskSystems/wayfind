use super::parts::PartsError;
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    PartsError(PartsError),
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PartsError(error) => error.fmt(f),
        }
    }
}

impl From<PartsError> for InsertError {
    fn from(error: PartsError) -> Self {
        Self::PartsError(error)
    }
}
