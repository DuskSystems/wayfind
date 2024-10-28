use std::{error::Error, fmt::Display};

/// Errors that can occur when creating a [`Routable`](`crate::Routable`).
#[derive(Debug, PartialEq, Eq)]
pub enum RoutableError {
    /// The route was not provided when building the [`Routable`](`crate::Routable`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RoutableError;
    ///
    /// let error = RoutableError::MissingRoute;
    ///
    /// let display = "
    /// missing route
    ///
    /// A route must be provided when building a Routable
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    MissingRoute,
}

impl Error for RoutableError {}

impl Display for RoutableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRoute => write!(
                f,
                r#"missing route

A route must be provided when building a Routable"#
            ),
        }
    }
}
