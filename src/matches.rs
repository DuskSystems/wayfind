use crate::node::NodeData;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
pub struct Match<'k, 'v, T> {
    pub data: &'k NodeData<T>,
    pub parameters: Vec<Parameter<'k, 'v>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter<'k, 'v> {
    pub key: &'k [u8],
    pub value: &'v [u8],
}
