use std::{error::Error, fmt::Display};

/// Errors relating to malformed routes.
#[derive(Debug, PartialEq, Eq)]
pub enum RouteError {
    /// The route is empty.
    EmptyRoute,

    /// Empty braces were found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::EmptyBraces {
    ///     route: "/{}".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let display = "
    /// empty braces
    ///
    ///    Route: /{}
    ///           ^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyBraces {
        /// The route containing empty braces.
        route: String,
        /// The position of the empty brace.
        position: usize,
    },

    /// An unescaped brace was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::UnescapedBrace {
    ///     route: "/{".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let display = "
    /// unescaped brace
    ///
    ///    Route: /{
    ///           ^
    ///
    /// tip: Use '{{' and '}}' to represent literal '{' and '}' characters in the route
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    UnescapedBrace {
        /// The route containing an unescaped brace.
        route: String,
        /// The position of the unescaped brace.
        position: usize,
    },

    /// An empty parameter name was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::EmptyParameter {
    ///     route: "/{:}".to_string(),
    ///     start: 1,
    ///     length: 3,
    /// };
    ///
    /// let display = "
    /// empty parameter name
    ///
    ///    Route: /{:}
    ///           ^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyParameter {
        /// The route containing an empty parameter.
        route: String,
        /// The position of the parameter with a empty name.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An invalid parameter name was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::InvalidParameter {
    ///     route: "/{a/b}".to_string(),
    ///     name: "a/b".to_string(),
    ///     start: 1,
    ///     length: 5,
    /// };
    ///
    /// let display = "
    /// invalid parameter name
    ///
    ///    Route: /{a/b}
    ///           ^^^^^
    ///
    /// tip: Parameter names must not contain the characters: ':', '*', '?', '{', '}', '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidParameter {
        /// The route containing an invalid parameter.
        route: String,
        /// The invalid parameter name.
        name: String,
        /// The position of the parameter with a invalid name.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// A wildcard parameter with no name was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::EmptyWildcard {
    ///     route: "/{*}".to_string(),
    ///     start: 1,
    ///     length: 3,
    /// };
    ///
    /// let display = "
    /// empty wildcard name
    ///
    ///    Route: /{*}
    ///           ^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyWildcard {
        /// The route containing an empty wildcard parameter.
        route: String,
        /// The position of the wildcard parameter with a empty name.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An empty constraint name was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::EmptyConstraint {
    ///     route: "/{a:}".to_string(),
    ///     start: 1,
    ///     length: 4,
    /// };
    ///
    /// let display = "
    /// empty constraint name
    ///
    ///    Route: /{a:}
    ///           ^^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyConstraint {
        /// The route containing an empty constraint.
        route: String,
        /// The position of the parameter with an empty constraint.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An invalid constraint name was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::RouteError;
    ///
    /// let error = RouteError::InvalidConstraint {
    ///     route: "/{a:b/c}".to_string(),
    ///     name: "b/c".to_string(),
    ///     start: 1,
    ///     length: 7,
    /// };
    ///
    /// let display = "
    /// invalid constraint name
    ///
    ///    Route: /{a:b/c}
    ///           ^^^^^^^
    ///
    /// tip: Constraint names must not contain the characters: ':', '*', '?', '{', '}', '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidConstraint {
        /// The route containing an invalid constraint.
        route: String,
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
            Self::EmptyRoute => write!(f, "empty route"),

            Self::EmptyBraces { route, position } => {
                let arrow = " ".repeat(*position) + "^^";
                write!(
                    f,
                    r#"empty braces

   Route: {route}
         {arrow}"#
                )
            }

            Self::UnescapedBrace { route, position } => {
                let arrow = " ".repeat(*position) + "^";
                write!(
                    f,
                    r#"unescaped brace

   Route: {route}
         {arrow}

tip: Use '{{{{' and '}}}}' to represent literal '{{' and '}}' characters in the route"#
                )
            }

            Self::EmptyParameter {
                route,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"empty parameter name

   Route: {route}
         {arrow}"#
                )
            }

            Self::InvalidParameter {
                route,
                start,
                length,
                ..
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"invalid parameter name

   Route: {route}
         {arrow}

tip: Parameter names must not contain the characters: ':', '*', '?', '{{', '}}', '/'"#
                )
            }

            Self::EmptyWildcard {
                route,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"empty wildcard name

   Route: {route}
         {arrow}"#
                )
            }

            Self::EmptyConstraint {
                route,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"empty constraint name

   Route: {route}
         {arrow}"#
                )
            }

            Self::InvalidConstraint {
                route,
                start,
                length,
                ..
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"invalid constraint name

   Route: {route}
         {arrow}

tip: Constraint names must not contain the characters: ':', '*', '?', '{{', '}}', '/'"#
                )
            }
        }
    }
}
