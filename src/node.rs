use alloc::boxed::Box;
use alloc::vec::Vec;

use memchr::memmem::FinderRev;

use crate::node::flags::Flags;
use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};

mod conflict;
mod delete;
mod display;
mod find;
pub(crate) mod flags;
mod insert;
mod optimize;
pub(crate) mod search;

/// A pre-computed suffix needle for parameter matching.
#[derive(Clone, Debug)]
pub(crate) struct Suffix {
    pub needle: Box<[u8]>,
    pub finder: Box<FinderRev<'static>>,
}

/// Pre-computed reachable condition.
#[derive(Clone, Debug)]
pub(crate) enum Reachable {
    Suffix(Box<[u8]>),
    Contains { needle: Box<[u8]>, needle_id: usize },
    Flexible,
    End,
}

impl PartialEq for Reachable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Suffix(a), Self::Suffix(b))
            | (Self::Contains { needle: a, .. }, Self::Contains { needle: b, .. }) => a == b,
            (Self::Flexible, Self::Flexible) | (Self::End, Self::End) => true,
            _ => false,
        }
    }
}

impl Eq for Reachable {}

#[derive(Clone, Debug)]
pub(crate) struct NodeData {
    /// The key to the stored data in the router's slab.
    pub key: usize,

    /// This node's template.
    pub template: Box<str>,
}

/// Represents a node in the tree structure.
#[derive(Clone, Debug)]
pub(crate) struct Node<S> {
    /// Node ID.
    pub id: usize,

    /// The node's type-specific state.
    pub state: S,
    /// Optional data associated with this node.
    pub data: Option<NodeData>,

    pub static_children: Vec<Node<StaticState>>,
    pub dynamic_children: Vec<Node<DynamicState>>,
    pub wildcard_children: Vec<Node<WildcardState>>,
    pub end_wildcard: Option<Box<Node<EndWildcardState>>>,

    /// State flags.
    pub flags: Flags,
    /// Minimum bytes of remaining path needed for any match through this node.
    pub shortest: usize,
    /// Maximum bytes of remaining path for any match through this node.
    pub longest: usize,
    /// Exact suffixes the remaining path must end with for any match through this node.
    pub tails: Box<[Box<[u8]>]>,
    /// Fallback reachability checks when `tails` is empty.
    pub reachable: Box<[Reachable]>,
    /// Byte needles from static children, used for suffix-guided parameter matching.
    pub suffixes: Box<[Suffix]>,
}

#[cfg(target_pointer_width = "64")]
const _: () = {
    assert!(size_of::<NodeData>() == 24, "NodeData size");
    assert!(size_of::<Option<NodeData>>() == 24, "Option<NodeData> size");
    assert!(size_of::<Suffix>() == 24, "Suffix size");
    assert!(size_of::<Reachable>() == 32, "Reachable size");
    assert!(size_of::<Flags>() == 1, "Flags size");
};

impl<S> Node<S> {
    /// Creates a new empty node.
    #[must_use]
    pub(crate) fn new(state: S) -> Self {
        Self {
            id: 0,

            state,
            data: None,

            static_children: Vec::new(),
            dynamic_children: Vec::new(),
            wildcard_children: Vec::new(),
            end_wildcard: None,

            flags: Flags::default(),
            shortest: usize::MAX,
            longest: 0,
            tails: Box::default(),
            reachable: Box::default(),
            suffixes: Box::default(),
        }
    }
}
