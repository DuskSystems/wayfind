use super::route::RouteError;
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to delete a route from a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    /// A [`RouteError`] that occurred during the delete.
    RouteError(RouteError),

    /// Path to be deleted was not found in the router.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::DeleteError;
    ///
    /// let error = DeleteError::NotFound {
    ///     path: "/not_found".to_string(),
    /// };
    ///
    /// let display = "
    /// not found
    ///
    ///    Path: /not_found
    ///
    /// The specified path does not exist in the router
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    NotFound {
        /// The path that was not found in the router.
        path: String,
    },
}

impl Error for DeleteError {}

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RouteError(error) => error.fmt(f),
            Self::NotFound { path } => write!(
                f,
                r#"not found

   Path: {path}

The specified path does not exist in the router"#
            ),
        }
    }
}

impl From<RouteError> for DeleteError {
    fn from(error: RouteError) -> Self {
        Self::RouteError(error)
    }
}
