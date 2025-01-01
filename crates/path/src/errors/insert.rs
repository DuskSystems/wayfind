use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum PathInsertError {
    Overlapping { ids: Vec<usize> },
    UnknownConstraint { constraint: String },
}

impl Error for PathInsertError {}

impl Display for PathInsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overlapping { ids } => write!(f, r"overlapping authorities {ids:?}"),
            Self::UnknownConstraint { constraint } => write!(
                f,
                r"unknown path constraint

   Constraint: {constraint}

The router doesn't recognize this constraint"
            ),
        }
    }
}
