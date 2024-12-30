use std::{error::Error, fmt::Display};

/// Errors relating to attempting to delete a route from a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    Authority(wayfind_authority::errors::DeleteError),
    Path(wayfind_path::errors::DeleteError),
    Method(wayfind_method::errors::DeleteError),
    NotFound,
}

impl Error for DeleteError {}

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authority(error) => error.fmt(f),
            Self::Path(error) => error.fmt(f),
            Self::Method(error) => error.fmt(f),
            Self::NotFound => write!(f, "not found"),
        }
    }
}

impl From<wayfind_authority::errors::DeleteError> for DeleteError {
    fn from(error: wayfind_authority::errors::DeleteError) -> Self {
        Self::Authority(error)
    }
}

impl From<wayfind_path::errors::DeleteError> for DeleteError {
    fn from(error: wayfind_path::errors::DeleteError) -> Self {
        Self::Path(error)
    }
}

impl From<wayfind_method::errors::DeleteError> for DeleteError {
    fn from(error: wayfind_method::errors::DeleteError) -> Self {
        Self::Method(error)
    }
}
