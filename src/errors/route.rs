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
        /// The position where the empty braces were found.
        position: usize,
    },

    /// An unescaped brace was found in the path.
    ///
    /// # Examples
    ///
    /// ```
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
        /// The path constaining an unescaped brace.
        path: String,
        /// The position of the unescaped brace.
        position: usize,
    },

    /// An empty parameter name was found in the path.
    ///
    /// # Examples
    ///
    /// ```
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
        /// The path constaining an empty parameter.
        path: String,
        /// The position of the empty parameter.
        start: usize,
        /// The length of the empty parameter (including braces).
        length: usize,
    },

    /// An invalid parameter name was found in the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::InvalidParameter {
    ///     path: "/{a/b}".to_string(),
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
        /// The path constaining an invalid parameter.
        path: String,
        /// The position of the invalid parameter.
        start: usize,
        /// The length of the invalid parameter (including braces).
        length: usize,
    },

    /// An empty wildcard parameter was found in the path.
    ///
    /// # Examples
    ///
    /// ```
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
        /// The path constaining an empty wildcard parameter.
        path: String,
        /// The position of the empty wildcard parameter.
        start: usize,
        /// The length of the empty wildcard parameter (including braces).
        length: usize,
    },

    /// An empty constraint name was found in the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::EmptyConstraint {
    ///     path: "/{a:}".to_string(),
    ///     position: 3,
    /// };
    ///
    /// let display = "
    /// empty constraint name
    ///
    ///    Path: /{a:}
    ///             ^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyConstraint {
        /// The path constaining an empty constraint.
        path: String,
        /// The position of the empty constraint delimiter (i.e. ':').
        position: usize,
    },

    /// An invalid constraint name was found in the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::InvalidConstraint {
    ///     path: "/{a:b/c}".to_string(),
    ///     start: 4,
    ///     length: 3,
    /// };
    ///
    /// let display = "
    /// invalid constraint name
    ///
    ///    Path: /{a:b/c}
    ///              ^^^
    ///
    /// tip: Constraint names must not contain the characters: ':', '*', '?', '{', '}', '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidConstraint {
        /// The path constaining an invalid constraint.
        path: String,
        /// The position of the invalid constraint.
        start: usize,
        /// The length of the invalid constraint.
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
            Self::EmptyConstraint { path, position } => {
                let arrow = " ".repeat(*position) + "^";
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
