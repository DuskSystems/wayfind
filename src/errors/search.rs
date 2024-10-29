use super::EncodingError;
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to search for a match in a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
    /// A [`EncodingError`] that occurred during the search.
    EncodingError(EncodingError),

    /// Invalid UTF-8 sequence encountered.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::SearchError;
    ///
    /// let error = SearchError::Utf8Error {
    ///     key: "parameter".to_string(),
    ///     value: "hello�world".to_string(),
    /// };
    ///
    /// let display = "
    /// invalid UTF-8 sequence
    ///
    ///      Key: parameter
    ///    Value: hello�world
    ///
    /// Expected: valid UTF-8 characters
    ///    Found: invalid byte sequence
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    Utf8Error {
        /// The parameter key.
        /// This may contain UTF-8 replacement symbols.
        key: String,
        /// The parameter value.
        /// This may contain UTF-8 replacement symbols.
        value: String,
    },
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EncodingError(error) => error.fmt(f),
            Self::Utf8Error { key, value } => {
                write!(
                    f,
                    "invalid UTF-8 sequence

     Key: {key}
   Value: {value}

Expected: valid UTF-8 characters
   Found: invalid byte sequence",
                )
            }
        }
    }
}

impl From<EncodingError> for SearchError {
    fn from(error: EncodingError) -> Self {
        Self::EncodingError(error)
    }
}
