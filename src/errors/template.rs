use alloc::{fmt, string::String};
use core::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum TemplateError {
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
    ///     template: "abc".to_owned(),
    /// };
    ///
    /// let display = r"
    /// missing leading slash
    ///
    ///     Template: abc
    ///
    /// help: Templates must begin with '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    MissingLeadingSlash {
        /// The template missing a leading slash.
        template: String,
    },

    /// Empty angles were found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::EmptyAngles {
    ///     template: "/<>".to_owned(),
    ///     position: 1,
    /// };
    ///
    /// let display = r"
    /// empty angles
    ///
    ///     Template: /<>
    ///                ^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyAngles {
        /// The template containing empty angles.
        template: String,
        /// The position of the first empty angle.
        position: usize,
    },

    /// An unbalanced angle was found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::UnbalancedAngle {
    ///     template: "/<".to_owned(),
    ///     position: 1,
    /// };
    ///
    /// let display = r"
    /// unbalanced angle
    ///
    ///     Template: /<
    ///                ^
    ///
    /// help: Each '<' must have a matching '>'
    ///
    /// try:
    ///     - Add the missing closing angle
    ///     - Use '\<' and '\>' to represent literal angles
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    UnbalancedAngle {
        /// The template containing an unbalanced angle.
        template: String,
        /// The position of the unbalanced angle.
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
    ///     template: "/()".to_owned(),
    ///     position: 1,
    /// };
    ///
    /// let display = r"
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
    ///     template: "/(".to_owned(),
    ///     position: 1,
    /// };
    ///
    /// let display = r"
    /// unbalanced parenthesis
    ///
    ///     Template: /(
    ///                ^
    ///
    /// help: Each '(' must have a matching ')'
    ///
    /// try:
    ///     - Add the missing closing parenthesis
    ///     - Use '\(' and '\)' to represent literal parentheses
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
    ///     template: "/<:>".to_owned(),
    ///     start: 1,
    ///     length: 3,
    /// };
    ///
    /// let display = r"
    /// empty parameter name
    ///
    ///     Template: /<:>
    ///                ^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyParameter {
        /// The template containing an empty parameter.
        template: String,
        /// The position of the opening angle of the empty name parameter.
        start: usize,
        /// The length of the parameter (including angles).
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
    ///     template: "/<a/b>".to_owned(),
    ///     name: "a/b".to_owned(),
    ///     start: 1,
    ///     length: 5,
    /// };
    ///
    /// let display = r"
    /// invalid parameter name: 'a/b'
    ///
    ///     Template: /<a/b>
    ///                ^^^^^
    ///
    /// help: Parameter names must not contain the characters: ':', '*', '<', '>', '(', ')', '{', '}', '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidParameter {
        /// The template containing an invalid parameter.
        template: String,
        /// The invalid parameter name.
        name: String,
        /// The position of the opening angle of the invalid name parameter.
        start: usize,
        /// The length of the parameter (including angles).
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
    ///     template: "/<id>/<id>".to_owned(),
    ///     name: "id".to_owned(),
    ///     first: 1,
    ///     first_length: 4,
    ///     second: 6,
    ///     second_length: 4,
    /// };
    ///
    /// let display = r"
    /// duplicate parameter name: 'id'
    ///
    ///     Template: /<id>/<id>
    ///                ^^^^ ^^^^
    ///
    /// help: Parameter names must be unique within a template
    ///
    /// try:
    ///     - Rename one of the parameters to be unique
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    DuplicateParameter {
        /// The template containing duplicate parameters.
        template: String,
        /// The duplicated parameter name.
        name: String,
        /// The position of the opening angle of the first occurrence.
        first: usize,
        /// The length of the first parameter (including angles).
        first_length: usize,
        /// The position of the opening angle of the second occurrence.
        second: usize,
        /// The length of the second parameter (including angles).
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
    ///     template: "/<*>".to_owned(),
    ///     start: 1,
    ///     length: 3,
    /// };
    ///
    /// let display = r"
    /// empty wildcard name
    ///
    ///     Template: /<*>
    ///                ^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyWildcard {
        /// The template containing an empty wildcard parameter.
        template: String,
        /// The position of the opening angle of the empty wildcard parameter.
        start: usize,
        /// The length of the parameter (including angles).
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
    ///     template: "/<a:>".to_owned(),
    ///     start: 1,
    ///     length: 4,
    /// };
    ///
    /// let display = r"
    /// empty constraint name
    ///
    ///     Template: /<a:>
    ///                ^^^^
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    EmptyConstraint {
        /// The template containing an empty constraint.
        template: String,
        /// The position of the opening angle of the empty constraint parameter.
        start: usize,
        /// The length of the parameter (including angles).
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
    ///     template: "/<a:b/c>".to_owned(),
    ///     name: "b/c".to_owned(),
    ///     start: 1,
    ///     length: 7,
    /// };
    ///
    /// let display = r"
    /// invalid constraint name: 'b/c'
    ///
    ///     Template: /<a:b/c>
    ///                ^^^^^^^
    ///
    /// help: Constraint names must not contain the characters: ':', '*', '<', '>', '(', ')', '{', '}', '/'
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    InvalidConstraint {
        /// The template containing an invalid constraint.
        template: String,
        /// The invalid constraint name.
        name: String,
        /// The position of the opening angle of the invalid constraint parameter.
        start: usize,
        /// The length of the parameter (including angles).
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
    ///     template: "/<a><b>".to_owned(),
    ///     start: 1,
    ///     length: 6,
    /// };
    ///
    /// let display = r"
    /// touching parameters
    ///
    ///     Template: /<a><b>
    ///                ^^^^^^
    ///
    /// help: Parameters must be separated by at least one part
    ///
    /// try:
    ///     - Add a part between the parameters
    ///     - Combine the parameters if they represent a single value
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    TouchingParameters {
        /// The template containing touching parameters.
        template: String,
        /// The position of the first opening angle.
        start: usize,
        /// The combined length of both parameters (including angles).
        length: usize,
    },
}

impl Error for TemplateError {}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty template"),

            Self::MissingLeadingSlash { template } => {
                write!(
                    f,
                    r"missing leading slash

    Template: {template}

help: Templates must begin with '/'"
                )
            }

            Self::EmptyAngles { template, position } => {
                let arrow = " ".repeat(*position) + "^^";
                write!(
                    f,
                    r"empty angles

    Template: {template}
              {arrow}"
                )
            }

            Self::UnbalancedAngle { template, position } => {
                let arrow = " ".repeat(*position) + "^";
                write!(
                    f,
                    r"unbalanced angle

    Template: {template}
              {arrow}

help: Each '<' must have a matching '>'

try:
    - Add the missing closing angle
    - Use '\<' and '\>' to represent literal angles"
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

help: Each '(' must have a matching ')'

try:
    - Add the missing closing parenthesis
    - Use '\(' and '\)' to represent literal parentheses"
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
                name,
                start,
                length,
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"invalid parameter name: '{name}'

    Template: {template}
              {arrow}

help: Parameter names must not contain the characters: ':', '*', '<', '>', '(', ')', '{{', '}}', '/'"
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

help: Parameter names must be unique within a template

try:
    - Rename one of the parameters to be unique"
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
                name,
                start,
                length,
                ..
            } => {
                let arrow = " ".repeat(*start) + &"^".repeat(*length);
                write!(
                    f,
                    r"invalid constraint name: '{name}'

    Template: {template}
              {arrow}

help: Constraint names must not contain the characters: ':', '*', '<', '>', '(', ')', '{{', '}}', '/'"
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

help: Parameters must be separated by at least one part

try:
    - Add a part between the parameters
    - Combine the parameters if they represent a single value"
                )
            }
        }
    }
}
