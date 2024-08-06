use super::parts::PartsError;
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    PartsError(PartsError),
    NotFound,
}

impl Error for DeleteError {}

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PartsError(error) => error.fmt(f),
            Self::NotFound => write!(f, "Path Not Found"),
        }
    }
}

impl From<PartsError> for DeleteError {
    fn from(error: PartsError) -> Self {
        Self::PartsError(error)
    }
}
