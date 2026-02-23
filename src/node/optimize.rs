use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use core::cmp::Reverse;

use crate::node::Node;
use crate::state::StaticState;

impl<S> Node<S> {
    pub fn optimize(&mut self) {
        if !self.needs_optimization {
            return;
        }

        for child in &mut self.static_children {
            child.optimize();
        }

        for child in &mut self.dynamic_children {
            child.optimize();
            child.state.suffixes = Self::collect_suffixes(&child.static_children);
        }

        for child in &mut self.wildcard_children {
            child.optimize();
            child.state.suffixes = Self::collect_suffixes(&child.static_children);
        }

        self.static_children
            .sort_by(|a, b| a.state.prefix.cmp(&b.state.prefix));

        self.dynamic_children.sort_by(|a, b| {
            let a_len = a.state.suffixes.first().map_or(0, |s| s.len());
            let b_len = b.state.suffixes.first().map_or(0, |s| s.len());
            b_len
                .cmp(&a_len)
                .then_with(|| a.state.name.cmp(&b.state.name))
        });

        self.wildcard_children.sort_by(|a, b| {
            let a_len = a.state.suffixes.first().map_or(0, |s| s.len());
            let b_len = b.state.suffixes.first().map_or(0, |s| s.len());
            b_len
                .cmp(&a_len)
                .then_with(|| a.state.name.cmp(&b.state.name))
        });

        self.dynamic_segment_only = self
            .dynamic_children
            .iter()
            .all(|node| Self::is_segment_only(node));

        self.wildcard_segment_only = self
            .wildcard_children
            .iter()
            .all(|node| Self::is_segment_only(node));

        self.shortest = self.compute_shortest();
        self.tail = self.compute_tail().into_boxed_slice();

        self.needs_optimization = false;
    }

    /// Returns `true` if all static children start with `/` (no inline parameters).
    fn is_segment_only<T>(node: &Node<T>) -> bool {
        node.dynamic_children.is_empty()
            && node.wildcard_children.is_empty()
            && node.end_wildcard.is_none()
            && node
                .static_children
                .iter()
                .all(|child| child.state.prefix.first() == Some(&b'/'))
    }

    /// Collects static suffixes from children for parameter matching.
    fn collect_suffixes(children: &[Node<StaticState>]) -> Vec<Box<[u8]>> {
        let mut seen = BTreeSet::new();
        Self::collect_suffixes_recursive(children, &mut Vec::new(), &mut seen);

        let mut suffixes: Vec<Box<[u8]>> = seen.into_iter().map(Vec::into_boxed_slice).collect();
        suffixes.sort_by_key(|suffix| Reverse(suffix.len()));
        suffixes
    }

    /// Minimum bytes of remaining path needed for any match through this node.
    fn compute_shortest(&self) -> usize {
        let base = if self.data.is_some() || self.end_wildcard.is_some() {
            Some(0)
        } else {
            None
        };

        base.into_iter()
            .chain(
                self.static_children
                    .iter()
                    .map(|child| child.state.prefix.len().saturating_add(child.shortest)),
            )
            .chain(
                self.dynamic_children
                    .iter()
                    .map(|child| 1_usize.saturating_add(child.shortest)),
            )
            .chain(
                self.wildcard_children
                    .iter()
                    .map(|child| 1_usize.saturating_add(child.shortest)),
            )
            .min()
            .unwrap_or(usize::MAX)
    }

    /// Maximum bytes of remaining path for any match through this node.
    fn compute_longest(&self) -> usize {
        if !self.dynamic_children.is_empty() || !self.wildcard_children.is_empty() {
            return usize::MAX;
        }

        let base = if self.end_wildcard.is_some() {
            usize::MAX
        } else {
            0
        };

        self.static_children
            .iter()
            .map(|child| {
                child
                    .state
                    .prefix
                    .len()
                    .saturating_add(child.compute_longest())
            })
            .fold(base, usize::max)
    }

    /// Computes the longest fixed suffix the path must end with for any match through this node.
    fn compute_tail(&self) -> Vec<u8> {
        let mut candidates: Vec<Vec<u8>> = Vec::new();

        if self.data.is_some() || self.end_wildcard.is_some() {
            candidates.push(Vec::new());
        }

        for child in &self.static_children {
            let is_pure =
                child.shortest == child.compute_longest() && child.tail.len() == child.shortest;

            if is_pure {
                let mut tail = child.state.prefix.clone();
                tail.extend_from_slice(&child.tail);
                candidates.push(tail);
            } else {
                candidates.push(child.tail.to_vec());
            }
        }

        for child in &self.dynamic_children {
            candidates.push(child.tail.to_vec());
        }

        for child in &self.wildcard_children {
            candidates.push(child.tail.to_vec());
        }

        match candidates.len() {
            0 => Vec::new(),
            1 => candidates.into_iter().next().unwrap_or_default(),
            _ => Self::longest_common_suffix(&candidates),
        }
    }

    /// Returns the longest byte sequence that is a suffix of every candidate.
    fn longest_common_suffix(candidates: &[Vec<u8>]) -> Vec<u8> {
        let first = &candidates[0];
        let mut length = first.len();

        for other in &candidates[1..] {
            length = length.min(other.len());

            while length > 0 {
                if first[first.len() - length..] == other[other.len() - length..] {
                    break;
                }

                length -= 1;
            }

            if length == 0 {
                return Vec::new();
            }
        }

        first[first.len() - length..].to_vec()
    }

    fn collect_suffixes_recursive(
        children: &[Node<StaticState>],
        current: &mut Vec<u8>,
        seen: &mut BTreeSet<Vec<u8>>,
    ) {
        for child in children {
            current.extend_from_slice(&child.state.prefix);

            let is_suffix = child.data.is_some()
                || !child.dynamic_children.is_empty()
                || !child.wildcard_children.is_empty()
                || child.end_wildcard.is_some();

            if is_suffix {
                seen.insert(current.clone());
            }

            Self::collect_suffixes_recursive(&child.static_children, current, seen);

            current.truncate(current.len() - child.state.prefix.len());
        }
    }
}
