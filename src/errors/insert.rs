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
    ///     route: "/existing/route".to_string(),
    /// };
    ///
    /// let display = "
    /// duplicate route
    ///
    ///    Route: /existing/route
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    DuplicateRoute {
        /// The route that already exists in the router.
        route: String,
    },

    /// The constraint specified in the route is not recognized by the router.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::InsertError;
    ///
    /// let error = InsertError::UnknownConstraint {
    ///     constraint: "unknown_constraint".to_string(),
    /// };
    ///
    /// let display = "
    /// unknown constraint
    ///
    ///    Constraint: unknown_constraint
    ///
    /// The router doesn't recognize this constraint
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    UnknownConstraint {
        /// The name of the unrecognized constraint.
        constraint: String,
    },
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EncodingError(error) => error.fmt(f),
            Self::RouteError(error) => error.fmt(f),
            Self::DuplicateRoute { route } => write!(
                f,
                r#"duplicate route

   Route: {route}"#
            ),
            Self::UnknownConstraint { constraint } => write!(
                f,
                r#"unknown constraint

   Constraint: {constraint}

The router doesn't recognize this constraint"#
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
