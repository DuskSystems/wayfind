use super::AuthorityTemplateError;
use crate::errors::EncodingError;
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum AuthorityDeleteError {
    EncodingError(EncodingError),
    TemplateError(AuthorityTemplateError),
    NotFound { authority: String },
    Mismatch { authority: String, inserted: String },
}

impl Error for AuthorityDeleteError {}

impl Display for AuthorityDeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EncodingError(error) => error.fmt(f),
            Self::TemplateError(error) => error.fmt(f),
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

impl From<EncodingError> for AuthorityDeleteError {
    fn from(error: EncodingError) -> Self {
        Self::EncodingError(error)
    }
}

impl From<AuthorityTemplateError> for AuthorityDeleteError {
    fn from(error: AuthorityTemplateError) -> Self {
        Self::TemplateError(error)
    }
}
