use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum RouteError {
    EmptyPath,

    // Braces
    EmptyBraces,

    // Escaping
    UnescapedBrace,

    // Parameter
    EmptyParameter,
    InvalidParameter,

    // Wildcard
    EmptyWildcard,

    // Constraint
    EmptyConstraint,
    InvalidConstraint,
}

impl Error for RouteError {}

impl Display for RouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyPath => write!(f, "EmptyPath"),

            // Braces
            Self::EmptyBraces => write!(f, "EmptyBraces"),

            // Escaping
            Self::UnescapedBrace => write!(f, "UnescapedBrace"),

            // Parameter
            Self::EmptyParameter => write!(f, "EmptyParameter"),
            Self::InvalidParameter => write!(f, "InvalidParameter"),

            // Wildcard
            Self::EmptyWildcard => write!(f, "EmptyWildcard"),

            // Constraint
            Self::EmptyConstraint => write!(f, "EmptyConstraint"),
            Self::InvalidConstraint => write!(f, "InvalidConstraint"),
        }
    }
}
