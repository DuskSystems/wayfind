use crate::node::NodeData;
use smallvec::SmallVec;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
pub struct Match<'a, T> {
    pub data: &'a NodeData<'a, T>,
    pub parameters: SmallVec<[Parameter<'a>; 4]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter<'a> {
    pub key: &'a [u8],
    pub value: &'a [u8],
}
