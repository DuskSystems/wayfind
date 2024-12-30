use super::PathTemplateError;
use crate::errors::EncodingError;
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to delete a route from a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum PathDeleteError {
    /// A [`EncodingError`] that occurred during the decoding.
    EncodingError(EncodingError),

    /// A [`RouteError`] that occurred during the delete.
    TemplateError(PathTemplateError),

    /// Route to be deleted was not found in the router.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathDeleteError;
    ///
    /// let error = PathDeleteError::NotFound {
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
    /// use wayfind::errors::PathDeleteError;
    ///
    /// let error = PathDeleteError::Mismatch {
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

impl Error for PathDeleteError {}

impl Display for PathDeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EncodingError(error) => error.fmt(f),
            Self::TemplateError(error) => error.fmt(f),
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

impl From<EncodingError> for PathDeleteError {
    fn from(error: EncodingError) -> Self {
        Self::EncodingError(error)
    }
}

impl From<PathTemplateError> for PathDeleteError {
    fn from(error: PathTemplateError) -> Self {
        Self::TemplateError(error)
    }
}
