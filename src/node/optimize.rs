use alloc::boxed::Box;
use alloc::vec::Vec;

use hashbrown::{HashMap, HashSet};
use rustc_hash::FxBuildHasher;

use crate::node::Node;
use crate::node::bounds::Bounds;
use crate::node::reachable::Reachable;
use crate::node::suffixes::Suffixes;

impl<S, T> Node<S, T> {
    /// Optimizes the tree.
    pub(crate) fn optimize(&mut self) {
        let mut needles = HashMap::with_hasher(FxBuildHasher);
        self.optimize_inner(&mut needles);
    }

    fn optimize_inner(&mut self, needles: &mut HashMap<Box<[u8]>, usize, FxBuildHasher>) {
        if !self.flags.needs_optimization() {
            return;
        }

        for child in &mut self.static_children {
            child.optimize_inner(needles);
        }

        let mut seen = HashSet::with_hasher(FxBuildHasher);
        let mut current = Vec::new();

        for child in &mut self.dynamic_children {
            child.optimize_inner(needles);
            Suffixes::update(child, &mut current, &mut seen);
        }

        for child in &mut self.wildcard_children {
            child.optimize_inner(needles);
            Suffixes::update(child, &mut current, &mut seen);
        }

        self.static_children
            .sort_by(|a, b| a.state.prefix.cmp(&b.state.prefix));

        self.dynamic_children.sort_by(|a, b| {
            b.suffixes
                .longest()
                .cmp(&a.suffixes.longest())
                .then_with(|| a.state.name.cmp(&b.state.name))
        });

        self.wildcard_children.sort_by(|a, b| {
            b.suffixes
                .longest()
                .cmp(&a.suffixes.longest())
                .then_with(|| a.state.name.cmp(&b.state.name))
        });

        self.flags
            .set_dynamic_segment_only(self.dynamic_children.iter().all(is_segment_only));

        self.flags
            .set_wildcard_segment_only(self.wildcard_children.iter().all(is_segment_only));

        self.bounds = Bounds::compute(self);
        self.reachable = Reachable::compute(self, needles);

        self.flags.set_needs_optimization(false);
    }
}

/// Returns `true` if all static children start with `/`.
fn is_segment_only<S, T>(node: &Node<S, T>) -> bool {
    node.dynamic_children.is_empty()
        && node.wildcard_children.is_empty()
        && node.end_wildcard.is_none()
        && node
            .static_children
            .iter()
            .all(|child| child.state.prefix.first() == Some(&b'/'))
}
