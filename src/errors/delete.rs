use super::{route::RouteError, EncodingError};
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to delete a route from a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    /// A [`EncodingError`] that occurred during the decoding.
    EncodingError(EncodingError),

    /// A [`RouteError`] that occurred during the delete.
    RouteError(RouteError),

    /// Route to be deleted was not found in the router.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::DeleteError;
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
}

impl Error for DeleteError {}

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EncodingError(error) => error.fmt(f),
            Self::RouteError(error) => error.fmt(f),
            Self::NotFound { route } => write!(
                f,
                r#"not found

   Route: {route}

The specified route does not exist in the router"#
            ),
        }
    }
}

impl From<EncodingError> for DeleteError {
    fn from(error: EncodingError) -> Self {
        Self::EncodingError(error)
    }
}

impl From<RouteError> for DeleteError {
    fn from(error: RouteError) -> Self {
        Self::RouteError(error)
    }
}
