use alloc::{format, string::String, vec::Vec};
use core::cmp::Ordering;

pub trait State {
    fn priority(&self) -> usize;
    fn padding(&self) -> usize;
    fn key(&self) -> &str;
}

/// Root node state
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RootState {
    priority: usize,
    padding: usize,
    key: String,
}

impl RootState {
    pub const fn new() -> Self {
        Self {
            priority: 0,
            padding: 0,
            key: String::new(),
        }
    }
}

impl State for RootState {
    fn priority(&self) -> usize {
        self.priority
    }

    fn padding(&self) -> usize {
        self.padding
    }

    fn key(&self) -> &str {
        &self.key
    }
}

/// Static path segment state
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticState {
    pub prefix: Vec<u8>,
    priority: usize,
    padding: usize,
    key: String,
}

impl StaticState {
    pub fn new(prefix: Vec<u8>) -> Self {
        let priority = prefix.len();
        let padding = prefix.len().saturating_sub(1);
        let key = String::from_utf8_lossy(&prefix).into_owned();

        Self {
            prefix,
            priority,
            padding,
            key,
        }
    }
}

impl State for StaticState {
    fn priority(&self) -> usize {
        self.priority
    }

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

/// Dynamic parameter state
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DynamicState {
    pub name: String,
    pub constraint: Option<String>,
    priority: usize,
    padding: usize,
    key: String,
}

impl DynamicState {
    pub fn new(name: String, constraint: Option<String>) -> Self {
        let mut priority = name.len();
        if constraint.is_some() {
            priority += 10_000;
        }

        let padding = name.len().saturating_sub(1);
        let key = constraint.as_ref().map_or_else(
            || format!("{{{name}}}"),
            |constraint| format!("{{{name}:{constraint}}}"),
        );

        Self {
            name,
            constraint,
            priority,
            padding,
            key,
        }
    }
}

impl State for DynamicState {
    fn priority(&self) -> usize {
        self.priority
    }

    fn padding(&self) -> usize {
        self.padding
    }

    fn key(&self) -> &str {
        &self.key
    }
}

impl Ord for DynamicState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.constraint.cmp(&other.constraint))
    }
}

impl PartialOrd for DynamicState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Wildcard state that can match across path segments
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WildcardState {
    pub name: String,
    pub constraint: Option<String>,
    priority: usize,
    padding: usize,
    key: String,
}

impl WildcardState {
    pub fn new(name: String, constraint: Option<String>) -> Self {
        let mut priority = name.len();
        if constraint.is_some() {
            priority += 10_000;
        }

        let padding = name.len().saturating_sub(1);
        let key = constraint.as_ref().map_or_else(
            || format!("{{*{name}}}"),
            |constraint| format!("{{*{name}:{constraint}}}"),
        );

        Self {
            name,
            constraint,
            priority,
            padding,
            key,
        }
    }
}

impl State for WildcardState {
    fn priority(&self) -> usize {
        self.priority
    }

    fn padding(&self) -> usize {
        self.padding
    }

    fn key(&self) -> &str {
        &self.key
    }
}

impl Ord for WildcardState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.constraint.cmp(&other.constraint))
    }
}

impl PartialOrd for WildcardState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// End wildcard state that matches remaining path
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EndWildcardState {
    pub name: String,
    pub constraint: Option<String>,
    priority: usize,
    padding: usize,
    key: String,
}

impl EndWildcardState {
    pub fn new(name: String, constraint: Option<String>) -> Self {
        let mut priority = name.len();
        if constraint.is_some() {
            priority += 10_000;
        }

        let padding = name.len().saturating_sub(1);
        let key = constraint.as_ref().map_or_else(
            || format!("{{*{name}}}"),
            |constraint| format!("{{*{name}:{constraint}}}"),
        );

        Self {
            name,
            constraint,
            priority,
            padding,
            key,
        }
    }
}

impl State for EndWildcardState {
    fn priority(&self) -> usize {
        self.priority
    }

    fn padding(&self) -> usize {
        self.padding
    }

    fn key(&self) -> &str {
        &self.key
    }
}

impl Ord for EndWildcardState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.constraint.cmp(&other.constraint))
    }
}

impl PartialOrd for EndWildcardState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
