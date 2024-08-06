use crate::{errors::parts::PartsError, node::NodeConstraint, parts::Parts};

pub struct Route<'a> {
    pub path: &'a str,
    pub parts: Parts<'a>,
    pub constraints: Vec<(&'a str, NodeConstraint)>,
}

impl<'a> Route<'a> {
    pub fn new(path: &'a str, constraints: Vec<(&'a str, NodeConstraint)>) -> Result<Self, PartsError> {
        Ok(Self {
            path,
            parts: Parts::new(path.as_bytes())?,
            constraints,
        })
    }
}
