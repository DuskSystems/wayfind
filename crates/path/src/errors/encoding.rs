use std::{error::Error, fmt::Display};

/// Errors relating to attempting to decode strings.
#[derive(Debug, PartialEq, Eq)]
pub enum EncodingError {
    /// Invalid UTF-8 sequence encountered.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_path::errors::EncodingError;
    ///
    /// let error = EncodingError::Utf8Error {
    ///     input: "hello�world".to_string(),
    /// };
    ///
    /// let display = "
    /// invalid UTF-8 sequence
    ///
    ///    Input: hello�world
    ///
    /// Expected: valid UTF-8 characters
    ///    Found: invalid byte sequence
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    Utf8Error {
        /// The invalid input.
        /// This will contain UTF-8 replacement symbols.
        input: String,
    },
}

impl Error for EncodingError {}

impl Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Utf8Error { input } => {
                write!(
                    f,
                    "invalid UTF-8 sequence

   Input: {input}

Expected: valid UTF-8 characters
   Found: invalid byte sequence",
                )
            }
        }
    }
}
