use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum PartsError {
    InvalidPath,
    InvalidRegex,
    RegexNotEnabled,
}

impl Error for PartsError {}

impl Display for PartsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "Invalid Path"),
            Self::InvalidRegex => write!(f, "Invalid Regex"),
            Self::RegexNotEnabled => write!(f, "Regex Not Enabled"),
        }
    }
}
