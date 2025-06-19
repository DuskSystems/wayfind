use alloc::{borrow::ToOwned, fmt, format, string::String, vec::Vec};
use core::error::Error;

use crate::errors::TemplateError;

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
}

impl Error for InsertError {}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        }
    }
}

impl From<TemplateError> for InsertError {
    fn from(error: TemplateError) -> Self {
        Self::Template(error)
    }
}
