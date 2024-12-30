use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    NotFound,
    Mismatch,
}

impl Error for DeleteError {}

impl Display for DeleteError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
