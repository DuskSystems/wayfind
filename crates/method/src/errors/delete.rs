use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum MethodDeleteError {
    NotFound,
    Mismatch,
}

impl Error for MethodDeleteError {}

impl Display for MethodDeleteError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
