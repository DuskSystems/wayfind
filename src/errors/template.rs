use alloc::fmt;
use alloc::string::String;
use alloc::vec::Vec;
use core::error::Error;

#[derive(Eq, PartialEq, Debug)]
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

    /// An empty parameter name was found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::EmptyParameter {
    ///     template: "/<>".to_owned(),
    ///     start: 1,
    ///     length: 2,
    /// };
    ///
    /// let display = r"
    /// empty parameter name
    ///
    ///     Template: /<>
    ///                ^^
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
    /// help: Parameter names must not contain the characters: '*', '<', '>', '/'
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

    /// Too many parameters were found in a single segment.
    ///
    /// At most 2 parameters are allowed per segment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::TooManyInline {
    ///     template: "/<a>-<b>-<c>".to_owned(),
    ///     parameters: vec![(1, 3), (5, 3), (9, 3)],
    /// };
    ///
    /// let display = r"
    /// too many parameters in segment
    ///
    ///     Template: /<a>-<b>-<c>
    ///                ^^^ ^^^ ^^^
    ///
    /// help: At most 2 parameters are allowed per segment
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    TooManyInline {
        /// The template containing too many parameters.
        template: String,
        /// Positions and lengths of each parameter (start, length).
        parameters: Vec<(usize, usize)>,
    },

    /// Too many wildcards were found in the template.
    ///
    /// At most 2 wildcards are allowed per template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::TooManyWildcards {
    ///     template: "/<*a>/x/<*b>/y/<*c>".to_owned(),
    ///     parameters: vec![(1, 4), (8, 4), (15, 4)],
    /// };
    ///
    /// let display = r"
    /// too many wildcards
    ///
    ///     Template: /<*a>/x/<*b>/y/<*c>
    ///                ^^^^   ^^^^   ^^^^
    ///
    /// help: At most 2 wildcards are allowed per template
    /// ";
    ///
    /// assert_eq!(error.to_string(), display.trim());
    /// ```
    TooManyWildcards {
        /// The template containing too many wildcards.
        template: String,
        /// Positions and lengths of each wildcard (start, length).
        parameters: Vec<(usize, usize)>,
    },
}

impl Error for TemplateError {}

impl fmt::Display for TemplateError {
    #[allow(clippy::too_many_lines)]
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

            Self::UnbalancedAngle { template, position } => {
                let arrow = " ".repeat(*position) + "^";
                write!(
                    f,
                    r"unbalanced angle

    Template: {template}
              {arrow}

help: Each '<' must have a matching '>'

try:
    - Add the missing closing angle"
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

help: Parameter names must not contain the characters: '*', '<', '>', '/'"
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

            Self::TooManyInline {
                template,
                parameters,
            } => {
                let mut arrow = " ".repeat(template.len());
                for (start, length) in parameters {
                    arrow.replace_range(*start..(*start + *length), &"^".repeat(*length));
                }

                write!(
                    f,
                    r"too many parameters in segment

    Template: {template}
              {arrow}

help: At most 2 parameters are allowed per segment"
                )
            }

            Self::TooManyWildcards {
                template,
                parameters,
            } => {
                let mut arrow = " ".repeat(template.len());
                for (start, length) in parameters {
                    arrow.replace_range(*start..(*start + *length), &"^".repeat(*length));
                }

                write!(
                    f,
                    r"too many wildcards

    Template: {template}
              {arrow}

help: At most 2 wildcards are allowed per template"
                )
            }
        }
    }
}
