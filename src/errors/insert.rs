use alloc::fmt;
use alloc::string::String;
use core::error::Error;

use crate::errors::TemplateError;

/// Errors relating to template insertion.
#[derive(Clone, Eq, PartialEq)]
pub enum InsertError {
    /// A [`TemplateError`] that occurred during the insert.
    Template(TemplateError),

    /// A conflicting template already exists in the router.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::InsertError;
    ///
    /// let error = InsertError::Conflict {
    ///     template: "/users/<id>".to_owned(),
    ///     existing: "/users/<user>".to_owned(),
    /// };
    ///
    /// let display = "conflict: `/users/<id>` conflicts with `/users/<user>`";
    /// let debug = r"
    /// error: conflict detected
    ///
    ///     /users/<id>
    ///     ━━━━━━━━━━━ conflicts with `/users/<user>`
    ///
    /// help: templates cannot overlap with existing routes
    /// ";
    ///
    /// assert_eq!(format!("{error}"), display);
    /// assert_eq!(format!("{error:?}"), debug.trim());
    /// ```
    Conflict {
        /// The template being inserted.
        template: String,
        /// The existing template that conflicts.
        existing: String,
    },
}

impl Error for InsertError {}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Template(error) => error.fmt(f),
            Self::Conflict { template, existing } => {
                write!(f, "conflict: `{template}` conflicts with `{existing}`")
            }
        }
    }
}

impl fmt::Debug for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Template(error) => error.fmt(f),
            Self::Conflict { template, existing } => {
                let underline = "━".repeat(template.len());
                write!(
                    f,
                    "error: conflict detected

    {template}
    {underline} conflicts with `{existing}`

help: templates cannot overlap with existing routes"
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
