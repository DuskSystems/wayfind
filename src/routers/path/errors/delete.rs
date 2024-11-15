use super::PathRouteError;
use crate::errors::EncodingError;
use alloc::{string::String, vec::Vec};
use core::{error::Error, fmt::Display};

/// Errors relating to attempting to delete a route from a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum PathDeleteError {
    /// Multiple [`PathDeleteError`] errors occurred during the delete.
    Multiple(Vec<PathDeleteError>),

    /// A [`EncodingError`] that occurred during the decoding.
    EncodingError(EncodingError),

    /// A [`RouteError`] that occurred during the delete.
    PathRouteError(PathRouteError),

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
    /// let error = PathDeleteError::RouteMismatch {
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
    RouteMismatch {
        /// The route that was attempted to be deleted.
        route: String,
        /// The route stored as stored in the router.
        inserted: String,
    },
}

impl Error for PathDeleteError {}

impl Display for PathDeleteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Multiple(errors) => {
                writeln!(f, "multiple path delete errors occurred:\n---\n")?;
                for (index, error) in errors.iter().enumerate() {
                    write!(f, "{error}")?;
                    if index < errors.len() - 1 {
                        writeln!(f, "\n---\n")?;
                    }
                }
                Ok(())
            }
            Self::EncodingError(error) => error.fmt(f),
            Self::PathRouteError(error) => error.fmt(f),
            Self::NotFound { route } => write!(
                f,
                r#"not found

   Route: {route}

The specified route does not exist in the router"#
            ),
            Self::RouteMismatch { route, inserted } => write!(
                f,
                r#"delete mismatch

      Route: {route}
   Inserted: {inserted}

The route must be deleted using the same format as was inserted"#
            ),
        }
    }
}

impl From<EncodingError> for PathDeleteError {
    fn from(error: EncodingError) -> Self {
        Self::EncodingError(error)
    }
}

impl From<PathRouteError> for PathDeleteError {
    fn from(error: PathRouteError) -> Self {
        Self::PathRouteError(error)
    }
}
