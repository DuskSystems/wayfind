use super::PathTemplateError;
use crate::PathId;
use std::{error::Error, fmt::Display};

/// Errors relating to attempting to insert a route into a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum PathInsertError {
    /// A [`TemplateError`] that occurred during the insert operation.
    TemplateError(PathTemplateError),

    /// TODO
    OverlappingRoutes { ids: Vec<PathId> },

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TemplateError(error) => error.fmt(f),
            Self::OverlappingRoutes { ids } => write!(f, r"overlapping routes {ids:?}"),
            Self::UnknownConstraint { constraint } => write!(
                f,
                r"unknown constraint

   Constraint: {constraint}

The router doesn't recognize this constraint"
            ),
        }
    }
}

impl From<PathTemplateError> for PathInsertError {
    fn from(error: PathTemplateError) -> Self {
        Self::TemplateError(error)
    }
}
