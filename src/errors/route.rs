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
            Self::EmptyPath => write!(f, "empty path"),

            // Braces
            Self::EmptyBraces { path, position } => {
                let arrow = " ".repeat(*position) + "^^";
                write!(
                    f,
                    r#"empty braces

   Path: {path}
         {arrow}"#
                )
            }

            // Escaping
            Self::UnescapedBrace { path, position } => {
                let arrow = " ".repeat(*position) + "^";
                write!(
                    f,
                    r#"unescaped brace

   Path: {path}
         {arrow}

tip: Use '{{{{' and '}}}}' to represent literal '{{' and '}}' characters in the path"#
                )
            }

            // Parameter
            Self::EmptyParameter {
                path,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"empty parameter name

   Path: {path}
         {arrow}"#
                )
            }

            Self::InvalidParameter {
                path,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"invalid parameter name

   Path: {path}
         {arrow}

tip: Parameter names must not contain the characters: ':', '*', '?', '{{', '}}', '/'"#
                )
            }

            // Wildcard
            Self::EmptyWildcard {
                path,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"empty wildcard name

   Path: {path}
         {arrow}"#
                )
            }

            // Constraint
            Self::EmptyConstraint {
                path,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"empty constraint name

   Path: {path}
         {arrow}"#
                )
            }

            Self::InvalidConstraint {
                path,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"invalid constraint name

   Path: {path}
         {arrow}

tip: Constraint names must not contain the characters: ':', '*', '?', '{{', '}}', '/'"#
                )
            }
        }
    }
}
