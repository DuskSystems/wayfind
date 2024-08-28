use std::{error::Error, fmt::Display};

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

    /// Invalid UTF-8 sequence encountered.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathError;
    ///
    /// let error = PathError::Utf8Error {
    ///     input: "/hello%FFworld".to_string(),
    ///     decoded: "/hello�world".to_string(),
    ///     position: 6,
    ///     length: 1,
    /// };
    ///
    /// let display = "
    /// invalid UTF-8 sequence
    ///
    /// Original: /hello%FFworld
    ///  Decoded: /hello�world
    ///                 ^
    ///
    /// Expected: valid UTF-8 encoded characters
    ///    Found: invalid byte sequence at position 6 after decoding
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    Utf8Error {
        /// The unaltered input string.
        input: String,
        /// The post-decoded input string.
        /// This will contain UTF-8 replacement symbols.
        decoded: String,
        /// The position in the decoded string where the invalid UTF-8 was found.
        position: usize,
        /// The length of the invalid UTF-8 sequence.
        length: usize,
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
                    "invalid percent-encoding

   Input: {input}
          {arrow}

Expected: '%' followed by two hexadecimal digits (a-F, 0-9)
   Found: '{character}'",
                )
            }
            Self::Utf8Error {
                input,
                decoded,
                position,
                length,
            } => {
                let arrow = " ".repeat(*position) + &"^".repeat(*length);
                write!(
                    f,
                    "invalid UTF-8 sequence

Original: {input}
 Decoded: {decoded}
          {arrow}

Expected: valid UTF-8 encoded characters
   Found: invalid byte sequence at position {position} after decoding",
                )
            }
        }
    }
}
