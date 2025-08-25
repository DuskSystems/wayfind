use alloc::{string::String, vec::Vec};
use core::{cmp::Ordering, fmt};

#[derive(Clone, Debug, Eq, PartialEq)]
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

impl fmt::Display for StaticState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.prefix))
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

impl fmt::Display for DynamicState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.name)
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

impl fmt::Display for WildcardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<*{}>", self.name)
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

impl fmt::Display for EndWildcardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<*{}>", self.name)
    }
}

impl Ord for EndWildcardState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for EndWildcardState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
