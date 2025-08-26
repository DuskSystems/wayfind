use alloc::{fmt, string::String};
use core::error::Error;

use crate::errors::TemplateError;

#[derive(Eq, PartialEq, Debug)]
pub enum InsertError {
    /// A [`TemplateError`] that occurred during the insert.
    Template(TemplateError),

    /// A conflicting template found during the insert.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::InsertError;
    ///
    /// let error = InsertError::Conflict {
    ///     template: "/users/<id>".to_owned(),
    ///     conflict: "/users/<user>".to_owned(),
    /// };
    ///
    /// let display = r"
    /// conflict detected
    ///
    ///     Template: /users/<id>
    ///     Conflict: /users/<user>
    ///
    /// help: Templates cannot overlap with existing templates
    ///
    /// try:
    ///     - Modify the template to be more specific
    ///     - Remove the conflicting template
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    Conflict {
        /// The template being inserted.
        template: String,
        /// The existing template that conflicts.
        conflict: String,
    },
}

impl Error for InsertError {}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Template(error) => error.fmt(f),
            Self::Conflict { template, conflict } => {
                write!(
                    f,
                    r"conflict detected

    Template: {template}
    Conflict: {conflict}

help: Templates cannot overlap with existing templates

try:
    - Modify the template to be more specific
    - Remove the conflicting template"
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
