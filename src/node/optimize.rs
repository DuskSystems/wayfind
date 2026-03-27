use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use core::cmp::Reverse;

use hashbrown::HashMap;
use memchr::memmem::FinderRev;
use rustc_hash::FxBuildHasher;

use crate::node::{Node, Reachable, Suffix};
use crate::state::StaticState;

impl<S> Node<S> {
    pub(crate) fn optimize(
        &mut self,
        needles: &mut HashMap<Vec<u8>, usize, FxBuildHasher>,
        counter: &mut usize,
    ) {
        self.id = *counter;
        *counter += 1;

        if !self.flags.is_needs_optimization() {
            self.assign_child_ids(counter);
            return;
        }

        for child in &mut self.static_children {
            child.optimize(needles, counter);
        }

        let mut seen = BTreeSet::new();
        let mut current = Vec::new();

        for child in &mut self.dynamic_children {
            child.optimize(needles, counter);
            update_suffixes(child, &mut current, &mut seen);
        }

        for child in &mut self.wildcard_children {
            child.optimize(needles, counter);
            update_suffixes(child, &mut current, &mut seen);
        }

        self.static_children
            .sort_by(|a, b| a.state.prefix.cmp(&b.state.prefix));

        self.dynamic_children.sort_by(|a, b| {
            let a_len = a.suffixes.first().map_or(0, |suffix| suffix.needle.len());
            let b_len = b.suffixes.first().map_or(0, |suffix| suffix.needle.len());
            b_len
                .cmp(&a_len)
                .then_with(|| a.state.name.cmp(&b.state.name))
        });

        self.wildcard_children.sort_by(|a, b| {
            let a_len = a.suffixes.first().map_or(0, |suffix| suffix.needle.len());
            let b_len = b.suffixes.first().map_or(0, |suffix| suffix.needle.len());
            b_len
                .cmp(&a_len)
                .then_with(|| a.state.name.cmp(&b.state.name))
        });

        self.flags
            .set_dynamic_segment_only(self.dynamic_children.iter().all(is_segment_only));

        self.flags
            .set_wildcard_segment_only(self.wildcard_children.iter().all(is_segment_only));

        self.shortest = self.compute_shortest();
        self.longest = self.compute_longest();
        self.tails = self.compute_tails();

        if self.tails.is_empty() {
            self.reachable = self.compute_reachable(needles);
            self.flags.set_needs_cache(
                self.reachable
                    .iter()
                    .any(|entry| matches!(entry, Reachable::Contains { .. } | Reachable::Flexible)),
            );
        } else {
            self.reachable = Box::default();
            self.flags.set_needs_cache(false);
        }

        self.flags.set_needs_optimization(false);
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

    fn compute_reachable(
        &self,
        needles: &mut HashMap<Vec<u8>, usize, FxBuildHasher>,
    ) -> Box<[Reachable]> {
        if self.data.is_some() || self.end_wildcard.is_some() {
            return Box::new([Reachable::Flexible, Reachable::End]);
        }

        let mut reachable = Vec::new();
        let mut prefix = Vec::new();
        collect_reachable(&self.static_children, &mut prefix, &mut reachable, needles);

        for child in &self.dynamic_children {
            extend_child_reachable(child.reachable.iter(), &child.tails, &mut reachable);
        }

        for child in &self.wildcard_children {
            extend_child_reachable(child.reachable.iter(), &child.tails, &mut reachable);
        }

        reachable.into_boxed_slice()
    }

    fn assign_child_ids(&mut self, counter: &mut usize) {
        for child in &mut self.static_children {
            child.id = *counter;
            *counter += 1;
            child.assign_child_ids(counter);
        }

        for child in &mut self.dynamic_children {
            child.id = *counter;
            *counter += 1;
            child.assign_child_ids(counter);
        }

        for child in &mut self.wildcard_children {
            child.id = *counter;
            *counter += 1;
            child.assign_child_ids(counter);
        }

        if let Some(child) = &mut self.end_wildcard {
            child.id = *counter;
            *counter += 1;
        }
    }
}

/// Updates the suffixes on a dynamic or wildcard child node.
fn update_suffixes<T>(child: &mut Node<T>, current: &mut Vec<u8>, seen: &mut BTreeSet<Vec<u8>>) {
    let new = collect_suffixes(&child.static_children, current, seen);
    let unchanged = child.suffixes.len() == new.len()
        && child
            .suffixes
            .iter()
            .zip(&new)
            .all(|(entry, bytes)| *entry.needle == *bytes.as_slice());

    if !unchanged {
        child.suffixes = new
            .into_iter()
            .map(|bytes| {
                let finder = Box::new(FinderRev::new(&bytes).into_owned());
                Suffix {
                    needle: bytes.into_boxed_slice(),
                    finder,
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
    }
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
    collect_suffixes_recursive(children, current, seen);

    let mut suffixes: Vec<Vec<u8>> = seen.iter().cloned().collect();
    suffixes.sort_by_key(|suffix| Reverse(suffix.len()));
    suffixes
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

        collect_suffixes_recursive(&child.static_children, current, seen);
        current.truncate(current.len() - child.state.prefix.len());
    }
}

fn extend_child_reachable<'a>(
    entries: impl Iterator<Item = &'a Reachable>,
    tails: &[Box<[u8]>],
    reachable: &mut Vec<Reachable>,
) {
    let before = reachable.len();
    reachable.extend(entries.cloned());

    if reachable.len() == before && !tails.is_empty() {
        for tail in tails {
            reachable.push(Reachable::Suffix(tail.clone()));
            reachable.push(Reachable::End);
        }
    }
}

fn collect_reachable(
    children: &[Node<StaticState>],
    prefix: &mut Vec<u8>,
    reachable: &mut Vec<Reachable>,
    needles: &mut HashMap<Vec<u8>, usize, FxBuildHasher>,
) {
    for child in children {
        let start = prefix.len();
        prefix.extend_from_slice(&child.state.prefix);

        if child.shortest == child.longest {
            collect_fixed_length(child, prefix, reachable);
        } else {
            collect_variable_length(child, prefix, reachable, needles);
        }

        prefix.truncate(start);
    }
}

fn collect_fixed_length(child: &Node<StaticState>, prefix: &[u8], reachable: &mut Vec<Reachable>) {
    if child.tails.is_empty() {
        let has_suffix = child
            .reachable
            .iter()
            .any(|entry| matches!(entry, Reachable::Suffix(_)));

        if has_suffix {
            for entry in &*child.reachable {
                if let Reachable::Suffix(bytes) = entry {
                    let mut combined = prefix.to_vec();
                    combined.extend_from_slice(bytes);
                    reachable.push(Reachable::Suffix(combined.into_boxed_slice()));
                    reachable.push(Reachable::End);
                }
            }
        } else {
            reachable.push(Reachable::Suffix(prefix.to_vec().into_boxed_slice()));
            reachable.push(Reachable::End);
        }
    } else {
        for tail in &*child.tails {
            let mut combined = prefix.to_vec();
            combined.extend_from_slice(tail);
            reachable.push(Reachable::Suffix(combined.into_boxed_slice()));
            reachable.push(Reachable::End);
        }
    }
}

fn collect_variable_length(
    child: &Node<StaticState>,
    prefix: &[u8],
    reachable: &mut Vec<Reachable>,
    needles: &mut HashMap<Vec<u8>, usize, FxBuildHasher>,
) {
    let contains = Reachable::Contains {
        needle_id: {
            let needle = prefix.to_vec();
            let next = needles.len();
            *needles.entry(needle).or_insert(next)
        },
        needle: prefix.to_vec().into_boxed_slice(),
    };

    if child.tails.is_empty() {
        for child_group in child
            .reachable
            .split(|entry| matches!(entry, Reachable::End))
        {
            if child_group.is_empty() {
                continue;
            }

            reachable.push(contains.clone());

            for condition in child_group {
                if let Reachable::Suffix(bytes) = condition {
                    reachable.push(Reachable::Suffix(bytes.clone()));
                }
            }

            reachable.push(Reachable::End);
        }
    } else {
        for tail in &*child.tails {
            reachable.push(contains.clone());
            reachable.push(Reachable::Suffix(tail.clone()));
            reachable.push(Reachable::End);
        }

        let has_flexible = !child.dynamic_children.is_empty()
            || !child.wildcard_children.is_empty()
            || child.end_wildcard.is_some()
            || child.data.is_some();

        if has_flexible {
            reachable.push(contains);
            reachable.push(Reachable::End);
        }
    }
}
