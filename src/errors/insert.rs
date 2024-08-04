use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    InvalidPath,
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "Invalid Path"),
        }
    }
}
