use crate::node::NodeData;
use alloc::vec::Vec;
use core::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
pub struct Match<'a, T> {
    pub data: &'a NodeData<T>,
    pub parameters: Vec<Parameter<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter<'a> {
    pub key: &'a [u8],
    pub value: &'a [u8],
}
