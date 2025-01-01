use std::{error::Error, fmt::Display};
use wayfind_authority::errors::AuthorityDeleteError;
use wayfind_method::errors::MethodDeleteError;
use wayfind_path::errors::PathDeleteError;

/// Errors relating to attempting to delete a route from a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    Authority(AuthorityDeleteError),
    Path(PathDeleteError),
    Method(MethodDeleteError),
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

impl From<AuthorityDeleteError> for DeleteError {
    fn from(error: AuthorityDeleteError) -> Self {
        Self::Authority(error)
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
