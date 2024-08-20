use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum RouteError {
    EmptyPath,

    // Braces
    EmptyBraces {
        path: String,
        position: usize,
    },

    // Escaping
    UnescapedBrace {
        path: String,
        position: usize,
    },

    // Parameter
    EmptyParameter {
        path: String,
        start: usize,
        length: usize,
    },

    InvalidParameter {
        path: String,
        start: usize,
        length: usize,
    },

    // Wildcard
    EmptyWildcard {
        path: String,
        start: usize,
        length: usize,
    },

    // Constraint
    EmptyConstraint {
        path: String,
        start: usize,
        length: usize,
    },

    InvalidConstraint {
        path: String,
        start: usize,
        length: usize,
    },
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
            Self::EmptyParameter {
                path,
                start,
                length,
            } => {
                let underline = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"error: empty parameter name

   Path: {path}
         {underline}"#
                )
            }

            Self::InvalidParameter {
                path,
                start,
                length,
            } => {
                let underline = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"error: invalid parameter name

   Path: {path}
         {underline}

tip: Parameter names must not contain the characters: ':', '*', '?', '{{', '}}', '/'"#
                )
            }

            // Wildcard
            Self::EmptyWildcard {
                path,
                start,
                length,
            } => {
                let underline = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"error: empty wildcard name

   Path: {path}
         {underline}"#
                )
            }

            // Constraint
            Self::EmptyConstraint {
                path,
                start,
                length,
            } => {
                let underline = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"error: empty constraint name

   Path: {path}
         {underline}"#
                )
            }

            Self::InvalidConstraint {
                path,
                start,
                length,
            } => {
                let underline = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"error: invalid constraint name

   Path: {path}
         {underline}

tip: Constraint names must not contain the characters: ':', '*', '?', '{{', '}}', '/'"#
                )
            }
        }
    }
}
