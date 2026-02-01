use alloc::fmt;
use core::error::Error;

use crate::errors::TemplateError;

/// Errors relating to template deletion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeleteError {
    /// A [`TemplateError`] that occurred during the delete.
    Template(TemplateError),

    /// Template to be deleted was not found in the router.
    NotFound,
}

impl Error for DeleteError {}

impl fmt::Display for DeleteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Template(error) => error.fmt(f),
            Self::NotFound => write!(f, "not found"),
        }
    }
}

impl From<TemplateError> for DeleteError {
    fn from(error: TemplateError) -> Self {
        Self::Template(error)
    }
}
