use crate::chain::DataChain;
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to insert a route into a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    Authority(wayfind_authority::errors::InsertError),
    Path(wayfind_path::errors::InsertError),
    Method(wayfind_method::errors::InsertError),
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

impl From<wayfind_authority::errors::InsertError> for InsertError {
    fn from(error: wayfind_authority::errors::InsertError) -> Self {
        Self::Authority(error)
    }
}

impl From<wayfind_path::errors::InsertError> for InsertError {
    fn from(error: wayfind_path::errors::InsertError) -> Self {
        Self::Path(error)
    }
}

impl From<wayfind_method::errors::InsertError> for InsertError {
    fn from(error: wayfind_method::errors::InsertError) -> Self {
        Self::Method(error)
    }
}
