use std::{error::Error, fmt::Display};

use super::EncodingError;

/// Errors relating to malformed routes.
#[derive(Debug, PartialEq, Eq)]
pub enum TemplateError {
    /// A [`EncodingError`] that occurred during the decoding.
    Encoding(EncodingError),

    /// The template is empty.
    Empty,

    /// The template must start with '/'.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::MissingLeadingSlash {
    ///     template: "abc".to_string(),
    /// };
    ///
    /// let display = "
    /// missing leading slash
    ///
    ///     Template: abc
    ///
    /// tip: Routes must begin with '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    MissingLeadingSlash {
        /// The template missing a leading slash.
        template: String,
    },

    /// Empty braces were found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::EmptyBraces {
    ///     template: "/{}".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let display = "
    /// empty braces
    ///
    ///     Template: /{}
    ///                ^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyBraces {
        /// The template containing empty braces.
        template: String,
        /// The position of the first empty brace.
        position: usize,
    },

    /// An unbalanced brace was found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::UnbalancedBrace {
    ///     template: "/{".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let display = "
    /// unbalanced brace
    ///
    ///     Template: /{
    ///                ^
    ///
    /// tip: Use '\\{' and '\\}' to represent literal '{' and '}' characters in the template
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    UnbalancedBrace {
        /// The template containing an unbalanced brace.
        template: String,
        /// The position of the unbalanced brace.
        position: usize,
    },

    /// Empty parentheses were found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::EmptyParentheses {
    ///     template: "/()".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let display = "
    /// empty parentheses
    ///
    ///     Template: /()
    ///                ^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyParentheses {
        /// The template containing empty parentheses.
        template: String,
        /// The position of the first empty parenthesis.
        position: usize,
    },

    /// An unbalanced parenthesis was found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::UnbalancedParenthesis {
    ///     template: "/(".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let display = "
    /// unbalanced parenthesis
    ///
    ///     Template: /(
    ///                ^
    ///
    /// tip: Use '\\(' and '\\)' to represent literal '(' and ')' characters in the template
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    UnbalancedParenthesis {
        /// The template containing an unbalanced parenthesis.
        template: String,
        /// The position of the unbalanced parenthesis.
        position: usize,
    },

    /// An empty parameter name was found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::EmptyParameter {
    ///     template: "/{:}".to_string(),
    ///     start: 1,
    ///     length: 3,
    /// };
    ///
    /// let display = "
    /// empty parameter name
    ///
    ///     Template: /{:}
    ///                ^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyParameter {
        /// The template containing an empty parameter.
        template: String,
        /// The position of the opening brace of the empty name parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An invalid parameter name was found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::InvalidParameter {
    ///     template: "/{a/b}".to_string(),
    ///     name: "a/b".to_string(),
    ///     start: 1,
    ///     length: 5,
    /// };
    ///
    /// let display = "
    /// invalid parameter name
    ///
    ///     Template: /{a/b}
    ///                ^^^^^
    ///
    /// tip: Parameter names must not contain the characters: ':', '*', '{', '}', '(', ')', '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidParameter {
        /// The template containing an invalid parameter.
        template: String,
        /// The invalid parameter name.
        name: String,
        /// The position of the opening brace of the invalid name parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// A duplicate parameter name was found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::DuplicateParameter {
    ///     template: "/{id}/{id}".to_string(),
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
    ///     Template: /{id}/{id}
    ///                ^^^^ ^^^^
    ///
    /// tip: Parameter names must be unique within a template
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    DuplicateParameter {
        /// The template containing duplicate parameters.
        template: String,
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

    /// A wildcard parameter with no name was found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::EmptyWildcard {
    ///     template: "/{*}".to_string(),
    ///     start: 1,
    ///     length: 3,
    /// };
    ///
    /// let display = "
    /// empty wildcard name
    ///
    ///     Template: /{*}
    ///                ^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyWildcard {
        /// The template containing an empty wildcard parameter.
        template: String,
        /// The position of the opening brace of the empty wildcard parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An empty constraint name was found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::EmptyConstraint {
    ///     template: "/{a:}".to_string(),
    ///     start: 1,
    ///     length: 4,
    /// };
    ///
    /// let display = "
    /// empty constraint name
    ///
    ///     Template: /{a:}
    ///                ^^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyConstraint {
        /// The template containing an empty constraint.
        template: String,
        /// The position of the opening brace of the empty constraint parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// An invalid constraint name was found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::InvalidConstraint {
    ///     template: "/{a:b/c}".to_string(),
    ///     name: "b/c".to_string(),
    ///     start: 1,
    ///     length: 7,
    /// };
    ///
    /// let display = "
    /// invalid constraint name
    ///
    ///     Template: /{a:b/c}
    ///                ^^^^^^^
    ///
    /// tip: Constraint names must not contain the characters: ':', '*', '{', '}', '(', ')', '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidConstraint {
        /// The template containing an invalid constraint.
        template: String,
        /// The invalid constraint name.
        name: String,
        /// The position of the opening brace of the invalid constraint parameter.
        start: usize,
        /// The length of the parameter (including braces).
        length: usize,
    },

    /// Two parameters side by side were found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::TouchingParameters {
    ///     template: "/{a}{b}".to_string(),
    ///     start: 1,
    ///     length: 6,
    /// };
    ///
    /// let display = "
    /// touching parameters
    ///
    ///     Template: /{a}{b}
    ///                ^^^^^^
    ///
    /// tip: Touching parameters are not supported
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    TouchingParameters {
        /// The template containing touching parameters.
        template: String,
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
            Self::Empty => write!(f, "empty template"),

            Self::MissingLeadingSlash { template } => {
                write!(
                    f,
                    r"missing leading slash

    Template: {template}

tip: Routes must begin with '/'"
                )
            }

            Self::EmptyBraces { template, position } => {
                let arrow = " ".repeat(*position) + "^^";
                write!(
                    f,
                    r"empty braces

    Template: {template}
              {arrow}"
                )
            }

            Self::UnbalancedBrace { template, position } => {
                let arrow = " ".repeat(*position) + "^";
                write!(
                    f,
                    r"unbalanced brace

    Template: {template}
              {arrow}

tip: Use '\{{' and '\}}' to represent literal '{{' and '}}' characters in the template"
                )
            }

            Self::EmptyParentheses { template, position } => {
                let arrow = " ".repeat(*position) + "^^";
                write!(
                    f,
                    r"empty parentheses

    Template: {template}
              {arrow}"
                )
            }

            Self::UnbalancedParenthesis { template, position } => {
                let arrow = " ".repeat(*position) + "^";
                write!(
                    f,
                    r"unbalanced parenthesis

    Template: {template}
              {arrow}

tip: Use '\(' and '\)' to represent literal '(' and ')' characters in the template"
                )
            }

            Self::EmptyParameter {
                template,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"empty parameter name

    Template: {template}
              {arrow}"
                )
            }

            Self::InvalidParameter {
                template,
                start,
                length,
                ..
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"invalid parameter name

    Template: {template}
              {arrow}

tip: Parameter names must not contain the characters: ':', '*', '{{', '}}', '(', ')', '/'"
                )
            }

            Self::DuplicateParameter {
                template,
                name,
                first,
                first_length,
                second,
                second_length,
            } => {
                let mut arrow = " ".repeat(template.len());

                arrow.replace_range(*first..(*first + *first_length), &"^".repeat(*first_length));

                arrow.replace_range(
                    *second..(*second + *second_length),
                    &"^".repeat(*second_length),
                );

                write!(
                    f,
                    r"duplicate parameter name: '{name}'

    Template: {template}
              {arrow}

tip: Parameter names must be unique within a template"
                )
            }

            Self::EmptyWildcard {
                template,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"empty wildcard name

    Template: {template}
              {arrow}"
                )
            }

            Self::EmptyConstraint {
                template,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"empty constraint name

    Template: {template}
              {arrow}"
                )
            }

            Self::InvalidConstraint {
                template,
                start,
                length,
                ..
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"invalid constraint name

    Template: {template}
              {arrow}

tip: Constraint names must not contain the characters: ':', '*', '{{', '}}', '(', ')', '/'"
                )
            }

            Self::TouchingParameters {
                template,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"touching parameters

    Template: {template}
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
