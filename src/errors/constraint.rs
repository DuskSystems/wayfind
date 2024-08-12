use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum ConstraintError {
    DuplicateName,
}

impl Error for ConstraintError {}

impl Display for ConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateName => write!(f, "Duplicate Name"),
        }
    }
}
