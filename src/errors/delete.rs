use super::PathDeleteError;
use core::{error::Error, fmt::Display};

/// Errors relating to attempting to delete a route from a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    /// A [`PathDeleteError`] occurred.
    Path(PathDeleteError),
}

impl Error for DeleteError {}

impl Display for DeleteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
