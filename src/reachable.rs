use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::needle::NeedleCache;
use crate::node::Node;
use crate::state::StaticState;

/// A single reachability condition.
#[derive(Clone, Debug)]
enum Condition {
    /// The remaining path must end with these bytes.
    EndsWith(Box<[u8]>),
    /// The remaining path must contain these bytes.
    Contains { needle: Box<[u8]>, id: usize },
}

impl Condition {
    fn contains(needles: &mut BTreeMap<Box<[u8]>, usize>, prefix: &[u8]) -> Self {
        let len = needles.len();
        let id = *needles.entry(prefix.into()).or_insert(len);
        Self::Contains {
            needle: prefix.into(),
            id,
        }
    }
}

/// A group of conditions that must all pass.
#[derive(Clone, Debug)]
struct Group {
    conditions: Box<[Condition]>,
}

impl Group {
    fn single(condition: Condition) -> Self {
        Self {
            conditions: Box::new([condition]),
        }
    }
}

/// Pre-computed reachability conditions for a node.
#[derive(Clone, Debug, Default)]
pub(crate) struct Reachable {
    groups: Box<[Group]>,
}

impl Reachable {
    /// Returns `true` if no reachability constraints exist.
    fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Returns `true` if the remaining path could reach a match through this node.
    pub(crate) fn check(&self, needles: &mut NeedleCache, path: &str, offset: usize) -> bool {
        if self.groups.is_empty() {
            return true;
        }

        let remaining = &path.as_bytes()[offset..];

        self.groups.iter().any(|group| {
            group.conditions.iter().all(|condition| match condition {
                Condition::EndsWith(suffix) => {
                    remaining.len() >= suffix.len()
                        && suffix
                            .iter()
                            .rev()
                            .zip(remaining.iter().rev())
                            .all(|(a, b)| a == b)
                }
                Condition::Contains { needle, id } => needles
                    .rightmost(*id, needle, path.as_bytes())
                    .is_some_and(|position| position >= offset),
            })
        })
    }

    /// Computes reachability conditions for a node's subtree.
    pub(crate) fn compute<S, T>(
        node: &Node<S, T>,
        needles: &mut BTreeMap<Box<[u8]>, usize>,
    ) -> Self {
        // Nodes with data or end wildcards are always reachable (match here).
        if node.data.is_some() || node.end_wildcard.is_some() {
            return Self::default();
        }

        let mut groups = Vec::new();

        for child in &node.static_children {
            let mut prefix = child.state.prefix.to_vec();
            if !Self::collect_static(child, &mut prefix, &mut groups, needles) {
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

            groups.extend(child.groups.iter().cloned());
        }

        if groups.is_empty() {
            return Self::default();
        }

        Self {
            groups: groups.into_boxed_slice(),
        }
    }

    /// Returns `false` if any branch is unconstrained.
    fn collect_static<T>(
        node: &Node<StaticState, T>,
        prefix: &mut Vec<u8>,
        groups: &mut Vec<Group>,
        needles: &mut BTreeMap<Box<[u8]>, usize>,
    ) -> bool {
        let has_data = node.data.is_some();
        let has_params = !node.dynamic_children.is_empty()
            || !node.wildcard_children.is_empty()
            || node.end_wildcard.is_some();

        if has_params && prefix.len() <= 1 {
            return false;
        }

        if has_data && !has_params && node.static_children.is_empty() {
            groups.push(Group::single(Condition::EndsWith(
                prefix.clone().into_boxed_slice(),
            )));

            return true;
        }

        if has_params {
            let contains = Condition::contains(needles, prefix);
            let mut deeper_groups = Vec::new();
            let mut has_unconstrained = node.end_wildcard.is_some();

            for child_reachable in node
                .dynamic_children
                .iter()
                .map(|child| &child.reachable)
                .chain(node.wildcard_children.iter().map(|child| &child.reachable))
            {
                if child_reachable.is_empty() {
                    has_unconstrained = true;
                } else {
                    for group in &*child_reachable.groups {
                        deeper_groups.push(group.conditions.to_vec());
                    }
                }
            }

            if has_unconstrained || deeper_groups.is_empty() {
                groups.push(Group::single(contains));
            } else {
                for mut deeper in deeper_groups {
                    let already_contains = deeper.iter().any(|condition| {
                        matches!(condition, Condition::Contains { needle, .. } if **needle == *prefix)
                    });

                    if !already_contains {
                        deeper.insert(0, contains.clone());
                    }

                    groups.push(Group {
                        conditions: deeper.into_boxed_slice(),
                    });
                }
            }
        }

        if has_data && (has_params || !node.static_children.is_empty()) {
            groups.push(Group::single(Condition::contains(needles, prefix)));
        }

        for child in &node.static_children {
            let start = prefix.len();
            prefix.extend_from_slice(&child.state.prefix);

            if !Self::collect_static(child, prefix, groups, needles) {
                return false;
            }

            prefix.truncate(start);
        }

        true
    }
}
