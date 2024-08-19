use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    InvalidEncoding,
}

impl Error for DecodeError {}

impl Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidEncoding => write!(f, "Invalid Encoding"),
        }
    }
}
