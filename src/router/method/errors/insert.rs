use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum MethodInsertError {
    Empty,
    Conflict,
}

impl Error for MethodInsertError {}

impl Display for MethodInsertError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
