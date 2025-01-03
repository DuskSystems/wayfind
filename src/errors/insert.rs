use crate::chain::DataChain;
use std::{error::Error, fmt::Display};
use wayfind_authority::errors::AuthorityInsertError;
use wayfind_method::errors::MethodInsertError;
use wayfind_path::errors::PathInsertError;

/// Errors relating to attempting to insert a route into a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    Authority(AuthorityInsertError),
    Path(PathInsertError),
    Method(MethodInsertError),
    Conflict { chain: DataChain },
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authority(error) => error.fmt(f),
            Self::Path(error) => error.fmt(f),
            Self::Method(error) => error.fmt(f),
            Self::Conflict { chain } => write!(
                f,
                r"chain conflict

    Chain: {chain:?}"
            ),
        }
    }
}

impl From<AuthorityInsertError> for InsertError {
    fn from(error: AuthorityInsertError) -> Self {
        Self::Authority(error)
    }
}

impl From<PathInsertError> for InsertError {
    fn from(error: PathInsertError) -> Self {
        Self::Path(error)
    }
}

impl From<MethodInsertError> for InsertError {
    fn from(error: MethodInsertError) -> Self {
        Self::Method(error)
    }
}
