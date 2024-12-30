use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum DecodingError {
    /// Invalid percent-encoding character encountered.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_percent::errors::DecodingError;
    ///
    /// let error = DecodingError::InvalidCharacter {
    ///     input: "/hello%GGworld".to_string(),
    ///     position: 6,
    ///     character: vec![b'%', b'G', b'G'],
    /// };
    ///
    /// let display = "
    /// invalid character
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
    InvalidCharacter {
        /// The unaltered input string.
        input: String,
        /// The position in the input where the invalid encoding was found.
        position: usize,
        /// The invalid character sequence.
        character: Vec<u8>,
    },
}

impl Error for DecodingError {}

impl Display for DecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCharacter {
                input,
                position,
                character,
            } => {
                let character = String::from_utf8_lossy(character);
                let arrow = " ".repeat(*position) + &"^".repeat(character.len());

                write!(
                    f,
                    "invalid character

   Input: {input}
          {arrow}

Expected: '%' followed by two hexadecimal digits (a-F, 0-9)
   Found: '{character}'",
                )
            }
        }
    }
}
