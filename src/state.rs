use alloc::boxed::Box;
use alloc::string::String;
use core::fmt;

/// Root node of the tree.
#[derive(Clone, Debug)]
pub(crate) struct RootState;

impl RootState {
    #[must_use]
    pub(crate) const fn new() -> Self {
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
#[derive(Clone, Debug)]
pub(crate) struct StaticState {
    pub first: u8,
    pub prefix: Box<[u8]>,
}

impl StaticState {
    #[must_use]
    pub(crate) fn new(prefix: &[u8]) -> Self {
        Self {
            first: prefix.first().copied().unwrap_or(0),
            prefix: prefix.into(),
        }
    }
}

impl fmt::Display for StaticState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.prefix))
    }
}

/// Dynamic parameter with its name.
#[derive(Clone, Debug)]
pub(crate) struct DynamicState {
    pub name: Box<str>,
}

impl DynamicState {
    #[must_use]
    pub(crate) fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

impl fmt::Display for DynamicState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.name)
    }
}

/// Wildcard parameter with its name.
#[derive(Clone, Debug)]
pub(crate) struct WildcardState {
    pub name: Box<str>,
}

impl WildcardState {
    #[must_use]
    pub(crate) fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

impl fmt::Display for WildcardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<*{}>", self.name)
    }
}

/// End wildcard parameter with its name.
#[derive(Clone, Debug)]
pub(crate) struct EndWildcardState {
    pub name: Box<str>,
}

impl EndWildcardState {
    #[must_use]
    pub(crate) fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

impl fmt::Display for EndWildcardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<*{}>", self.name)
    }
}
