use super::TemplateError;
use crate::errors::EncodingError;
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to delete a route from a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    /// A [`EncodingError`] that occurred during the decoding.
    Encoding(EncodingError),

    /// A [`TemplateError`] that occurred during the delete.
    Template(TemplateError),

    /// Route to be deleted was not found in the router.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_path::errors::DeleteError;
    ///
    /// let error = DeleteError::NotFound {
    ///     route: "/not_found".to_string(),
    /// };
    ///
    /// let display = "
    /// not found
    ///
    ///    Route: /not_found
    ///
    /// The specified route does not exist in the router
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    NotFound {
        /// The route that was not found in the router.
        route: String,
    },

    /// Tried to delete a route using a format that doesn't match how it was inserted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_path::errors::DeleteError;
    ///
    /// let error = DeleteError::Mismatch {
    ///     route: "/users/{id}/".to_string(),
    ///     inserted: "/users/{id}(/)".to_string(),
    /// };
    ///
    /// let display = "
    /// delete mismatch
    ///
    ///       Route: /users/{id}/
    ///    Inserted: /users/{id}(/)
    ///
    /// The route must be deleted using the same format as was inserted
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    Mismatch {
        /// The route that was attempted to be deleted.
        route: String,
        /// The route stored as stored in the router.
        inserted: String,
    },
}

impl Error for DeleteError {}

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encoding(error) => error.fmt(f),
            Self::Template(error) => error.fmt(f),
            Self::NotFound { route } => write!(
                f,
                r"not found

   Route: {route}

The specified route does not exist in the router"
            ),
            Self::Mismatch { route, inserted } => write!(
                f,
                r"delete mismatch

      Route: {route}
   Inserted: {inserted}

The route must be deleted using the same format as was inserted"
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
