use alloc::{fmt, string::String};
use core::error::Error;

use crate::errors::TemplateError;

#[derive(Debug, PartialEq, Eq)]
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

    /// Tried to delete a template using a format that doesn't match how it was inserted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::DeleteError;
    ///
    /// let error = DeleteError::Mismatch {
    ///     template: "/users/{id}/".to_owned(),
    ///     inserted: "/users/{id}(/)".to_owned(),
    /// };
    ///
    /// let display = r"
    /// delete mismatch
    ///
    ///     Template: /users/{id}/
    ///     Inserted: /users/{id}(/)
    ///
    /// help: The template must be deleted using the same format as was inserted
    ///
    /// try:
    ///     - Use the exact template format shown in 'Inserted'
    ///     - Check for differences in optional segments or trailing slashes
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    Mismatch {
        /// The template that was attempted to be deleted.
        template: String,
        /// The template as stored in the router.
        inserted: String,
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
            Self::Mismatch { template, inserted } => write!(
                f,
                r"delete mismatch

    Template: {template}
    Inserted: {inserted}

help: The template must be deleted using the same format as was inserted

try:
    - Use the exact template format shown in 'Inserted'
    - Check for differences in optional segments or trailing slashes"
            ),
        }
    }
}

impl From<TemplateError> for DeleteError {
    fn from(error: TemplateError) -> Self {
        Self::Template(error)
    }
}
