use super::EncodingError;
use core::{error::Error, fmt::Display};

/// Errors that can occur when creating a [`Request`](`crate::Request`).
#[derive(Debug, PartialEq, Eq)]
pub enum RequestError {
    /// A [`EncodingError`] that occurred during the decoding.
    Encoding(EncodingError),

    /// The path was not provided when building the [`Request`](`crate::Request`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RequestError;
    ///
    /// let error = RequestError::MissingPath;
    ///
    /// let display = "
    /// missing path
    ///
    /// A path must be provided when building a Request
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    MissingPath,
}

impl Error for RequestError {}

impl Display for RequestError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Encoding(error) => error.fmt(f),
            Self::MissingPath => write!(
                f,
                r"missing path

A path must be provided when building a Request"
            ),
        }
    }
}

impl From<EncodingError> for RequestError {
    fn from(error: EncodingError) -> Self {
        Self::Encoding(error)
    }
}
