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

pub trait Constraint {
    fn name() -> &'static str;
    fn check(segment: &str) -> bool;
}

#[derive(Clone)]
pub struct NodeConstraint {
    pub name: &'static str,
    pub check: fn(&str) -> bool,
}

impl Debug for NodeConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for NodeConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && std::ptr::eq(self.check as *const (), other.check as *const ())
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
    pub constraint: Option<NodeConstraint>,

    pub static_children: Vec<Node<T>>,
    pub dynamic_children: Vec<Node<T>>,
    pub wildcard_children: Vec<Node<T>>,
    pub end_wildcard_children: Vec<Node<T>>,

    // TODO: Come up with a better names.
    pub quick_dynamic: bool,
}
