use alloc::{format, string::String, vec::Vec};
use core::cmp::Ordering;

pub trait NodeState: Ord {
    fn padding(&self) -> usize;
    fn key(&self) -> String;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RootState;

impl RootState {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl NodeState for RootState {
    fn padding(&self) -> usize {
        0
    }

    fn key(&self) -> String {
        String::new()
    }
}

impl Ord for RootState {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for RootState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticState {
    pub prefix: Vec<u8>,
}

impl StaticState {
    #[must_use]
    pub const fn new(prefix: Vec<u8>) -> Self {
        Self { prefix }
    }
}

impl NodeState for StaticState {
    fn padding(&self) -> usize {
        self.prefix.len().saturating_sub(1)
    }

    fn key(&self) -> String {
        String::from_utf8_lossy(&self.prefix).into_owned()
    }
}

impl Ord for StaticState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.prefix.cmp(&other.prefix)
    }
}

impl PartialOrd for StaticState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DynamicState {
    pub name: String,
}

impl DynamicState {
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self { name }
    }
}

impl NodeState for DynamicState {
    fn padding(&self) -> usize {
        self.name.len().saturating_sub(1)
    }

    fn key(&self) -> String {
        format!("<{}>", self.name)
    }
}

impl Ord for DynamicState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for DynamicState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WildcardState {
    pub name: String,
}

impl WildcardState {
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self { name }
    }
}

impl NodeState for WildcardState {
    fn padding(&self) -> usize {
        self.name.len().saturating_sub(1)
    }

    fn key(&self) -> String {
        format!("<*{}>", self.name)
    }
}

impl Ord for WildcardState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for WildcardState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EndWildcardState {
    pub name: String,
}

impl EndWildcardState {
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self { name }
    }
}

impl NodeState for EndWildcardState {
    fn padding(&self) -> usize {
        self.name.len().saturating_sub(1)
    }

    fn key(&self) -> String {
        format!("<*{}>", self.name)
    }
}

impl PartialOrd for EndWildcardState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EndWildcardState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}
