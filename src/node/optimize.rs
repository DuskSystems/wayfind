use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use core::cmp::Reverse;

use crate::node::Node;
use crate::priority::Priority;
use crate::state::StaticState;

impl<S> Node<S> {
    pub(crate) fn optimize(&mut self) {
        self.optimize_inner(Priority::default());
    }

    fn optimize_inner(&mut self, parent: Priority) {
        if !self.needs_optimization {
            return;
        }

        if let Some(data) = &mut self.data {
            data.priority = parent.clone();
        }

        for child in &mut self.static_children {
            let child_priority = parent.clone().with_static(child.state.prefix.len());
            child.optimize_inner(child_priority);
        }

        for child in &mut self.dynamic_children {
            let child_priority = parent.clone().with_dynamic();
            child.optimize_inner(child_priority);
            child.static_suffixes = Self::collect_static_suffixes(&child.static_children);
        }

        for child in &mut self.wildcard_children {
            let child_priority = parent.clone().with_wildcard();
            child.optimize_inner(child_priority);
            child.static_suffixes = Self::collect_static_suffixes(&child.static_children);
        }

        if let Some(child) = &mut self.end_wildcard {
            let child_priority = parent.with_wildcard();
            child.optimize_inner(child_priority);
        }

        self.static_children.sort();
        self.dynamic_children.sort();
        self.wildcard_children.sort();

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

    /// True if all static children start with '/' (no inline parameters).
    fn is_segment_only<T>(node: &Node<T>) -> bool {
        node.dynamic_children.is_empty()
            && node.wildcard_children.is_empty()
            && node.end_wildcard.is_none()
            && node
                .static_children
                .iter()
                .all(|child| child.state.prefix.first() == Some(&b'/'))
    }

    /// Collect static suffixes for inline matching, sorted longest first.
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
