use alloc::fmt;
use alloc::string::String;
use alloc::vec::Vec;
use core::error::Error;
use core::ops::Range;

/// Errors relating to template parsing.
#[derive(Clone, Eq, PartialEq)]
pub enum TemplateError {
    /// The template is empty.
    Empty,

    /// The template must start with `/`.
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
    /// let display = "missing leading slash in `abc`";
    /// let debug = r"error: missing leading slash
    ///
    ///     abc
    ///     ━━━
    ///
    /// help: templates must begin with `/`";
    ///
    /// assert_eq!(error.to_string(), display);
    /// assert_eq!(format!("{error:?}"), debug);
    /// ```
    MissingLeadingSlash {
        /// The template missing a leading slash.
        template: String,
    },

    /// An unbalanced angle bracket was found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::UnbalancedAngle {
    ///     template: "/<id".to_owned(),
    ///     position: 1..2,
    /// };
    ///
    /// let display = "unbalanced angle bracket in `/<id`";
    /// let debug = r"error: unbalanced angle bracket
    ///
    ///     /<id
    ///      ━
    ///
    /// help: each `<` must have a matching `>`";
    ///
    /// assert_eq!(error.to_string(), display);
    /// assert_eq!(format!("{error:?}"), debug);
    /// ```
    UnbalancedAngle {
        /// The template containing an unbalanced angle bracket.
        template: String,
        /// The position of the unbalanced angle bracket.
        position: Range<usize>,
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
    ///     position: 1..3,
    /// };
    ///
    /// let display = "empty parameter name in `/<>`";
    /// let debug = r"error: empty parameter name
    ///
    ///     /<>
    ///      ━━
    ///
    /// help: provide a name between `<` and `>`";
    ///
    /// assert_eq!(error.to_string(), display);
    /// assert_eq!(format!("{error:?}"), debug);
    /// ```
    EmptyParameter {
        /// The template containing an empty parameter.
        template: String,
        /// The position of the empty parameter.
        position: Range<usize>,
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
    ///     position: 1..6,
    /// };
    ///
    /// let display = "invalid parameter name `a/b` in `/<a/b>`";
    /// let debug = r"error: invalid parameter name: `a/b`
    ///
    ///     /<a/b>
    ///      ━━━━━
    ///
    /// help: parameter names must not contain `*`, `<`, `>`, or `/`";
    ///
    /// assert_eq!(error.to_string(), display);
    /// assert_eq!(format!("{error:?}"), debug);
    /// ```
    InvalidParameter {
        /// The template containing an invalid parameter.
        template: String,
        /// The invalid parameter name.
        name: String,
        /// The position of the invalid parameter.
        position: Range<usize>,
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
    ///     original: 1..5,
    ///     duplicate: 6..10,
    /// };
    ///
    /// let display = "duplicate parameter name `id` in `/<id>/<id>`";
    /// let debug = r"error: duplicate parameter name: `id`
    ///
    ///     /<id>/<id>
    ///      ━━━━ ━━━━
    ///
    /// help: rename one of the parameters";
    ///
    /// assert_eq!(error.to_string(), display);
    /// assert_eq!(format!("{error:?}"), debug);
    /// ```
    DuplicateParameter {
        /// The template containing duplicate parameters.
        template: String,
        /// The duplicated parameter name.
        name: String,
        /// The position of the original parameter.
        original: Range<usize>,
        /// The position of the duplicate parameter.
        duplicate: Range<usize>,
    },

    /// An empty wildcard name was found in the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::EmptyWildcard {
    ///     template: "/<*>".to_owned(),
    ///     position: 1..4,
    /// };
    ///
    /// let display = "empty wildcard name in `/<*>`";
    /// let debug = r"error: empty wildcard name
    ///
    ///     /<*>
    ///      ━━━
    ///
    /// help: provide a name after `*`";
    ///
    /// assert_eq!(error.to_string(), display);
    /// assert_eq!(format!("{error:?}"), debug);
    /// ```
    EmptyWildcard {
        /// The template containing an empty wildcard.
        template: String,
        /// The position of the empty wildcard.
        position: Range<usize>,
    },

    /// Two parameters are directly adjacent without a separator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::TouchingParameters {
    ///     template: "/<a><b>".to_owned(),
    ///     first: "a".to_owned(),
    ///     second: "b".to_owned(),
    ///     position: 1..7,
    /// };
    ///
    /// let display = "touching parameters in `/<a><b>`";
    /// let debug = r"error: touching parameters `a` and `b`
    ///
    ///     /<a><b>
    ///      ━━━━━━
    ///
    /// help: parameters must be separated by at least one static segment";
    ///
    /// assert_eq!(error.to_string(), display);
    /// assert_eq!(format!("{error:?}"), debug);
    /// ```
    TouchingParameters {
        /// The template containing touching parameters.
        template: String,
        /// The name of the first parameter.
        first: String,
        /// The name of the second parameter.
        second: String,
        /// The combined position of both parameters.
        position: Range<usize>,
    },

    /// Too many parameters were found in a single segment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::errors::TemplateError;
    ///
    /// let error = TemplateError::TooManyParameters {
    ///     template: "/<a>-<b>".to_owned(),
    ///     positions: vec![1..4, 5..8],
    /// };
    ///
    /// let display = "too many parameters in segment in `/<a>-<b>`";
    /// let debug = r"error: too many parameters in segment
    ///
    ///     /<a>-<b>
    ///      ━━━ ━━━
    ///
    /// help: only one parameter is allowed per segment";
    ///
    /// assert_eq!(error.to_string(), display);
    /// assert_eq!(format!("{error:?}"), debug);
    /// ```
    TooManyParameters {
        /// The template containing too many parameters.
        template: String,
        /// The positions of each parameter in the segment.
        positions: Vec<Range<usize>>,
    },
}

impl Error for TemplateError {}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty template"),
            Self::MissingLeadingSlash { template } => {
                write!(f, "missing leading slash in `{template}`")
            }
            Self::UnbalancedAngle { template, .. } => {
                write!(f, "unbalanced angle bracket in `{template}`")
            }
            Self::EmptyParameter { template, .. } => {
                write!(f, "empty parameter name in `{template}`")
            }
            Self::InvalidParameter { template, name, .. } => {
                write!(f, "invalid parameter name `{name}` in `{template}`")
            }
            Self::DuplicateParameter { template, name, .. } => {
                write!(f, "duplicate parameter name `{name}` in `{template}`")
            }
            Self::EmptyWildcard { template, .. } => {
                write!(f, "empty wildcard name in `{template}`")
            }
            Self::TouchingParameters { template, .. } => {
                write!(f, "touching parameters in `{template}`")
            }
            Self::TooManyParameters { template, .. } => {
                write!(f, "too many parameters in segment in `{template}`")
            }
        }
    }
}

