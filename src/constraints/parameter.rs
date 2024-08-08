use regex::Regex;
use std::{cmp::Ordering, fmt::Debug};

#[derive(Clone)]
pub enum ParameterConstraint {
    Function(fn(&str) -> bool),
    Regex(Regex),
}

impl Debug for ParameterConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(_) => write!(f, "ParameterConstraint::Function"),
            Self::Regex(regex) => write!(f, "ParameterConstraint::Regex({})", regex.as_str()),
        }
    }
}

impl PartialEq for ParameterConstraint {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Function(left), Self::Function(right)) => std::ptr::eq(left, right),
            (Self::Regex(left), Self::Regex(right)) => left.as_str() == right.as_str(),
            _ => false,
        }
    }
}

impl Eq for ParameterConstraint {}

impl PartialOrd for ParameterConstraint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ParameterConstraint {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Function(_), Self::Regex(_)) => Ordering::Less,
            (Self::Regex(_), Self::Function(_)) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}
