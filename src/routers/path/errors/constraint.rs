use core::{error::Error, fmt::Display};

/// Errors relating to constraints.
#[derive(Debug, PartialEq, Eq)]
pub enum PathConstraintError {
    /// Constraint name is already in use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathConstraintError;
    ///
    /// let error = PathConstraintError::DuplicateName {
    ///     name: "my_constraint",
    ///     existing_type: "my_crate::constraints::A",
    ///     new_type: "my_crate::constraints::B",
    /// };
    ///
    /// let display = "
    /// duplicate constraint name
    ///
    /// The constraint name 'my_constraint' is already in use:
    ///     - existing constraint type: 'my_crate::constraints::A'
    ///     - new constraint type: 'my_crate::constraints::B'
    ///
    /// help: each constraint must have a unique name
    ///
    /// try:
    ///     - Check if you have accidentally added the same constraint twice
    ///     - Ensure different constraints have different names
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    DuplicateName {
        /// The name of the constraint.
        name: &'static str,
        /// The [`type_name`](std::any::type_name) of the already existing constraint.
        existing_type: &'static str,
        /// The [`type_name`](std::any::type_name) of the attempted new constraint.
        new_type: &'static str,
    },
}

impl Error for PathConstraintError {}

impl Display for PathConstraintError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DuplicateName {
                name,
                existing_type,
                new_type,
            } => write!(
                f,
                "duplicate constraint name

The constraint name '{name}' is already in use:
    - existing constraint type: '{existing_type}'
    - new constraint type: '{new_type}'

help: each constraint must have a unique name

try:
    - Check if you have accidentally added the same constraint twice
    - Ensure different constraints have different names",
            ),
        }
    }
}