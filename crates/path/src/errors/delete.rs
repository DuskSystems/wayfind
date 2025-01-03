use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum PathDeleteError {
    Mismatch { path: String, inserted: String },
}

impl Error for PathDeleteError {}

impl Display for PathDeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mismatch { path, inserted } => write!(
                f,
                r"delete mismatch

        Path: {path}
    Inserted: {inserted}

The path must be deleted using the same format as was inserted"
            ),
        }
    }
}
