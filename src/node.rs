use regex::bytes::Regex;
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

#[derive(Clone)]
pub enum NodeConstraint {
    Regex(Regex),
}

impl Debug for NodeConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regex(regex) => write!(f, "{}", regex.as_str()),
        }
    }
}

impl PartialEq for NodeConstraint {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Regex(left), Self::Regex(right)) => left.as_str() == right.as_str(),
        }
    }
}

impl Eq for NodeConstraint {}

impl From<Regex> for NodeConstraint {
    fn from(regex: Regex) -> Self {
        Self::Regex(regex)
    }
}

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
