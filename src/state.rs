use alloc::{format, string::String, vec::Vec};
use core::cmp::Ordering;

pub trait NodeState: Ord {
    fn padding(&self) -> usize;
    fn key(&self) -> &str;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RootState {
    padding: usize,
    key: String,
}

impl RootState {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            padding: 0,
            key: String::new(),
        }
    }
}

impl NodeState for RootState {
    fn padding(&self) -> usize {
        self.padding
    }

    fn key(&self) -> &str {
        &self.key
    }
}

impl Ord for RootState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
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
    padding: usize,
    key: String,
}

impl StaticState {
    #[must_use]
    pub fn new(prefix: Vec<u8>) -> Self {
        let padding = prefix.len().saturating_sub(1);
        let key = String::from_utf8_lossy(&prefix).into_owned();

        Self {
            prefix,
            padding,
            key,
        }
    }
}

impl NodeState for StaticState {
    fn padding(&self) -> usize {
        self.padding
    }

    fn key(&self) -> &str {
        &self.key
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
    padding: usize,
    key: String,
}

impl DynamicState {
    #[must_use]
    pub fn new(name: String) -> Self {
        let padding = name.len().saturating_sub(1);
        let key = format!("{{{name}}}");

        Self { name, padding, key }
    }
}

impl NodeState for DynamicState {
    fn padding(&self) -> usize {
        self.padding
    }

    fn key(&self) -> &str {
        &self.key
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
pub struct DynamicConstrainedState {
    pub name: String,
    pub constraint: usize,
    padding: usize,
    key: String,
}

impl DynamicConstrainedState {
    #[must_use]
    pub fn new(name: String, constraint: usize) -> Self {
        let padding = name.len().saturating_sub(1);
        let key = format!("{{{name}:{constraint}}}");

        Self {
            name,
            constraint,
            padding,
            key,
        }
    }
}

impl NodeState for DynamicConstrainedState {
    fn padding(&self) -> usize {
        self.padding
    }

    fn key(&self) -> &str {
        &self.key
    }
}

impl Ord for DynamicConstrainedState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.constraint.cmp(&other.constraint))
    }
}

impl PartialOrd for DynamicConstrainedState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WildcardState {
    pub name: String,
    padding: usize,
    key: String,
}

impl WildcardState {
    #[must_use]
    pub fn new(name: String) -> Self {
        let padding = name.len().saturating_sub(1);
        let key = format!("{{*{name}}}");

        Self { name, padding, key }
    }
}

impl NodeState for WildcardState {
    fn padding(&self) -> usize {
        self.padding
    }

    fn key(&self) -> &str {
        &self.key
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
pub struct WildcardConstrainedState {
    pub name: String,
    pub constraint: usize,
    padding: usize,
    key: String,
}

impl WildcardConstrainedState {
    #[must_use]
    pub fn new(name: String, constraint: usize) -> Self {
        let padding = name.len().saturating_sub(1);
        let key = format!("{{*{name}:{constraint}}}");

        Self {
            name,
            constraint,
            padding,
            key,
        }
    }
}

impl NodeState for WildcardConstrainedState {
    fn padding(&self) -> usize {
        self.padding
    }

    fn key(&self) -> &str {
        &self.key
    }
}

impl Ord for WildcardConstrainedState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.constraint.cmp(&other.constraint))
    }
}

impl PartialOrd for WildcardConstrainedState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EndWildcardState {
    pub name: String,
    padding: usize,
    key: String,
}

impl EndWildcardState {
    #[must_use]
    pub fn new(name: String) -> Self {
        let padding = name.len().saturating_sub(1);
        let key = format!("{{*{name}}}");

        Self { name, padding, key }
    }
}

impl NodeState for EndWildcardState {
    fn padding(&self) -> usize {
        self.padding
    }

    fn key(&self) -> &str {
        &self.key
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EndWildcardConstrainedState {
    pub name: String,
    pub constraint: usize,
    padding: usize,
    key: String,
}

impl EndWildcardConstrainedState {
    #[must_use]
    pub fn new(name: String, constraint: usize) -> Self {
        let padding = name.len().saturating_sub(1);
        let key = format!("{{*{name}:{constraint}}}");

        Self {
            name,
            constraint,
            padding,
            key,
        }
    }
}

impl NodeState for EndWildcardConstrainedState {
    fn padding(&self) -> usize {
        self.padding
    }

    fn key(&self) -> &str {
        &self.key
    }
}

impl Ord for EndWildcardConstrainedState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.constraint.cmp(&other.constraint))
    }
}

impl PartialOrd for EndWildcardConstrainedState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
