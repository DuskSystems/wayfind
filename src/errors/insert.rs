use alloc::string::String;
use core::error::Error;
use core::fmt;

use crate::errors::TemplateError;

/// Errors relating to template insertion.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum InsertError {
    /// A [`TemplateError`] that occurred during the insert.
    Template {
        /// The template that caused the error.
        template: String,
        /// The underlying template error.
        error: TemplateError,
    },

    /// A conflicting template already exists in the router.
    Conflict {
        /// The new template being inserted.
        new: String,
        /// The existing template that conflicts.
        existing: String,
    },
}

impl Error for InsertError {}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Template { template, error } => {
                write!(f, "invalid template `{template}`: {error}")
            }
            Self::Conflict { new, existing } => {
                write!(f, "`{new}` conflicts with `{existing}`")
            }
        }
    }
}
