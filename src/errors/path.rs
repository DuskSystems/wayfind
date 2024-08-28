use super::DecodeError;
use std::{error::Error, fmt::Display, str::Utf8Error};

/// Errors relating to attempting to insert a route into a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum PathError {
    // TODO: Consider collapsing decode error in this, if we never need to use it elsewhere.
    /// A [`DecodeError`] that occurred during the path creation.
    DecodeError(DecodeError),

    // TODO: Consider having a custom error here.
    /// A [`Utf8Error`] that occurred during the path creation.
    Utf8Error {
        valid_up_to: usize,
        error_len: Option<usize>,
    },
}

impl Error for PathError {}

impl Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DecodeError(error) => error.fmt(f),
            Self::Utf8Error { .. } => todo!(),
        }
    }
}

impl From<DecodeError> for PathError {
    fn from(error: DecodeError) -> Self {
        Self::DecodeError(error)
    }
}

impl From<Utf8Error> for PathError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8Error {
            valid_up_to: error.valid_up_to(),
            error_len: error.error_len(),
        }
    }
}
