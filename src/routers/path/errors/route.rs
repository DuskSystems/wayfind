use alloc::string::String;

use crate::errors::EncodingError;

/// Errors relating to malformed routes.
#[derive(Debug, PartialEq, Eq)]
pub enum PathRouteError {
    /// A [`EncodingError`] that occurred during the decoding.
    EncodingError(EncodingError),

    /// The route is empty.
    Empty,

    /// The route must start with '/'.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathRouteError;
    ///
    /// let error = PathRouteError::MissingLeadingSlash {
    ///     route: "abc".to_string(),
    /// };
    ///
    /// let display = "
    /// missing leading slash
    ///
    ///     Route: abc
    ///
    /// tip: Routes must begin with '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    MissingLeadingSlash {
        /// The route missing a leading slash.
        route: String,
    },

    /// Empty braces were found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathRouteError;
    ///
    /// let error = PathRouteError::EmptyBraces {
    ///     route: "/{}".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let display = "
    /// empty braces
    ///
    ///     Route: /{}
    ///             ^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyBraces {
        /// The route containing empty braces.
        route: String,
        /// The position of the first empty brace.
        position: usize,
    },

    /// An unbalanced brace was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathRouteError;
    ///
    /// let error = PathRouteError::UnbalancedBrace {
    ///     route: "/{".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let display = "
    /// unbalanced brace
    ///
    ///     Route: /{
    ///             ^
    ///
    /// tip: Use '\\{' and '\\}' to represent literal '{' and '}' characters in the route
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    UnbalancedBrace {
        /// The route containing an unbalanced brace.
        route: String,
        /// The position of the unbalanced brace.
        position: usize,
    },

    /// Empty parentheses were found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathRouteError;
    ///
    /// let error = PathRouteError::EmptyParentheses {
    ///     route: "/()".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let display = "
    /// empty parentheses
    ///
    ///     Route: /()
    ///             ^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyParentheses {
        /// The route containing empty parentheses.
        route: String,
        /// The position of the first empty parenthesis.
        position: usize,
    },

    /// An unbalanced parenthesis was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathRouteError;
    ///
    /// let error = PathRouteError::UnbalancedParenthesis {
    ///     route: "/(".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let display = "
    /// unbalanced parenthesis
    ///
    ///     Route: /(
    ///             ^
    ///
    /// tip: Use '\\(' and '\\)' to represent literal '(' and ')' characters in the route
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    UnbalancedParenthesis {
        /// The route containing an unbalanced parenthesis.
        route: String,
        /// The position of the unbalanced parenthesis.
        position: usize,
    },

    /// An empty parameter name was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathRouteError;
    ///
    /// let error = PathRouteError::EmptyParameter {
    ///     route: "/{:}".to_string(),
    ///     start: 1,
    ///     length: 3,
    /// };
    ///
    /// let display = "
    /// empty parameter name
    ///
    ///     Route: /{:}
    ///             ^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyParameter {
        /// The route containing an empty parameter.
        route: String,
        /// The position of the opening brace of the empty name parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An invalid parameter name was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathRouteError;
    ///
    /// let error = PathRouteError::InvalidParameter {
    ///     route: "/{a/b}".to_string(),
    ///     name: "a/b".to_string(),
    ///     start: 1,
    ///     length: 5,
    /// };
    ///
    /// let display = "
    /// invalid parameter name
    ///
    ///     Route: /{a/b}
    ///             ^^^^^
    ///
    /// tip: Parameter names must not contain the characters: ':', '*', '{', '}', '(', ')', '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidParameter {
        /// The route containing an invalid parameter.
        route: String,
        /// The invalid parameter name.
        name: String,
        /// The position of the opening brace of the invalid name parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// A duplicate parameter name was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathRouteError;
    ///
    /// let error = PathRouteError::DuplicateParameter {
    ///     route: "/{id}/{id}".to_string(),
    ///     name: "id".to_string(),
    ///     first: 1,
    ///     first_length: 4,
    ///     second: 6,
    ///     second_length: 4,
    /// };
    ///
    /// let display = "
    /// duplicate parameter name: 'id'
    ///
    ///     Route: /{id}/{id}
    ///             ^^^^ ^^^^
    ///
    /// tip: Parameter names must be unique within a route
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    DuplicateParameter {
        /// The route containing duplicate parameters.
        route: String,
        /// The duplicated parameter name.
        name: String,
        /// The position of the opening brace of the first occurrence.
        first: usize,
        /// The length of the first parameter (including braces).
        first_length: usize,
        /// The position of the opening brace of the second occurrence.
        second: usize,
        /// The length of the second parameter (including braces).
        second_length: usize,
    },

    /// A wildcard parameter with no name was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathRouteError;
    ///
    /// let error = PathRouteError::EmptyWildcard {
    ///     route: "/{*}".to_string(),
    ///     start: 1,
    ///     length: 3,
    /// };
    ///
    /// let display = "
    /// empty wildcard name
    ///
    ///     Route: /{*}
    ///             ^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyWildcard {
        /// The route containing an empty wildcard parameter.
        route: String,
        /// The position of the opening brace of the empty wildcard parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An empty constraint name was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathRouteError;
    ///
    /// let error = PathRouteError::EmptyConstraint {
    ///     route: "/{a:}".to_string(),
    ///     start: 1,
    ///     length: 4,
    /// };
    ///
    /// let display = "
    /// empty constraint name
    ///
    ///     Route: /{a:}
    ///             ^^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyConstraint {
        /// The route containing an empty constraint.
        route: String,
        /// The position of the opening brace of the empty constraint parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An invalid constraint name was found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathRouteError;
    ///
    /// let error = PathRouteError::InvalidConstraint {
    ///     route: "/{a:b/c}".to_string(),
    ///     name: "b/c".to_string(),
    ///     start: 1,
    ///     length: 7,
    /// };
    ///
    /// let display = "
    /// invalid constraint name
    ///
    ///     Route: /{a:b/c}
    ///             ^^^^^^^
    ///
    /// tip: Constraint names must not contain the characters: ':', '*', '{', '}', '(', ')', '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidConstraint {
        /// The route containing an invalid constraint.
        route: String,
        /// The invalid constraint name.
        name: String,
        /// The position of the opening brace of the invalid constraint parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// Two parameters side by side were found in the route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::PathRouteError;
    ///
    /// let error = PathRouteError::TouchingParameters {
    ///     route: "/{a}{b}".to_string(),
    ///     start: 1,
    ///     length: 6,
    /// };
    ///
    /// let display = "
    /// touching parameters
    ///
    ///     Route: /{a}{b}
    ///             ^^^^^^
    ///
    /// tip: Touching parameters are not supported
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    TouchingParameters {
        /// The route containing touching parameters.
        route: String,
        /// The position of the first opening brace.
        start: usize,
        /// The combined length of both parameters (including braces).
        length: usize,
    },
}

