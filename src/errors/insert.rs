use crate::routers::path::errors::PathInsertError;
use core::{error::Error, fmt::Display};

/// Errors relating to attempting to insert a route into a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    /// A [`PathInsertError`] occurred.
    PathInsertError(PathInsertError),
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PathInsertError(error) => error.fmt(f),
        }
    }
}

impl From<PathInsertError> for InsertError {
    fn from(error: PathInsertError) -> Self {
        Self::PathInsertError(error)
    }
}