impl fmt::Debug for TemplateError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(
                f,
                "error: empty template

help: templates must not be empty"
            ),

            Self::MissingLeadingSlash { template } => {
                let underline = "━".repeat(template.len());
                write!(
                    f,
                    "error: missing leading slash

    {template}
    {underline}

help: templates must begin with `/`"
                )
            }

            Self::UnbalancedAngle { template, position } => {
                let underline = " ".repeat(position.start) + &"━".repeat(position.len());
                write!(
                    f,
                    "error: unbalanced angle bracket

    {template}
    {underline}

help: each `<` must have a matching `>`"
                )
            }

            Self::EmptyParameter { template, position } => {
                let underline = " ".repeat(position.start) + &"━".repeat(position.len());
                write!(
                    f,
                    "error: empty parameter name

    {template}
    {underline}

help: provide a name between `<` and `>`"
                )
            }

            Self::InvalidParameter {
                template,
                name,
                position,
            } => {
                let underline = " ".repeat(position.start) + &"━".repeat(position.len());
                write!(
                    f,
                    "error: invalid parameter name: `{name}`

    {template}
    {underline}

help: parameter names must not contain `*`, `<`, `>`, or `/`"
                )
            }

            Self::DuplicateParameter {
                template,
                name,
                original,
                duplicate,
            } => {
                let underline: String = (0..template.len())
                    .map(|index| {
                        if original.contains(&index) || duplicate.contains(&index) {
                            '━'
                        } else {
                            ' '
                        }
                    })
                    .collect();

                write!(
                    f,
                    "error: duplicate parameter name: `{name}`

    {template}
    {underline}

help: rename one of the parameters"
                )
            }

            Self::EmptyWildcard { template, position } => {
                let underline = " ".repeat(position.start) + &"━".repeat(position.len());
                write!(
                    f,
                    "error: empty wildcard name

    {template}
    {underline}

help: provide a name after `*`"
                )
            }

            Self::TouchingParameters {
                template,
                first,
                second,
                position,
            } => {
                let underline = " ".repeat(position.start) + &"━".repeat(position.len());
                write!(
                    f,
                    "error: touching parameters `{first}` and `{second}`

    {template}
    {underline}

help: parameters must be separated by at least one static segment"
                )
            }

            Self::TooManyParameters {
                template,
                positions,
            } => {
                let underline: String = (0..template.len())
                    .map(|index| {
                        if positions.iter().any(|p| p.contains(&index)) {
                            '━'
                        } else {
                            ' '
                        }
                    })
                    .collect();

                write!(
                    f,
                    "error: too many parameters in segment

    {template}
    {underline}

help: only one parameter is allowed per segment"
                )
            }
        }
    }
}
