use std::{error::Error, fmt::Display};

use super::{EncodingError, TemplateError};

/// Errors relating to attempting to delete a template from a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    /// A [`EncodingError`] that occurred during the decoding.
    Encoding(EncodingError),

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
    ///     template: "/not_found".to_string(),
    /// };
    ///
    /// let display = "
    /// not found
    ///
    ///    Template: /not_found
    ///
    /// The specified template does not exist in the router
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
    ///     template: "/users/{id}/".to_string(),
    ///     inserted: "/users/{id}(/)".to_string(),
    /// };
    ///
    /// let display = "
    /// delete mismatch
    ///
    ///    Template: /users/{id}/
    ///    Inserted: /users/{id}(/)
    ///
    /// The template must be deleted using the same format as was inserted
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

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encoding(error) => error.fmt(f),
            Self::Template(error) => error.fmt(f),
            Self::NotFound { template } => write!(
                f,
                r"not found

   Template: {template}

The specified template does not exist in the router"
            ),
            Self::Mismatch { template, inserted } => write!(
                f,
                r"delete mismatch

   Template: {template}
   Inserted: {inserted}

The template must be deleted using the same format as was inserted"
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
