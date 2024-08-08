use crate::constraints::{parameter::ParameterConstraint, request::RequestConstraint};
use std::{fmt::Debug, sync::Arc};

pub mod delete;
pub mod display;
pub mod insert;
pub mod matches;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Root,
    Static,
    Dynamic,
    Wildcard,
    EndWildcard,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeData<T> {
    pub path: Arc<str>,
    pub value: T,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<T, R> {
    pub kind: NodeKind,

    pub prefix: Vec<u8>,
    pub data: Option<NodeData<T>>,

    pub parameter_constraints: Vec<ParameterConstraint>,
    pub request_constraints: Vec<RequestConstraint<R>>,

    pub static_children: Vec<Node<T, R>>,
    pub dynamic_children: Vec<Node<T, R>>,
    pub wildcard_children: Vec<Node<T, R>>,
    pub end_wildcard_children: Vec<Node<T, R>>,

    // TODO: Come up with a better names.
    pub quick_dynamic: bool,
}
