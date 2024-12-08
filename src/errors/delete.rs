use super::PathDeleteError;
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to delete a route from a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    /// A [`PathDeleteError`] occurred.
    Path(PathDeleteError),
}

impl Error for DeleteError {}

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path(error) => error.fmt(f),
        }
    }
}

impl From<PathDeleteError> for DeleteError {
    fn from(error: PathDeleteError) -> Self {
        Self::Path(error)
    }
}
