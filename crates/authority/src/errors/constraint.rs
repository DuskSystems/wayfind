use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum ConstraintError {
    DuplicateName {
        name: &'static str,
        existing_type: &'static str,
        new_type: &'static str,
    },
}

impl Error for ConstraintError {}

impl Display for ConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
