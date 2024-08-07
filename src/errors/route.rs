use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum RouteError {
    InvalidPath,
}

impl Error for RouteError {}

impl Display for RouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "Invalid Path"),
        }
    }
}
