use super::{MethodDeleteError, PathDeleteError};
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to delete a route from a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    Path(PathDeleteError),
    Method(MethodDeleteError),
    NotFound,
}

impl Error for DeleteError {}

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path(error) => error.fmt(f),
            Self::Method(error) => error.fmt(f),
            Self::NotFound => write!(f, "not found"),
        }
    }
}

impl From<PathDeleteError> for DeleteError {
    fn from(error: PathDeleteError) -> Self {
        Self::Path(error)
    }
}

impl From<MethodDeleteError> for DeleteError {
    fn from(error: MethodDeleteError) -> Self {
        Self::Method(error)
    }
}
