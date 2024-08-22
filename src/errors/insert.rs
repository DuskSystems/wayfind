use super::{decode::DecodeError, route::RouteError};
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum InsertError {
    RouteError(RouteError),
    DecodeError(DecodeError),
    EncodedPath { input: String, decoded: String },
    DuplicatePath { path: String },
    UnknownConstraint { constraint: String },
}

impl Error for InsertError {}

impl Display for InsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RouteError(error) => error.fmt(f),
            Self::DecodeError(error) => error.fmt(f),
            Self::EncodedPath { input, decoded } => write!(
                f,
                r#"encoded path

     Input: {}
   Decoded: {}

The router expects paths to be in their decoded form"#,
                input, decoded,
            ),
            Self::DuplicatePath { path } => write!(
                f,
                r#"duplicate path

   Path: {}"#,
                path
            ),
            Self::UnknownConstraint { constraint } => write!(
                f,
                r#"unknown constraint

   Constraint: {}

The router doesn't recognize this constraint"#,
                constraint
            ),
        }
    }
}

impl From<RouteError> for InsertError {
    fn from(error: RouteError) -> Self {
        Self::RouteError(error)
    }
}

impl From<DecodeError> for InsertError {
    fn from(error: DecodeError) -> Self {
        Self::DecodeError(error)
    }
}
