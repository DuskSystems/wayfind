use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum RouteError {
    EmptyPath,

    // Braces
    EmptyBraces { path: String, position: usize },

    // Escaping
    UnescapedBrace { path: String, position: usize },

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
            Self::EmptyPath => write!(f, "error: empty path"),

            // Braces
            Self::EmptyBraces { path, position } => {
                let underline = " ".repeat(*position) + "^^";
                write!(
                    f,
                    r#"error: empty braces

   Path: {path}
         {underline}"#
                )
            }

            // Escaping
            Self::UnescapedBrace { path, position } => {
                let underline = " ".repeat(*position) + "^";
                write!(
                    f,
                    r#"error: unescaped brace

   Path: {path}
         {underline}

tip: Use '{{{{' to represent a literal '{{' and '}}}}' to represent a literal '}}' in the path"#
                )
            }

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
