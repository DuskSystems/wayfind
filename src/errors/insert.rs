use std::{error::Error, fmt::Display};

use super::TemplateError;

/// Errors relating to attempting to insert a template into a [`Router`](crate::Router).
#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    /// A [`TemplateError`] that occurred during the insert operation.
    Template(TemplateError),

    /// FIXME
    Conflict,

    /// The constraint specified in the template is not recognized by the router.
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
            Self::Template(error) => error.fmt(f),
            Self::Conflict => write!(f, "CONFLICT"),
            Self::UnknownConstraint { constraint } => write!(
                f,
                r"unknown constraint

   Constraint: {constraint}

The router doesn't recognize this constraint"
            ),
        }
    }
}

impl From<TemplateError> for InsertError {
    fn from(error: TemplateError) -> Self {
        Self::Template(error)
    }
}
