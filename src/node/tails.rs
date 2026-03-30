use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::node::Node;

/// Precomputed fixed suffixes for pruning during search.
#[derive(Clone, Debug, Default)]
pub(crate) struct Tails {
    inner: Box<[Box<[u8]>]>,
}

impl Tails {
    /// Returns `true` if there are no tail suffixes.
    #[must_use]
    pub(crate) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns `true` if the remaining path ends with any of the tail suffixes.
    #[must_use]
    pub(crate) fn matches(&self, remaining: &[u8]) -> bool {
        self.inner.is_empty()
            || self.inner.iter().any(|tail| {
                remaining.len() >= tail.len()
                    && tail
                        .iter()
                        .rev()
                        .zip(remaining.iter().rev())
                        .all(|(a, b)| a == b)
            })
    }

    /// Computes the tail suffixes for a node based on its children.
    pub(crate) fn compute<S, T>(node: &Node<S, T>) -> Self {
        Self {
            inner: compute_tails(node),
        }
    }
}

/// Computes all possible fixed suffixes the path must end with for any match through this node.
fn compute_tails<S, T>(node: &Node<S, T>) -> Box<[Box<[u8]>]> {
    if node.data.is_some() || node.end_wildcard.is_some() {
        return Box::default();
    }

    let mut tails: Vec<Vec<u8>> = Vec::new();

    for child in &node.static_children {
        if child.bounds.is_fixed() {
            if child.tails.is_empty() {
                tails.push(child.state.prefix.to_vec());
            } else {
                for child_tail in &*child.tails.inner {
                    let mut tail = child.state.prefix.to_vec();
                    tail.extend_from_slice(child_tail);
                    tails.push(tail);
                }
            }
        } else if child.tails.is_empty() {
            return Box::default();
        } else {
            tails.extend(child.tails.inner.iter().map(|t| t.to_vec()));
        }
    }

    for child_tails in node
        .dynamic_children
        .iter()
        .map(|child| &child.tails)
        .chain(node.wildcard_children.iter().map(|child| &child.tails))
    {
        if child_tails.is_empty() {
            return Box::default();
        }

        tails.extend(child_tails.inner.iter().map(|t| t.to_vec()));
    }

    if tails.is_empty() {
        return Box::default();
    }

    tails.sort();
    tails.dedup();

    tails
        .into_iter()
        .map(Vec::into_boxed_slice)
        .collect::<Vec<_>>()
        .into_boxed_slice()
}
