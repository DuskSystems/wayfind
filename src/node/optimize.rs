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
            child.static_suffixes = Self::collect_static_suffixes(&child.static_children);
        }

        for child in &mut self.wildcard_children {
            child.optimize();
            child.static_suffixes = Self::collect_static_suffixes(&child.static_children);
        }

        if let Some(child) = &mut self.end_wildcard {
            child.optimize();
        }

        self.static_children
            .sort_by(|a, b| a.state.prefix.cmp(&b.state.prefix));

        self.dynamic_children.sort_by(|a, b| {
            let a_len = a.static_suffixes.first().map_or(0, Vec::len);
            let b_len = b.static_suffixes.first().map_or(0, Vec::len);
            b_len
                .cmp(&a_len)
                .then_with(|| a.state.name.cmp(&b.state.name))
        });

        self.wildcard_children.sort_by(|a, b| {
            let a_len = a.static_suffixes.first().map_or(0, Vec::len);
            let b_len = b.static_suffixes.first().map_or(0, Vec::len);
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

    /// Collects static suffixes for inline matching, sorted longest first.
    fn collect_static_suffixes(children: &[Node<StaticState>]) -> Vec<Vec<u8>> {
        let mut seen = BTreeSet::new();
        Self::collect_static_suffixes_recursive(children, &mut Vec::new(), &mut seen);

        let mut suffixes: Vec<Vec<u8>> = seen.into_iter().collect();
        suffixes.sort_by_key(|suffix| Reverse(suffix.len()));
        suffixes
    }

    fn collect_static_suffixes_recursive(
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

            Self::collect_static_suffixes_recursive(&child.static_children, current, seen);
            current.truncate(current.len() - child.state.prefix.len());
        }
    }
}
