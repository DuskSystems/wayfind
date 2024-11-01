use super::{route::RouteError, EncodingError};
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to insert a route into a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    /// A [`EncodingError`] that occurred during the decoding.
    EncodingError(EncodingError),

    /// A [`RouteError`] that occurred during the insert operation.
    RouteError(RouteError),

    /// The route being inserted already exists in the router.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::InsertError;
    ///
    /// let error = InsertError::DuplicateRoute {
    ///     route: "/route".to_string(),
    ///     conflict: "/existing(/{route})".to_string(),
    /// };
    ///
    /// let display = "
    /// duplicate route
    ///
    ///       Route: /route
    ///    Conflict: /existing(/{route})
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    DuplicateRoute {
        /// The route that was attempted to be inserted.
        route: String,

        /// The route that is conflicting.
        conflict: String,
    },
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EncodingError(error) => error.fmt(f),
            Self::RouteError(error) => error.fmt(f),
            Self::DuplicateRoute { route, conflict } => write!(
                f,
                r#"duplicate route

      Route: {route}
   Conflict: {conflict}"#
            ),
        }
    }
}

impl From<EncodingError> for InsertError {
    fn from(error: EncodingError) -> Self {
        Self::EncodingError(error)
    }
}

impl From<RouteError> for InsertError {
    fn from(error: RouteError) -> Self {
        Self::RouteError(error)
    }
}
