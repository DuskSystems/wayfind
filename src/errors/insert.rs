use core::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    InvalidPath,
    InvalidRegex,
    RegexNotEnabled,
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "Invalid Path"),
            Self::InvalidRegex => write!(f, "Invalid Regex"),
            Self::RegexNotEnabled => write!(f, "Regex Not Enabled"),
        }
    }
}
