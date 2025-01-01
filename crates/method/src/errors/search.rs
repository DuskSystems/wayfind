use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum MethodSearchError {
    NotAllowed,
}

impl Error for MethodSearchError {}

impl Display for MethodSearchError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
