use super::TemplateError;
use crate::AuthorityId;
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    TemplateError(TemplateError),
    Overlapping { ids: Vec<AuthorityId> },
    UnknownConstraint { constraint: String },
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TemplateError(error) => error.fmt(f),
            Self::Overlapping { ids } => write!(f, r"overlapping authorities {ids:?}"),
            Self::UnknownConstraint { constraint } => write!(
                f,
                r"unknown constraint

   Constraint: {constraint}

The router doesn't recognize this constraint"
            ),
        }
    }
}

impl From<TemplateError> for InsertError {
    fn from(error: TemplateError) -> Self {
        Self::TemplateError(error)
    }
}
