use std::{error::Error, fmt::Display};

/// Errors relating to percent-decoding failures.
#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// Invalid percent-encoding sequence encountered.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::DecodeError;
    ///
    /// let error = DecodeError::InvalidEncoding {
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
}

impl Error for DecodeError {}

impl Display for DecodeError {
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
        }
    }
}
