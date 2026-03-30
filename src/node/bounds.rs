use crate::node::Node;

/// Precomputed length bounds for pruning during search.
#[derive(Clone, Debug)]
pub(crate) struct Bounds {
    shortest: usize,
    longest: usize,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            shortest: usize::MAX,
            longest: 0,
        }
    }
}

impl Bounds {
    /// Returns the minimum bytes of remaining path needed for any match through this node.
    #[must_use]
    pub(crate) const fn shortest(&self) -> usize {
        self.shortest
    }

    /// Returns `true` if the shortest and longest bounds are the same.
    #[must_use]
    pub(crate) const fn is_fixed(&self) -> bool {
        self.shortest == self.longest
    }

    /// Returns `true` if the remaining path length falls within the bounds.
    #[must_use]
    pub(crate) const fn matches(&self, remaining: usize) -> bool {
        remaining >= self.shortest && remaining <= self.longest
    }

    /// Computes the bounds for a node based on its children.
    pub(crate) fn compute<S, T>(node: &Node<S, T>) -> Self {
        Self {
            shortest: compute_shortest(node),
            longest: compute_longest(node),
        }
    }
}

/// Minimum bytes of remaining path needed for any match through this node.
fn compute_shortest<S, T>(node: &Node<S, T>) -> usize {
    if node.data.is_some() || node.end_wildcard.is_some() {
        return 0;
    }

    node.static_children
        .iter()
        .map(|child| {
            child
                .state
                .prefix
                .len()
                .saturating_add(child.bounds.shortest())
        })
        .chain(
            node.dynamic_children
                .iter()
                .map(|child| 1_usize.saturating_add(child.bounds.shortest())),
        )
        .chain(
            node.wildcard_children
                .iter()
                .map(|child| 1_usize.saturating_add(child.bounds.shortest())),
        )
        .min()
        .unwrap_or(usize::MAX)
}

/// Maximum bytes of remaining path for any match through this node.
fn compute_longest<S, T>(node: &Node<S, T>) -> usize {
    if !node.dynamic_children.is_empty()
        || !node.wildcard_children.is_empty()
        || node.end_wildcard.is_some()
    {
        return usize::MAX;
    }

    node.static_children
        .iter()
        .map(|child| {
            child
                .state
                .prefix
                .len()
                .saturating_add(child.bounds.longest)
        })
        .fold(0, usize::max)
}
