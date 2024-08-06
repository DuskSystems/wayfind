use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum PartsError {
    InvalidPath,
}

impl Error for PartsError {}

impl Display for PartsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "Invalid Path"),
        }
    }
}
