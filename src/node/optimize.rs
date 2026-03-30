use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use core::cmp::Reverse;

use memchr::memmem::FinderRev;

use crate::node::Node;
use crate::node::bounds::Bounds;
use crate::node::tails::Tails;
use crate::state::StaticState;

impl<S, T> Node<S, T> {
    pub(crate) fn optimize(&mut self) {
        if !self.flags.needs_optimization() {
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

        self.flags.set_dynamic_segment_only(
            self.dynamic_children
                .iter()
                .all(|node| Self::is_segment_only(node)),
        );

        self.flags.set_wildcard_segment_only(
            self.wildcard_children
                .iter()
                .all(|node| Self::is_segment_only(node)),
        );

        self.bounds = Bounds::compute(self);
        self.tails = Tails::compute(self);

        self.flags.set_needs_optimization(false);
    }

    /// Returns `true` if all static children start with `/`.
    fn is_segment_only<U>(node: &Node<U, T>) -> bool {
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
        children: &[Node<StaticState, T>],
        current: &mut Vec<u8>,
        seen: &mut BTreeSet<Vec<u8>>,
    ) -> Vec<Vec<u8>> {
        seen.clear();
        Self::collect_suffixes_recursive(children, current, seen);

        let mut suffixes: Vec<Vec<u8>> = seen.iter().cloned().collect();
        suffixes.sort_by_key(|suffix| Reverse(suffix.len()));
        suffixes
    }

    fn collect_suffixes_recursive(
        children: &[Node<StaticState, T>],
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
