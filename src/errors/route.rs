use super::EncodingError;
use alloc::string::String;
use core::{error::Error, fmt::Display};

/// Errors that can occur when creating a [`Route`](`crate::Route`).
#[derive(Debug, PartialEq, Eq)]
pub enum RouteError {
    /// A [`EncodingError`] that occurred during the creation.
    EncodingError(EncodingError),

    /// The route was not provided when building the [`Route`](`crate::Route`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::MissingRoute;
    ///
    /// let display = "
    /// missing route
    ///
    /// A route must be provided when building a Route
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    MissingRoute,

    /// The route provided was percent-encoded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::EncodedRoute {
    ///     input: "/hello%20world".to_string(),
    ///     decoded: "/hello world".to_string(),
    /// };
    ///
    /// let display = "
    /// encoded route
    ///
    ///      Input: /hello%20world
    ///    Decoded: /hello world
    ///
    /// The router expects routes to be in their decoded form
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EncodedRoute {
        /// The original encoded input route.
        input: String,
        /// The decoded version of the route.
        decoded: String,
    },
}

impl Error for RouteError {}

impl Display for RouteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EncodingError(error) => error.fmt(f),

            Self::MissingRoute => write!(
                f,
                r#"missing route

A route must be provided when building a Route"#
            ),

            Self::EncodedRoute { input, decoded } => write!(
                f,
                r#"encoded route

     Input: {input}
   Decoded: {decoded}

The router expects routes to be in their decoded form"#
            ),
        }
    }
}

impl From<EncodingError> for RouteError {
    fn from(error: EncodingError) -> Self {
        Self::EncodingError(error)
    }
}
