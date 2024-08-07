use super::route::RouteError;
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    RouteError(RouteError),
    NotFound,
}

impl Error for DeleteError {}

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RouteError(error) => error.fmt(f),
            Self::NotFound => write!(f, "Path Not Found"),
        }
    }
}

impl From<RouteError> for DeleteError {
    fn from(error: RouteError) -> Self {
        Self::RouteError(error)
    }
}
