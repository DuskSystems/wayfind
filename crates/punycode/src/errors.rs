use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum PunycodeDecodingError {
    /// Invalid basic code point encountered (non-ASCII character).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_punycode::errors::PunycodeDecodingError;
    ///
    /// let error = PunycodeDecodingError::InvalidBasicCodePoint {
    ///     input: "hello²world".to_string(),
    ///     position: 5,
    ///     character: vec![0xC2, 0xB2],
    /// };
    ///
    /// let display = "
    /// invalid basic code point
    ///
    ///    Input: hello²world
    ///                ^
    ///
    /// Expected: ASCII character (0-127)
    ///    Found: '²'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidBasicCodePoint {
        /// The unaltered input string.
        input: String,
        /// The position in the input where the invalid code point was found.
        position: usize,
        /// The invalid character sequence.
        character: Vec<u8>,
    },

    /// Invalid digit encountered during Punycode decoding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_punycode::errors::PunycodeDecodingError;
    ///
    /// let error = PunycodeDecodingError::InvalidDigit {
    ///     input: "hello-@world".to_string(),
    ///     position: 6,
    ///     character: b'@',
    /// };
    ///
    /// let display = "
    /// invalid punycode digit
    ///
    ///    Input: hello-@world
    ///                 ^
    ///
    /// Expected: letter (a-z, A-Z) or digit (0-9)
    ///    Found: '@'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidDigit {
        /// The unaltered input string.
        input: String,
        /// The position in the input where the invalid digit was found.
        position: usize,
        /// The invalid character.
        character: u8,
    },

    /// Unexpected end of input during Punycode decoding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_punycode::errors::PunycodeDecodingError;
    ///
    /// let error = PunycodeDecodingError::UnexpectedEnd {
    ///     input: "hello-a".to_string(),
    ///     position: 7,
    /// };
    ///
    /// let display = "
    /// unexpected end of input
    ///
    ///    Input: hello-a
    ///                  ^
    ///
    /// Expected: more punycode digits
    ///    Found: end of input
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    UnexpectedEnd {
        /// The unaltered input string.
        input: String,
        /// The position where the input ended unexpectedly.
        position: usize,
    },

    /// Numeric overflow during Punycode decoding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_punycode::errors::PunycodeDecodingError;
    ///
    /// let error = PunycodeDecodingError::Overflow {
    ///     input: "hello-9999999999a".to_string(),
    ///     position: 7,
    /// };
    ///
    /// let display = "
    /// numeric overflow
    ///
    ///    Input: hello-9999999999a
    ///                  ^
    ///
    /// Overflow occurred while decoding punycode digits
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    Overflow {
        /// The unaltered input string.
        input: String,
        /// The position where the overflow occurred.
        position: usize,
    },

    /// Invalid Unicode code point generated during decoding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_punycode::errors::PunycodeDecodingError;
    ///
    /// let error = PunycodeDecodingError::InvalidCodePoint {
    ///     input: "hello-99999a".to_string(),
    ///     position: 11,
    ///     value: 0x0048A841,
    /// };
    ///
    /// let display = "
    /// invalid code point
    ///
    ///    Input: hello-99999a
    ///                      ^
    ///
    /// Cannot convert value 4761665 to valid Unicode character
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidCodePoint {
        /// The unaltered input string.
        input: String,
        /// The position where the invalid code point was generated.
        position: usize,
        /// The invalid code point value.
        value: u32,
    },
}

impl Error for PunycodeDecodingError {}

impl Display for PunycodeDecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBasicCodePoint {
                input,
                position,
                character,
            } => {
                let character = String::from_utf8_lossy(character);
                let arrow = " ".repeat(*position) + "^";

                write!(
                    f,
                    "invalid basic code point

   Input: {input}
          {arrow}

Expected: ASCII character (0-127)
   Found: '{character}'"
                )
            }
            Self::InvalidDigit {
                input,
                position,
                character,
            } => {
                let arrow = " ".repeat(*position) + "^";
                let char = *character as char;

                write!(
                    f,
                    "invalid punycode digit

   Input: {input}
          {arrow}

Expected: letter (a-z, A-Z) or digit (0-9)
   Found: '{char}'"
                )
            }
            Self::UnexpectedEnd { input, position } => {
                let arrow = " ".repeat(*position) + "^";

                write!(
                    f,
                    "unexpected end of input

   Input: {input}
          {arrow}

Expected: more punycode digits
   Found: end of input"
                )
            }
            Self::Overflow { input, position } => {
                let arrow = " ".repeat(*position) + "^";

                write!(
                    f,
                    "numeric overflow

   Input: {input}
          {arrow}

Overflow occurred while decoding punycode digits"
                )
            }
            Self::InvalidCodePoint {
                input,
                position,
                value,
            } => {
                let arrow = " ".repeat(*position) + "^";

                write!(
                    f,
                    "invalid code point

   Input: {input}
          {arrow}

Cannot convert value {value} to valid Unicode character"
                )
            }
        }
    }
}
