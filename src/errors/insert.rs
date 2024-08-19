use super::{decode::DecodeError, route::RouteError};
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    RouteError(RouteError),
    DecodeError(DecodeError),
    EncodedPath,
    DuplicatePath,
    UnknownConstraint,
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RouteError(error) => error.fmt(f),
            Self::DecodeError(error) => error.fmt(f),
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

impl From<DecodeError> for InsertError {
    fn from(error: DecodeError) -> Self {
        Self::DecodeError(error)
    }
}
