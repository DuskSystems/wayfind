use crate::constraints::{parameter::ParameterConstraint, request::RequestConstraint};
use std::{cmp::Ordering, fmt::Debug, sync::Arc};

pub mod delete;
pub mod display;
pub mod insert;
pub mod matches;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Debug)]
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

impl<T, R> PartialEq for Node<T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.prefix == other.prefix
            && self.kind == other.kind
            && self.parameter_constraints == other.parameter_constraints
            && self.request_constraints == other.request_constraints
            && self.static_children == other.static_children
            && self.dynamic_children == other.dynamic_children
            && self.wildcard_children == other.wildcard_children
            && self.end_wildcard_children == other.end_wildcard_children
            && self.quick_dynamic == other.quick_dynamic
    }
}

impl<T, R> Eq for Node<T, R> {}

impl<T, R> PartialOrd for Node<T, R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, R> Ord for Node<T, R> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind
            .cmp(&other.kind)
            .then_with(|| {
                other
                    .parameter_constraints
                    .len()
                    .cmp(&self.parameter_constraints.len())
            })
            .then_with(|| self.prefix.cmp(&other.prefix))
    }
}
