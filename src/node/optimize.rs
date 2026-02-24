use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use core::cmp::Reverse;

use memchr::memmem::FinderRev;

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

        let mut seen = BTreeSet::new();
        let mut current = Vec::new();

        for child in &mut self.dynamic_children {
            child.optimize();
            child.state.suffixes =
                Self::collect_suffixes(&child.static_children, &mut current, &mut seen)
                    .into_iter()
                    .map(|suffix| FinderRev::new(&suffix).into_owned())
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
        }

        for child in &mut self.wildcard_children {
            child.optimize();
            child.state.suffixes =
                Self::collect_suffixes(&child.static_children, &mut current, &mut seen)
                    .into_iter()
                    .map(|suffix| FinderRev::new(&suffix).into_owned())
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
        }

        self.static_children
            .sort_by(|a, b| a.state.prefix.cmp(&b.state.prefix));

        self.dynamic_children.sort_by(|a, b| {
            let a_len = a
                .state
                .suffixes
                .first()
                .map_or(0, |suffix| suffix.needle().len());

            let b_len = b
                .state
                .suffixes
                .first()
                .map_or(0, |suffix| suffix.needle().len());

            b_len
                .cmp(&a_len)
                .then_with(|| a.state.name.cmp(&b.state.name))
        });

        self.wildcard_children.sort_by(|a, b| {
            let a_len = a
                .state
                .suffixes
                .first()
                .map_or(0, |suffix| suffix.needle().len());

            let b_len = b
                .state
                .suffixes
                .first()
                .map_or(0, |suffix| suffix.needle().len());

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
        self.longest = self.compute_longest();
        self.tails = self.compute_tails();

        self.needs_optimization = false;
    }

    /// Returns `true` if all static children start with `/`.
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
    fn collect_suffixes(
        children: &[Node<StaticState>],
        current: &mut Vec<u8>,
        seen: &mut BTreeSet<Vec<u8>>,
    ) -> Vec<Vec<u8>> {
        seen.clear();
        Self::collect_suffixes_recursive(children, current, seen);

        let mut suffixes: Vec<Vec<u8>> = seen.iter().cloned().collect();
        suffixes.sort_by_key(|suffix| Reverse(suffix.len()));
        suffixes
    }

    /// Minimum bytes of remaining path needed for any match through this node.
    fn compute_shortest(&self) -> usize {
        if self.data.is_some() || self.end_wildcard.is_some() {
            return 0;
        }

        self.static_children
            .iter()
            .map(|child| child.state.prefix.len().saturating_add(child.shortest))
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
        if !self.dynamic_children.is_empty()
            || !self.wildcard_children.is_empty()
            || self.end_wildcard.is_some()
        {
            return usize::MAX;
        }

        self.static_children
            .iter()
            .map(|child| child.state.prefix.len().saturating_add(child.longest))
            .fold(0, usize::max)
    }

    /// Computes all possible fixed suffixes the path must end with for any match through this node.
    fn compute_tails(&self) -> Box<[Box<[u8]>]> {
        if self.data.is_some() || self.end_wildcard.is_some() {
            return Box::default();
        }

        let mut tails: Vec<Vec<u8>> = Vec::new();

        for child in &self.static_children {
            if child.shortest == child.longest {
                if child.tails.is_empty() {
                    tails.push(child.state.prefix.to_vec());
                } else {
                    for child_tail in &*child.tails {
                        let mut tail = child.state.prefix.to_vec();
                        tail.extend_from_slice(child_tail);
                        tails.push(tail);
                    }
                }
            } else if child.tails.is_empty() {
                return Box::default();
            } else {
                tails.extend(child.tails.iter().map(|t| t.to_vec()));
            }
        }

        for child_tails in self
            .dynamic_children
            .iter()
            .map(|child| &child.tails)
            .chain(self.wildcard_children.iter().map(|child| &child.tails))
        {
            if child_tails.is_empty() {
                return Box::default();
            }

            tails.extend(child_tails.iter().map(|t| t.to_vec()));
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
