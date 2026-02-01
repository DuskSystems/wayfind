use alloc::fmt;
use alloc::string::String;
use core::error::Error;

use crate::errors::TemplateError;

/// Errors relating to template deletion.
#[derive(Clone, Eq, PartialEq)]
pub enum DeleteError {
    /// A [`TemplateError`] that occurred during the delete.
    Template(TemplateError),

    /// Template to be deleted was not found in the router.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::DeleteError;
    ///
    /// let error = DeleteError::NotFound {
    ///     template: "/not_found".to_owned(),
    /// };
    ///
    /// let display = "template not found: `/not_found`";
    /// let debug = r"
    /// error: template not found
    ///
    ///     /not_found
    ///     ━━━━━━━━━━
    ///
    /// help: template does not exist in the router
    /// ";
    ///
    /// assert_eq!(format!("{error}"), display);
    /// assert_eq!(format!("{error:?}"), debug.trim());
    /// ```
    NotFound {
        /// The template that was not found in the router.
        template: String,
    },
}

impl Error for DeleteError {}

impl fmt::Display for DeleteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Template(error) => error.fmt(f),
            Self::NotFound { template } => {
                write!(f, "template not found: `{template}`")
            }
        }
    }
}

impl fmt::Debug for DeleteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Template(error) => error.fmt(f),
            Self::NotFound { template } => {
                let underline = "━".repeat(template.len());
                write!(
                    f,
                    "error: template not found

    {template}
    {underline}

help: template does not exist in the router"
                )
            }
        }
    }
}

impl From<TemplateError> for DeleteError {
    fn from(error: TemplateError) -> Self {
        Self::Template(error)
    }
}
