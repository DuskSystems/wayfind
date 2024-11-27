use super::PathRouteError;
use alloc::{string::String, vec::Vec};
use core::{error::Error, fmt::Display};

/// Errors relating to attempting to insert a route into a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum PathInsertError {
    /// Multiple [`PathInsertError`] errors occurred during the insert.
    Multiple(Vec<PathInsertError>),

    /// A [`PathRouteError`] that occurred during the insert operation.
    PathRouteError(PathRouteError),

    /// The route being inserted already exists in the router.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathInsertError;
    ///
    /// let error = PathInsertError::DuplicateRoute {
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

    /// The constraint specified in the route is not recognized by the router.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathInsertError;
    ///
    /// let error = PathInsertError::UnknownConstraint {
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

impl Error for PathInsertError {}

impl Display for PathInsertError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Multiple(errors) => {
                writeln!(f, "multiple path insert errors occurred:\n---\n")?;
                for (index, error) in errors.iter().enumerate() {
                    write!(f, "{error}")?;
                    if index < errors.len() - 1 {
                        writeln!(f, "\n---\n")?;
                    }
                }
                Ok(())
            }
            Self::PathRouteError(error) => error.fmt(f),
            Self::DuplicateRoute { route, conflict } => write!(
                f,
                r"duplicate route

      Route: {route}
   Conflict: {conflict}"
            ),
            Self::UnknownConstraint { constraint } => write!(
                f,
                r"unknown constraint

   Constraint: {constraint}

The router doesn't recognize this constraint"
            ),
        }
    }
}

impl From<PathRouteError> for PathInsertError {
    fn from(error: PathRouteError) -> Self {
        Self::PathRouteError(error)
    }
}
