use alloc::{fmt, string::String};
use core::error::Error;

use crate::errors::TemplateError;

#[derive(Eq, PartialEq, Debug)]
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
    /// let display = r"
    /// not found
    ///
    ///     Template: /not_found
    ///
    /// help: The specified template does not exist in the router
    ///
    /// try:
    ///     - Check if the template is correct
    ///     - Verify that the template was previously inserted
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
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
            Self::NotFound { template } => write!(
                f,
                r"not found

    Template: {template}

help: The specified template does not exist in the router

try:
    - Check if the template is correct
    - Verify that the template was previously inserted"
            ),
        }
    }
}

impl From<TemplateError> for DeleteError {
    fn from(error: TemplateError) -> Self {
        Self::Template(error)
    }
}
