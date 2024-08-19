use super::route::RouteError;
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    RouteError(RouteError),
    EncodedPath,
    DuplicatePath,
    UnknownConstraint,
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RouteError(error) => error.fmt(f),
            Self::EncodedPath => write!(f, "Encoded Path"),
            Self::DuplicatePath => write!(f, "Duplicate Path"),
            Self::UnknownConstraint => write!(f, "Unknown Constraint"),
        }
    }
}

impl From<RouteError> for InsertError {
    fn from(error: RouteError) -> Self {
        Self::RouteError(error)
    }
}
