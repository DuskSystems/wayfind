use std::{error::Error, fmt::Display};

/// Errors relating to attempting to search for a match in a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
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
    /// Expected: valid UTF-8 encoded characters
    ///    Found: invalid byte sequence
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    Utf8Error {
        /// The parameter key.
        key: String,
        /// The invalid parameter value.
        /// This will contain UTF-8 replacement symbols.
        value: String,
    },
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Utf8Error { key, value } => {
                write!(
                    f,
                    "invalid UTF-8 sequence

     Key: {key}
   Value: {value}

Expected: valid UTF-8 encoded characters
   Found: invalid byte sequence",
                )
            }
        }
    }
}
