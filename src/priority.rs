use core::cmp::Ordering;

/// Measures how specific a template is in terms of matching priority.
///
/// The priority is as follows:
/// 1. Static prefix length: more static segments, more specific
/// 2. Dynamic parameter count: fewer dynamics, more specific
/// 3. Wildcard parameter count: fewer wildcards, more specific
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub struct Priority {
    pub static_length: usize,
    pub dynamics_count: usize,
    pub wildcards_count: usize,
}

impl Priority {
    pub const fn with_static(mut self, length: usize) -> Self {
        self.static_length += length;
        self
    }

    pub const fn with_dynamic(mut self) -> Self {
        self.dynamics_count += 1;
        self
    }

    pub const fn with_wildcard(mut self) -> Self {
        self.wildcards_count += 1;
        self
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.static_length
            .cmp(&other.static_length)
            .then(other.dynamics_count.cmp(&self.dynamics_count))
            .then(other.wildcards_count.cmp(&self.wildcards_count))
    }
}
