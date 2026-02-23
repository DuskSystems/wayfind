use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::node::NodeData;

/// Root node of the tree.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RootState;

impl RootState {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl fmt::Display for RootState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

/// Static path segment bytes.
/// May not be valid UTF-8 due to multibyte splitting.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StaticState {
    pub first: u8,
    pub prefix: Vec<u8>,
}

impl StaticState {
    #[must_use]
    pub fn new(prefix: Vec<u8>) -> Self {
        Self {
            first: prefix.first().copied().unwrap_or(0),
            prefix,
        }
    }
}

impl fmt::Display for StaticState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.prefix))
    }
}

/// Dynamic parameter with its name.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DynamicState {
    pub name: String,
    pub suffixes: Vec<Box<[u8]>>,
}

impl DynamicState {
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self {
            name,
            suffixes: Vec::new(),
        }
    }
}

impl fmt::Display for DynamicState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.name)
    }
}

/// Wildcard parameter with its name.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct WildcardState {
    pub name: String,
    pub suffixes: Vec<Box<[u8]>>,
}

impl WildcardState {
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self {
            name,
            suffixes: Vec::new(),
        }
    }
}

impl fmt::Display for WildcardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<*{}>", self.name)
    }
}

/// End wildcard leaf node.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct EndWildcardState {
    pub name: String,
    pub data: NodeData,
}

impl fmt::Display for EndWildcardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<*{}>", self.name)
    }
}
