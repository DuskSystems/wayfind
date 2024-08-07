use super::route::RouteError;
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    RouteError(RouteError),
    DuplicatePath,
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RouteError(error) => error.fmt(f),
            Self::DuplicatePath => write!(f, "Duplicate Path"),
        }
    }
}

impl From<RouteError> for InsertError {
    fn from(error: RouteError) -> Self {
        Self::RouteError(error)
    }
}
