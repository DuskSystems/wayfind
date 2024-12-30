use super::TemplateError;
use crate::errors::EncodingError;
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    Encoding(EncodingError),
    Template(TemplateError),
    NotFound { authority: String },
    Mismatch { authority: String, inserted: String },
}

impl Error for DeleteError {}

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encoding(error) => error.fmt(f),
            Self::Template(error) => error.fmt(f),
            Self::NotFound { authority } => write!(
                f,
                r"not found

   Authority: {authority}

The specified authority does not exist in the router"
            ),
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

impl From<EncodingError> for DeleteError {
    fn from(error: EncodingError) -> Self {
        Self::Encoding(error)
    }
}

impl From<TemplateError> for DeleteError {
    fn from(error: TemplateError) -> Self {
        Self::Template(error)
    }
}
