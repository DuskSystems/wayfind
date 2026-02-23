use alloc::string::String;
use core::error::Error;
use core::fmt;

use crate::errors::TemplateError;

/// Errors relating to template insertion.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum InsertError {
    /// A [`TemplateError`] that occurred during the insert.
    Template(TemplateError),

    /// A conflicting template already exists in the router.
    Conflict {
        /// The existing template that conflicts.
        existing: String,
    },
}

impl Error for InsertError {}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Template(error) => error.fmt(f),
            Self::Conflict { existing } => write!(f, "conflict with `{existing}`"),
        }
    }
}

impl From<TemplateError> for InsertError {
    fn from(error: TemplateError) -> Self {
        Self::Template(error)
    }
}
