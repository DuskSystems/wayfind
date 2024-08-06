use crate::node::NodeData;
use smallvec::SmallVec;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
pub struct Match<'a, T> {
    pub data: &'a NodeData<T>,
    pub parameters: SmallVec<[Parameter<'a>; 8]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter<'a> {
    pub key: &'a str,
    pub value: &'a str,
}
