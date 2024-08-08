use regex::Regex;
use std::fmt::Debug;

#[derive(Clone)]
pub enum ParameterConstraint {
    Regex(Regex),
    // TODO: Consider casting this to a &str ahead of time?
    Function(fn(&str) -> bool),
}

impl Debug for ParameterConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regex(regex) => write!(f, "ParameterConstraint::Regex({})", regex.as_str()),
            Self::Function(_) => write!(f, "ParameterConstraint::Function"),
        }
    }
}

impl PartialEq for ParameterConstraint {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Regex(left), Self::Regex(right)) => left.as_str() == right.as_str(),
            (Self::Function(left), Self::Function(right)) => std::ptr::eq(left, right),
            _ => false,
        }
    }
}

impl Eq for ParameterConstraint {}
