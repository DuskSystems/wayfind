use alloc::string::String;
use core::error::Error;
use core::fmt;

/// Errors relating to template parsing.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TemplateError {
    /// The template is empty.
    Empty,

    /// The template must start with `/`.
    MissingLeadingSlash,

    /// An unbalanced angle bracket was found in the template.
    UnbalancedAngle,

    /// An empty parameter name was found in the template.
    EmptyParameter,

    /// An invalid parameter name was found in the template.
    InvalidParameter {
        /// The invalid parameter name.
        name: String,
    },

    /// A duplicate parameter name was found in the template.
    DuplicateParameter {
        /// The duplicated parameter name.
        name: String,
    },

    /// An empty wildcard name was found in the template.
    EmptyWildcard,

    /// Two parameters are directly adjacent without a separator.
    TouchingParameters,

    /// Too many parameters were found in a single segment.
    TooManyParameters,

    /// Too many non-trailing wildcards were found in the template.
    TooManyWildcards,
}

impl Error for TemplateError {}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty template"),
            Self::MissingLeadingSlash => write!(f, "missing leading slash"),
            Self::UnbalancedAngle => write!(f, "unbalanced angle bracket"),
            Self::EmptyParameter => write!(f, "empty parameter name"),
            Self::InvalidParameter { name } => write!(f, "invalid parameter name `{name}`"),
            Self::DuplicateParameter { name } => write!(f, "duplicate parameter name `{name}`"),
            Self::EmptyWildcard => write!(f, "empty wildcard name"),
            Self::TouchingParameters => {
                write!(f, "parameters must be separated by a static segment")
            }
            Self::TooManyParameters => write!(f, "only one parameter is allowed per segment"),
            Self::TooManyWildcards => write!(f, "only one wildcard is allowed per template"),
        }
    }
}
