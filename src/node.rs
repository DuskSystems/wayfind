use std::sync::Arc;

#[cfg(regex)]
use regex::bytes::Regex;

pub mod delete;
pub mod display;
pub mod insert;
pub mod matches;

#[derive(Clone, Debug)]
pub enum NodeKind {
    Root,
    Static,
    #[cfg(regex)]
    Regex(Regex),
    Dynamic,
    Wildcard,
    EndWildcard,
}

impl PartialEq for NodeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Root, Self::Root)
            | (Self::Static, Self::Static)
            | (Self::Dynamic, Self::Dynamic)
            | (Self::Wildcard, Self::Wildcard)
            | (Self::EndWildcard, Self::EndWildcard) => true,
            #[cfg(regex)]
            (Self::Regex(r1), Self::Regex(r2)) => r1.as_str() == r2.as_str(),
            _ => false,
        }
    }
}

impl Eq for NodeKind {}

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

    pub static_children: Vec<Node<T>>,
    #[cfg(regex)]
    pub regex_children: Vec<Node<T>>,
    pub dynamic_children: Vec<Node<T>>,
    pub wildcard_children: Vec<Node<T>>,
    pub end_wildcard: Option<Box<Node<T>>>,

    // TODO: Come up with a better names.
    #[cfg(regex)]
    pub quick_regex: bool,
    pub quick_dynamic: bool,
}
