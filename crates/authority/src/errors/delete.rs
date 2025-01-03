use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum AuthorityDeleteError {
    Mismatch { authority: String, inserted: String },
}

impl Error for AuthorityDeleteError {}

impl Display for AuthorityDeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mismatch {
                authority,
                inserted,
            } => write!(
                f,
                r"delete mismatch

   Authority: {authority}
    Inserted: {inserted}

The authority must be deleted using the same format as was inserted"
            ),
        }
    }
}