impl core::error::Error for PathRouteError {}

impl core::fmt::Display for PathRouteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EncodingError(error) => error.fmt(f),
            Self::Empty => write!(f, "empty route"),

            Self::MissingLeadingSlash { route } => {
                write!(
                    f,
                    r#"missing leading slash

    Route: {route}

tip: Routes must begin with '/'"#
                )
            }

            Self::EmptyBraces { route, position } => {
                let arrow = " ".repeat(*position) + "^^";
                write!(
                    f,
                    r#"empty braces

    Route: {route}
           {arrow}"#
                )
            }

            Self::UnbalancedBrace { route, position } => {
                let arrow = " ".repeat(*position) + "^";
                write!(
                    f,
                    r#"unbalanced brace

    Route: {route}
           {arrow}

tip: Use '\{{' and '\}}' to represent literal '{{' and '}}' characters in the route"#
                )
            }

            Self::EmptyParentheses { route, position } => {
                let arrow = " ".repeat(*position) + "^^";
                write!(
                    f,
                    r#"empty parentheses

    Route: {route}
           {arrow}"#
                )
            }

            Self::UnbalancedParenthesis { route, position } => {
                let arrow = " ".repeat(*position) + "^";
                write!(
                    f,
                    r#"unbalanced parenthesis

    Route: {route}
           {arrow}

tip: Use '\(' and '\)' to represent literal '(' and ')' characters in the route"#
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

tip: Parameter names must not contain the characters: ':', '*', '{{', '}}', '(', ')', '/'"#
                )
            }

            Self::DuplicateParameter {
                route,
                name,
                first,
                first_length,
                second,
                second_length,
            } => {
                let mut arrow = " ".repeat(route.len());

                arrow.replace_range(*first..(*first + *first_length), &"^".repeat(*first_length));

                arrow.replace_range(
                    *second..(*second + *second_length),
                    &"^".repeat(*second_length),
                );

                write!(
                    f,
                    r#"duplicate parameter name: '{name}'

    Route: {route}
           {arrow}

tip: Parameter names must be unique within a route"#
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

tip: Constraint names must not contain the characters: ':', '*', '{{', '}}', '(', ')', '/'"#
                )
            }

            Self::TouchingParameters {
                route,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r#"touching parameters

    Route: {route}
           {arrow}

tip: Touching parameters are not supported"#
                )
            }
        }
    }
}

impl From<EncodingError> for PathRouteError {
    fn from(error: EncodingError) -> Self {
        Self::EncodingError(error)
    }
}
