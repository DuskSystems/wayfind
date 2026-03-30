use alloc::boxed::Box;
use alloc::vec::Vec;

use hashbrown::HashMap;
use rustc_hash::FxBuildHasher;

use crate::node::Node;
use crate::state::StaticState;

/// Cached rightmost positions for `Contains` checks.
pub(crate) struct NeedleCache(HashMap<usize, Option<usize>, FxBuildHasher>);

impl NeedleCache {
    pub(crate) const fn new() -> Self {
        Self(HashMap::with_hasher(FxBuildHasher))
    }

    /// Returns the rightmost position of the needle, cached after first lookup.
    fn rightmost(&mut self, id: usize, needle: &[u8], path: &[u8]) -> Option<usize> {
        *self
            .0
            .entry(id)
            .or_insert_with(|| memchr::memmem::rfind(path, needle))
    }
}

/// A single reachability condition.
#[derive(Clone, Debug)]
enum Condition {
    /// The remaining path must end with these bytes.
    EndsWith(Box<[u8]>),
    /// The remaining path must contain these bytes.
    Contains { needle: Box<[u8]>, id: usize },
}

/// A group of conditions that must ALL pass for one possible match path.
#[derive(Clone, Debug)]
struct Group(Box<[Condition]>);

/// Pre-computed reachability conditions for a node.
#[derive(Clone, Debug, Default)]
pub(crate) struct Reachable(Box<[Group]>);

impl Reachable {
    /// Returns `true` if no reachability constraints exist.
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the remaining path could reach a match through this node.
    pub(crate) fn check(&self, path: &str, offset: usize, needles: &mut NeedleCache) -> bool {
        if self.0.is_empty() {
            return true;
        }

        let remaining = &path.as_bytes()[offset..];
        let bytes = path.as_bytes();

        self.0.iter().any(|group| {
            group.0.iter().all(|condition| match condition {
                Condition::EndsWith(suffix) => {
                    remaining.len() >= suffix.len()
                        && suffix
                            .iter()
                            .rev()
                            .zip(remaining.iter().rev())
                            .all(|(a, b)| a == b)
                }
                Condition::Contains { needle, id } => needles
                    .rightmost(*id, needle, bytes)
                    .is_some_and(|pos| pos >= offset),
            })
        })
    }

    /// Computes reachability conditions for a node's subtree.
    pub(crate) fn compute<S, T>(
        node: &Node<S, T>,
        needles: &mut HashMap<Box<[u8]>, usize, FxBuildHasher>,
    ) -> Self {
        // Nodes with data or end wildcards are always reachable (match here).
        if node.data.is_some() || node.end_wildcard.is_some() {
            return Self::default();
        }

        let mut groups = Vec::new();

        for child in &node.static_children {
            let mut prefix = child.state.prefix.to_vec();
            if !collect_groups(child, &mut prefix, &mut groups, needles) {
                return Self::default();
            }
        }

        for child in node
            .dynamic_children
            .iter()
            .map(|child| &child.reachable)
            .chain(node.wildcard_children.iter().map(|child| &child.reachable))
        {
            if child.is_empty() {
                return Self::default();
            }

            groups.extend(child.0.iter().cloned());
        }

        if groups.is_empty() {
            return Self::default();
        }

        Self(groups.into_boxed_slice())
    }
}

/// Returns the deduplicated needle ID for the given bytes.
fn needle_id(needles: &mut HashMap<Box<[u8]>, usize, FxBuildHasher>, bytes: &[u8]) -> usize {
    let len = needles.len();
    *needles.entry(bytes.into()).or_insert(len)
}

/// Walks a static subtree to collect condition groups.
/// Returns `false` if any branch is unconstrained.
fn collect_groups<T>(
    node: &Node<StaticState, T>,
    prefix: &mut Vec<u8>,
    groups: &mut Vec<Group>,
    needles: &mut HashMap<Box<[u8]>, usize, FxBuildHasher>,
) -> bool {
    let has_data = node.data.is_some();
    let has_params = !node.dynamic_children.is_empty()
        || !node.wildcard_children.is_empty()
        || node.end_wildcard.is_some();

    if has_params && prefix.len() <= 1 {
        return false;
    }

    if has_data && !has_params && node.static_children.is_empty() {
        groups.push(Group(Box::new([Condition::EndsWith(
            prefix.clone().into_boxed_slice(),
        )])));

        return true;
    }

    if has_params {
        let id = needle_id(needles, prefix);
        let contains = Condition::Contains {
            needle: prefix.clone().into_boxed_slice(),
            id,
        };

        let mut deeper_groups = Vec::new();
        let mut has_unconstrained = false;

        for child_reachable in node
            .dynamic_children
            .iter()
            .map(|child| &child.reachable)
            .chain(node.wildcard_children.iter().map(|child| &child.reachable))
        {
            if child_reachable.is_empty() {
                has_unconstrained = true;
            } else {
                for group in &*child_reachable.0 {
                    deeper_groups.push(group.0.to_vec());
                }
            }
        }

        if node.end_wildcard.is_some() {
            has_unconstrained = true;
        }

        if has_unconstrained || deeper_groups.is_empty() {
            groups.push(Group(Box::new([contains])));
        } else {
            for mut deeper in deeper_groups {
                if !deeper.iter().any(|condition| matches!(condition, Condition::Contains { needle: existing, .. } if **existing == *prefix))                 {
                    deeper.insert(0, contains.clone());
                }

                groups.push(Group(deeper.into_boxed_slice()));
            }
        }
    }

    if has_data && (has_params || !node.static_children.is_empty()) {
        let id = needle_id(needles, prefix);
        groups.push(Group(Box::new([Condition::Contains {
            needle: prefix.clone().into_boxed_slice(),
            id,
        }])));
    }

    for child in &node.static_children {
        let start = prefix.len();
        prefix.extend_from_slice(&child.state.prefix);

        if !collect_groups(child, prefix, groups, needles) {
            return false;
        }

        prefix.truncate(start);
    }

    true
}
