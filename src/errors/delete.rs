use alloc::string::String;
use core::error::Error;
use core::fmt;

use crate::errors::TemplateError;

/// Errors relating to template deletion.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum DeleteError {
    /// A [`TemplateError`] that occurred during the delete.
    Template {
        /// The template that caused the error.
        template: String,
        /// The underlying template error.
        error: TemplateError,
    },

    /// Template to be deleted was not found in the router.
    NotFound {
        /// The template that was not found.
        template: String,
    },
}

impl Error for DeleteError {}

impl fmt::Display for DeleteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Template { template, error } => {
                write!(f, "invalid template `{template}`: {error}")
            }
            Self::NotFound { template } => write!(f, "template `{template}` not found"),
        }
    }
}
