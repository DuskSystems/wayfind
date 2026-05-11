use crate::node::Node;

/// Pre-computed path length bounds for pruning during search.
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
    pub(crate) fn compute<S, T>(node: &Node<S, T>) -> Self {
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
        // A node with data can match here with 0 remaining bytes.
        if node.data.is_some() {
            return 0;
        }

        // An end-wildcard needs at least 1 byte.
        if node.end_wildcard.is_some() {
            return 1;
        }

        let static_lengths = node.static_children.iter().map(|child| {
            child
                .state
                .prefix
                .len()
                .saturating_add(child.bounds.shortest)
        });

        let dynamic_lengths = node
            .dynamic_children
            .iter()
            .map(|child| child.bounds.shortest.saturating_add(1));

        let wildcard_lengths = node
            .wildcard_children
            .iter()
            .map(|child| child.bounds.shortest.saturating_add(1));

        static_lengths
            .chain(dynamic_lengths)
            .chain(wildcard_lengths)
            .min()
            .unwrap_or(usize::MAX)
    }

    fn compute_longest<S, T>(node: &Node<S, T>) -> usize {
        // Parameters can consume any input.
        if node.has_parameters() {
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
            .max()
            .unwrap_or(0)
    }
}
