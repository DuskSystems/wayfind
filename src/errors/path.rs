use std::{error::Error, fmt::Display, str::Utf8Error};

/// Errors relating to attempting to decode and validate  path.
#[derive(Debug, PartialEq, Eq)]
pub enum PathError {
    /// Invalid percent-encoding sequence encountered.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathError;
    ///
    /// let error = PathError::InvalidEncoding {
    ///     input: "/hello%GGworld".to_string(),
    ///     position: 6,
    ///     character: [b'%', b'G', b'G'],
    /// };
    ///
    /// let display = "
    /// invalid percent-encoding
    ///
    ///    Input: /hello%GGworld
    ///                 ^^^
    ///
    /// Expected: '%' followed by two hexadecimal digits (a-F, 0-9)
    ///    Found: '%GG'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidEncoding {
        /// The unaltered input string.
        input: String,
        /// The position in the input where the invalid encoding was found.
        position: usize,
        /// The invalid character sequence.
        character: [u8; 3],
    },

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
            Self::InvalidEncoding {
                input,
                position,
                character,
            } => {
                let character = String::from_utf8_lossy(character);
                let arrow = " ".repeat(*position) + "^^^";

                write!(
                    f,
                    r#"invalid percent-encoding

   Input: {input}
          {arrow}

Expected: '%' followed by two hexadecimal digits (a-F, 0-9)
   Found: '{character}'"#,
                )
            }
            Self::Utf8Error { .. } => todo!(),
        }
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
