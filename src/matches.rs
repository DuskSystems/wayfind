use crate::node::NodeData;
use smallvec::SmallVec;
use std::{fmt::Debug, sync::Arc};

#[derive(Debug, Eq, PartialEq)]
pub struct Match<'a, T> {
    pub data: &'a NodeData<T>,
    pub parameters: SmallVec<[Parameter; 4]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    pub key: Arc<str>,
    pub value: Arc<str>,
}
