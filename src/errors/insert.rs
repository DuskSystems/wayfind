use std::{error::Error, fmt::Display};

use super::TemplateError;

#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    /// A [`TemplateError`] that occurred during the insert.
    Template(TemplateError),

    /// One or more conflicting templates found during the insert.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::InsertError;
    ///
    /// let error = InsertError::Conflict {
    ///     template: "(/a(/b))(/x/y)".to_owned(),
    ///     conflicts: vec![
    ///         "/a(/b)".to_owned(),
    ///         "/x/y".to_owned(),
    ///     ]
    /// };
    ///
    /// let display = r"
    /// conflicts detected
    ///
    ///     Template: (/a(/b))(/x/y)
    ///     Conflicts:
    ///         - /a(/b)
    ///         - /x/y
    ///
    /// help: Templates cannot overlap with existing templates
    ///
    /// try:
    ///     - Modify the template to be more specific
    ///     - Use a constraint to disambiguate the template
    ///     - Remove conflicting templates
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    Conflict {
        /// The template being inserted.
        template: String,
        /// List of existing templates that conflict.
        conflicts: Vec<String>,
    },

    /// The constraint specified in the template is not recognized by the router.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::InsertError;
    ///
    /// let error = InsertError::UnknownConstraint {
    ///     constraint: "unknown_constraint".to_owned(),
    /// };
    ///
    /// let display = r"
    /// unknown constraint
    ///
    ///     Constraint: unknown_constraint
    ///
    /// help: The router must be configured with this constraint before use
    ///
    /// try:
    ///     - Register the constraint with the router
    ///     - Check for typos in the constraint name
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
            Self::Conflict {
                template,
                conflicts,
            } => {
                let conflicts = conflicts
                    .iter()
                    .map(|conflict| format!("        - {conflict}"))
                    .collect::<Vec<_>>()
                    .join("\n")
                    .trim_end()
                    .to_owned();

                write!(
                    f,
                    r"conflicts detected

    Template: {template}
    Conflicts:
{conflicts}

help: Templates cannot overlap with existing templates

try:
    - Modify the template to be more specific
    - Use a constraint to disambiguate the template
    - Remove conflicting templates"
                )
            }
            Self::UnknownConstraint { constraint } => write!(
                f,
                r"unknown constraint

    Constraint: {constraint}

help: The router must be configured with this constraint before use

try:
    - Register the constraint with the router
    - Check for typos in the constraint name"
            ),
        }
    }
}

impl From<TemplateError> for InsertError {
    fn from(error: TemplateError) -> Self {
        Self::Template(error)
    }
}
