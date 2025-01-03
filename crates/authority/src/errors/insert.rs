use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum AuthorityInsertError {
    Overlapping { ids: Vec<usize> },
    UnknownConstraint { constraint: String },
}

impl Error for AuthorityInsertError {}

impl Display for AuthorityInsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overlapping { ids } => write!(f, r"overlapping authorities {ids:?}"),
            Self::UnknownConstraint { constraint } => write!(
                f,
                r"unknown authority constraint

   Constraint: {constraint}

The router doesn't recognize this constraint"
            ),
        }
    }
}
