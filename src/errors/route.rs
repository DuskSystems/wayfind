use std::{error::Error, fmt::Display};

/// Errors relating to malformed paths.
#[derive(Debug, PartialEq, Eq)]
pub enum RouteError {
    /// The path is empty.
    EmptyPath,

    /// Empty braces were found in the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::EmptyBraces {
    ///     path: "/{}".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let display = "
    /// empty braces
    ///
    ///    Path: /{}
    ///           ^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyBraces {
        /// The path containing empty braces.
        path: String,
        /// The position of the empty brace.
        position: usize,
    },

    /// An unescaped brace was found in the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::UnescapedBrace {
    ///     path: "/{".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let display = "
    /// unescaped brace
    ///
    ///    Path: /{
    ///           ^
    ///
    /// tip: Use '{{' and '}}' to represent literal '{' and '}' characters in the path
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    UnescapedBrace {
        /// The path containing an unescaped brace.
        path: String,
        /// The position of the unescaped brace.
        position: usize,
    },

    /// An empty parameter name was found in the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::EmptyParameter {
    ///     path: "/{:}".to_string(),
    ///     start: 1,
    ///     length: 3,
    /// };
    ///
    /// let display = "
    /// empty parameter name
    ///
    ///    Path: /{:}
    ///           ^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyParameter {
        /// The path containing an empty parameter.
        path: String,
        /// The position of the parameter with a empty name.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An invalid parameter name was found in the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::InvalidParameter {
    ///     path: "/{a/b}".to_string(),
    ///     name: "a/b".to_string(),
    ///     start: 1,
    ///     length: 5,
    /// };
    ///
    /// let display = "
    /// invalid parameter name
    ///
    ///    Path: /{a/b}
    ///           ^^^^^
    ///
    /// tip: Parameter names must not contain the characters: ':', '*', '?', '{', '}', '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidParameter {
        /// The path containing an invalid parameter.
        path: String,
        /// The invalid parameter name.
        name: String,
        /// The position of the parameter with a invalid name.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// A wildcard parameter with no name was found in the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::EmptyWildcard {
    ///     path: "/{*}".to_string(),
    ///     start: 1,
    ///     length: 3,
    /// };
    ///
    /// let display = "
    /// empty wildcard name
    ///
    ///    Path: /{*}
    ///           ^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyWildcard {
        /// The path containing an empty wildcard parameter.
        path: String,
        /// The position of the wildcard parameter with a empty name.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An empty constraint name was found in the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::EmptyConstraint {
    ///     path: "/{a:}".to_string(),
    ///     start: 1,
    ///     length: 4,
    /// };
    ///
    /// let display = "
    /// empty constraint name
    ///
    ///    Path: /{a:}
    ///           ^^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyConstraint {
        /// The path containing an empty constraint.
        path: String,
        /// The position of the parameter with an empty constraint.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An invalid constraint name was found in the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::InvalidConstraint {
    ///     path: "/{a:b/c}".to_string(),
    ///     name: "b/c".to_string(),
    ///     start: 1,
    ///     length: 7,
    /// };
    ///
    /// let display = "
    /// invalid constraint name
    ///
    ///    Path: /{a:b/c}
    ///           ^^^^^^^
    ///
    /// tip: Constraint names must not contain the characters: ':', '*', '?', '{', '}', '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidConstraint {
        /// The path containing an invalid constraint.
        path: String,
        /// The invalid constraint name.
        name: String,
        /// The position of the parameter with an invalid constraint.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },
}

impl Error for RouteError {}

impl Display for RouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyPath => write!(f, "empty path"),

            Self::EmptyBraces { path, position } => {
                let arrow = " ".repeat(*position) + "^^";
                write!(
                    f,
                    r#"empty braces

   Path: {path}
         {arrow}"#
                )
            }

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
                ..
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
                ..
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
