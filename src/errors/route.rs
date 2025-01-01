use wayfind_authority::errors::AuthorityTemplateError;
use wayfind_path::errors::PathTemplateError;

use super::EncodingError;
use std::{error::Error, fmt::Display};

/// Errors that can occur when creating a [`Route`](`crate::Route`).
#[derive(Debug, PartialEq, Eq)]
pub enum RouteError {
    Encoding(EncodingError),
    Authority(AuthorityTemplateError),
    Path(PathTemplateError),

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

    /// The authority provided was percent-encoded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::EncodedAuthority {
    ///     input: "ドメイン名例".to_string(),
    ///     decoded: "eckwd4c7cu47r2wf".to_string(),
    /// };
    ///
    /// let display = "
    /// encoded authority
    ///
    ///      Input: ドメイン名例
    ///    Decoded: eckwd4c7cu47r2wf
    ///
    /// The router expects authorities to be in their decoded form
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EncodedAuthority {
        /// The original encoded input authority.
        input: String,
        /// The decoded version of the authority.
        decoded: String,
    },

    /// The path provided was percent-encoded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::EncodedPath {
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
}

impl Error for RouteError {}

impl Display for RouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encoding(error) => error.fmt(f),
            Self::Authority(error) => error.fmt(f),
            Self::Path(error) => error.fmt(f),
            Self::MissingRoute => write!(
                f,
                r"missing route

A route must be provided when building a Route"
            ),
            Self::EncodedAuthority { input, decoded } => write!(
                f,
                r"encoded authority

     Input: {input}
   Decoded: {decoded}

The router expects authorities to be in their decoded form"
            ),
            Self::EncodedPath { input, decoded } => write!(
                f,
                r"encoded path

     Input: {input}
   Decoded: {decoded}

The router expects paths to be in their decoded form"
            ),
        }
    }
}

impl From<EncodingError> for RouteError {
    fn from(error: EncodingError) -> Self {
        Self::Encoding(error)
    }
}

impl From<AuthorityTemplateError> for RouteError {
    fn from(error: AuthorityTemplateError) -> Self {
        Self::Authority(error)
    }
}

impl From<PathTemplateError> for RouteError {
    fn from(error: PathTemplateError) -> Self {
        Self::Path(error)
    }
}
