use crate::node::Node;

/// Precomputed path length bounds for pruning during search.
#[derive(Clone, Debug)]
pub(crate) struct Bounds {
    /// Minimum remaining path bytes to reach any match.
    shortest: usize,
    /// Maximum remaining path bytes that could still match.
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
    pub(crate) fn new<S, T>(node: &Node<S, T>) -> Self {
        Self {
            shortest: Self::compute_shortest(node),
            longest: Self::compute_longest(node),
        }
    }

    pub(crate) const fn shortest(&self) -> usize {
        self.shortest
    }

    pub(crate) const fn longest(&self) -> usize {
        self.longest
    }

    fn compute_shortest<S, T>(node: &Node<S, T>) -> usize {
        // A node with data can match here with zero remaining bytes.
        if node.data.is_some() {
            return 0;
        }

        // An end-wildcard needs at least 1 byte (parameters can't be empty).
        if node.end_wildcard.is_some() {
            return 1;
        }

        let mut shortest = usize::MAX;

        for child in &node.static_children {
            let length = child
                .state
                .prefix
                .len()
                .saturating_add(child.bounds.shortest);

            shortest = shortest.min(length);
        }

        for child in &node.dynamic_children {
            let length = child.bounds.shortest.saturating_add(1);
            shortest = shortest.min(length);
        }

        for child in &node.wildcard_children {
            let length = child.bounds.shortest.saturating_add(1);
            shortest = shortest.min(length);
        }

        shortest
    }

    fn compute_longest<S, T>(node: &Node<S, T>) -> usize {
        // Dynamic and wildcard parameters can consume any input.
        if !node.dynamic_children.is_empty()
            || !node.wildcard_children.is_empty()
            || node.end_wildcard.is_some()
        {
            return usize::MAX;
        }

        let mut longest = 0;

        for child in &node.static_children {
            let length = child
                .state
                .prefix
                .len()
                .saturating_add(child.bounds.longest);

            longest = longest.max(length);
        }

        longest
    }
}
