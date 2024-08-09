use std::{
    fmt::{self, Debug},
    sync::Arc,
};

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

#[derive(Clone)]
pub struct NodeConstraint(pub fn(&str) -> bool);

impl Debug for NodeConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeConstraint(<function>)")
    }
}

impl PartialEq for NodeConstraint {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0 as *const (), other.0 as *const ())
    }
}

impl Eq for NodeConstraint {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeData<T> {
    pub path: Arc<str>,
    pub value: T,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<T> {
    pub kind: NodeKind,

    pub prefix: Vec<u8>,
    pub data: Option<NodeData<T>>,
    pub constraints: Vec<NodeConstraint>,

    pub static_children: Vec<Node<T>>,
    pub dynamic_children: Vec<Node<T>>,
    pub wildcard_children: Vec<Node<T>>,
    pub end_wildcard_children: Vec<Node<T>>,

    // TODO: Come up with a better names.
    pub quick_dynamic: bool,
}
