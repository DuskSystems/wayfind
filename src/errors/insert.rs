use crate::routers::path::errors::PathInsertError;
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to insert a route into a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    /// A [`PathInsertError`] occurred.
    Path(PathInsertError),
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path(error) => error.fmt(f),
        }
    }
}

impl From<PathInsertError> for InsertError {
    fn from(error: PathInsertError) -> Self {
        Self::Path(error)
    }
}
