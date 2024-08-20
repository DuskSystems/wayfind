use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    InvalidEncoding {
        input: String,
        position: usize,
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
