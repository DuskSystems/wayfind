use super::{decode::DecodeError, route::RouteError};
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to insert a route into a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    /// A [`RouteError`] that occurred during the insert operation.
    RouteError(RouteError),

    /// A [`DecodeError`] that occurred during the insert operation.
    DecodeError(DecodeError),

    /// The path provided was percent-encoded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::InsertError;
    ///
    /// let error = InsertError::EncodedPath {
    ///     input: "/hello%20world".to_string(),
    ///     decoded: "/hello world".to_string(),
    /// };
    ///
    /// let display = "
    /// encoded path
    ///
    ///      Input: /hello%20world
    ///    Decoded: /hello world
    ///
    /// The router expects paths to be in their decoded form
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EncodedPath {
        /// The original encoded input path.
        input: String,
        /// The decoded version of the path.
        decoded: String,
    },

    /// The path being inserted already exists in the router.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::InsertError;
    ///
    /// let error = InsertError::DuplicatePath {
    ///     path: "/existing/path".to_string(),
    /// };
    ///
    /// let display = "
    /// duplicate path
    ///
    ///    Path: /existing/path
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    DuplicatePath {
        /// The path that already exists in the router.
        path: String,
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
            Self::RouteError(error) => error.fmt(f),
            Self::DecodeError(error) => error.fmt(f),
            Self::EncodedPath { input, decoded } => write!(
                f,
                r#"encoded path

     Input: {input}
   Decoded: {decoded}

The router expects paths to be in their decoded form"#
            ),
            Self::DuplicatePath { path } => write!(
                f,
                r#"duplicate path

   Path: {path}"#
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

impl From<RouteError> for InsertError {
    fn from(error: RouteError) -> Self {
        Self::RouteError(error)
    }
}

impl From<DecodeError> for InsertError {
    fn from(error: DecodeError) -> Self {
        Self::DecodeError(error)
    }
}
