use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum SearchError {
    NotAllowed,
}

impl Error for SearchError {}

impl Display for SearchError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
