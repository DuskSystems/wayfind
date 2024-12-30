use crate::errors::EncodingError;
use std::{error::Error, fmt::Display};

/// Errors relating to malformed authorities.
#[derive(Debug, PartialEq, Eq)]
pub enum TemplateError {
    /// A [`EncodingError`] that occurred during the decoding.
    Encoding(EncodingError),

    /// The authority is empty.
    Empty,

    /// Empty braces were found in the authority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_authority::errors::TemplateError;
    ///
    /// let error = TemplateError::EmptyBraces {
    ///     authority: "{}".to_string(),
    ///     position: 0,
    /// };
    ///
    /// let display = "
    /// empty braces
    ///
    ///     Authority: {}
    ///                ^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyBraces {
        /// The authority containing empty braces.
        authority: String,
        /// The position of the first empty brace.
        position: usize,
    },

    /// An unbalanced brace was found in the authority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_authority::errors::TemplateError;
    ///
    /// let error = TemplateError::UnbalancedBrace {
    ///     authority: "{".to_string(),
    ///     position: 0,
    /// };
    ///
    /// let display = "
    /// unbalanced brace
    ///
    ///     Authority: {
    ///                ^
    ///
    /// tip: Use '\\{' and '\\}' to represent literal '{' and '}' characters in the authority
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    UnbalancedBrace {
        /// The authority containing an unbalanced brace.
        authority: String,
        /// The position of the unbalanced brace.
        position: usize,
    },

    /// An empty parameter name was found in the authority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_authority::errors::TemplateError;
    ///
    /// let error = TemplateError::EmptyParameter {
    ///     authority: "{:}".to_string(),
    ///     start: 0,
    ///     length: 3,
    /// };
    ///
    /// let display = "
    /// empty parameter name
    ///
    ///     Authority: {:}
    ///                ^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyParameter {
        /// The authority containing an empty parameter.
        authority: String,
        /// The position of the opening brace of the empty name parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An invalid parameter name was found in the authority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_authority::errors::TemplateError;
    ///
    /// let error = TemplateError::InvalidParameter {
    ///     authority: "{a.b}".to_string(),
    ///     name: "a.b".to_string(),
    ///     start: 0,
    ///     length: 5,
    /// };
    ///
    /// let display = "
    /// invalid parameter name
    ///
    ///     Authority: {a.b}
    ///                ^^^^^
    ///
    /// tip: Parameter names must not contain the characters: ':', '*', '{', '}', '.'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidParameter {
        /// The authority containing an invalid parameter.
        authority: String,
        /// The invalid parameter name.
        name: String,
        /// The position of the opening brace of the invalid name parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// A duplicate parameter name was found in the authority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_authority::errors::TemplateError;
    ///
    /// let error = TemplateError::DuplicateParameter {
    ///     authority: "{id}.{id}".to_string(),
    ///     name: "id".to_string(),
    ///     first: 0,
    ///     first_length: 4,
    ///     second: 5,
    ///     second_length: 4,
    /// };
    ///
    /// let display = "
    /// duplicate parameter name: 'id'
    ///
    ///     Authority: {id}.{id}
    ///                ^^^^ ^^^^
    ///
    /// tip: Parameter names must be unique within an authority
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    DuplicateParameter {
        /// The authority containing duplicate parameters.
        authority: String,
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

    /// A wildcard parameter with no name was found in the authority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_authority::errors::TemplateError;
    ///
    /// let error = TemplateError::EmptyWildcard {
    ///     authority: "{*}".to_string(),
    ///     start: 0,
    ///     length: 3,
    /// };
    ///
    /// let display = "
    /// empty wildcard name
    ///
    ///     Authority: {*}
    ///                ^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyWildcard {
        /// The authority containing an empty wildcard parameter.
        authority: String,
        /// The position of the opening brace of the empty wildcard parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An empty constraint name was found in the authority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_authority::errors::TemplateError;
    ///
    /// let error = TemplateError::EmptyConstraint {
    ///     authority: "{a:}".to_string(),
    ///     start: 0,
    ///     length: 4,
    /// };
    ///
    /// let display = "
    /// empty constraint name
    ///
    ///     Authority: {a:}
    ///                ^^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyConstraint {
        /// The authority containing an empty constraint.
        authority: String,
        /// The position of the opening brace of the empty constraint parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An invalid constraint name was found in the authority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_authority::errors::TemplateError;
    ///
    /// let error = TemplateError::InvalidConstraint {
    ///     authority: "{a:b/c}".to_string(),
    ///     name: "b/c".to_string(),
    ///     start: 0,
    ///     length: 7,
    /// };
    ///
    /// let display = "
    /// invalid constraint name
    ///
    ///     Authority: {a:b/c}
    ///                ^^^^^^^
    ///
    /// tip: Constraint names must not contain the characters: ':', '*', '{', '}', '.'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidConstraint {
        /// The authority containing an invalid constraint.
        authority: String,
        /// The invalid constraint name.
        name: String,
        /// The position of the opening brace of the invalid constraint parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// Two parameters side by side were found in the authority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind_authority::errors::TemplateError;
    ///
    /// let error = TemplateError::TouchingParameters {
    ///     authority: "{a}{b}".to_string(),
    ///     start: 0,
    ///     length: 6,
    /// };
    ///
    /// let display = "
    /// touching parameters
    ///
    ///     Authority: {a}{b}
    ///                ^^^^^^
    ///
    /// tip: Touching parameters are not supported
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    TouchingParameters {
        /// The authority containing touching parameters.
        authority: String,
        /// The position of the first opening brace.
        start: usize,
        /// The combined length of both parameters (including braces).
        length: usize,
    },
}

