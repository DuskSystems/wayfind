use crate::node::NodeConstraint;

pub struct Route<'a> {
    pub path: &'a str,
    pub constraints: Vec<(&'a str, NodeConstraint)>,
}

impl<'a> Route<'a> {
    #[must_use]
    pub const fn new(path: &'a str) -> Self {
        Self {
            path,
            constraints: vec![],
        }
    }
}

impl<'a> From<&'a str> for Route<'a> {
    fn from(path: &'a str) -> Self {
        Self::new(path)
    }
}

impl<'a> From<(&'a str, Vec<(&'a str, NodeConstraint)>)> for Route<'a> {
    fn from((path, constraints): (&'a str, Vec<(&'a str, NodeConstraint)>)) -> Self {
        Self { path, constraints }
    }
}
