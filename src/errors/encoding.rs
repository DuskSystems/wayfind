use alloc::string::String;
use core::{error::Error, fmt::Display};

/// Errors relating to attempting to decode UTF-8 and percent-encoded strings.
#[derive(Debug, PartialEq, Eq)]
pub enum EncodingError {
    /// Invalid UTF-8 sequence encountered.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::EncodingError;
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

    /// Invalid percent-encoding sequence encountered.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::EncodingError;
    ///
    /// let error = EncodingError::InvalidEncoding {
    ///     input: "/hello%GGworld".to_string(),
    ///     position: 6,
    ///     character: *b"%GG"
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

    /// The route provided was percent-encoded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::EncodingError;
    ///
    /// let error = EncodingError::EncodedRoute {
    ///     input: "/hello%20world".to_string(),
    ///     decoded: "/hello world".to_string(),
    /// };
    ///
    /// let display = "
    /// encoded route
    ///
    ///      Input: /hello%20world
    ///    Decoded: /hello world
    ///
    /// The router expects routes to be in their decoded form
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EncodedRoute {
        /// The original encoded input route.
        input: String,
        /// The decoded version of the route.
        decoded: String,
    },
}

impl Error for EncodingError {}

impl Display for EncodingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            Self::EncodedRoute { input, decoded } => write!(
                f,
                r#"encoded route

     Input: {input}
   Decoded: {decoded}

The router expects routes to be in their decoded form"#
            ),
        }
    }
}