impl Error for TemplateError {}

impl Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encoding(error) => error.fmt(f),
            Self::Empty => write!(f, "empty authority"),

            Self::EmptyBraces {
                authority,
                position,
            } => {
                let arrow = " ".repeat(*position) + "^^";
                write!(
                    f,
                    r"empty braces

    Authority: {authority}
               {arrow}"
                )
            }

            Self::UnbalancedBrace {
                authority,
                position,
            } => {
                let arrow = " ".repeat(*position) + "^";
                write!(
                    f,
                    r"unbalanced brace

    Authority: {authority}
               {arrow}

tip: Use '\{{' and '\}}' to represent literal '{{' and '}}' characters in the authority"
                )
            }

            Self::EmptyParameter {
                authority,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"empty parameter name

    Authority: {authority}
               {arrow}"
                )
            }

            Self::InvalidParameter {
                authority,
                start,
                length,
                ..
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"invalid parameter name

    Authority: {authority}
               {arrow}

tip: Parameter names must not contain the characters: ':', '*', '{{', '}}', '.'"
                )
            }

            Self::DuplicateParameter {
                authority,
                name,
                first,
                first_length,
                second,
                second_length,
            } => {
                let mut arrow = " ".repeat(authority.len());

                arrow.replace_range(*first..(*first + *first_length), &"^".repeat(*first_length));

                arrow.replace_range(
                    *second..(*second + *second_length),
                    &"^".repeat(*second_length),
                );

                write!(
                    f,
                    r"duplicate parameter name: '{name}'

    Authority: {authority}
               {arrow}

tip: Parameter names must be unique within an authority"
                )
            }

            Self::EmptyWildcard {
                authority,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"empty wildcard name

    Authority: {authority}
               {arrow}"
                )
            }

            Self::EmptyConstraint {
                authority,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"empty constraint name

    Authority: {authority}
               {arrow}"
                )
            }

            Self::InvalidConstraint {
                authority,
                start,
                length,
                ..
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"invalid constraint name

    Authority: {authority}
               {arrow}

tip: Constraint names must not contain the characters: ':', '*', '{{', '}}', '.'"
                )
            }

            Self::TouchingParameters {
                authority,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"touching parameters

    Authority: {authority}
               {arrow}

tip: Touching parameters are not supported"
                )
            }
        }
    }
}

impl From<EncodingError> for TemplateError {
    fn from(error: EncodingError) -> Self {
        Self::Encoding(error)
    }
}
