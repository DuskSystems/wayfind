use super::EncodingError;
use core::{error::Error, fmt::Display};

/// Errors relating to attempting to decode and validate path.
#[derive(Debug, PartialEq, Eq)]
pub enum PathError {
    /// A [`EncodingError`] that occurred during the decoding.
    EncodingError(EncodingError),
}

impl Error for PathError {}

impl Display for PathError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EncodingError(error) => error.fmt(f),
        }
    }
}

impl From<EncodingError> for PathError {
    fn from(error: EncodingError) -> Self {
        Self::EncodingError(error)
    }
}
