use std::fmt::{self, Debug};

pub trait Constraint {
    const NAME: &'static str;

    fn check(segment: &str) -> bool;
}

#[derive(Clone)]
pub struct NodeConstraint {
    pub name: &'static str,
    pub check: fn(&str) -> bool,
}

impl Debug for NodeConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for NodeConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && std::ptr::eq(self.check as *const (), other.check as *const ())
    }
}

impl Eq for NodeConstraint {}
