use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::Reverse;

use hashbrown::HashSet;
use memchr::memmem::FinderRev;
use rustc_hash::FxBuildHasher;

use crate::node::Node;
use crate::state::StaticState;

/// A single pre-computed suffix needle.
#[derive(Clone, Debug)]
pub(crate) struct Suffix {
    /// Raw byte pattern.
    pub needle: Box<[u8]>,

    /// Reverse finder for the needle.
    pub finder: FinderRev<'static>,
}

/// Pre-computed suffix needles for parameter matching.
#[derive(Clone, Default, Debug)]
pub(crate) struct Suffixes(Box<[Suffix]>);

impl Suffixes {
    /// Returns the length of the longest suffix needle, or 0 if empty.
    pub(crate) fn longest(&self) -> usize {
        self.0.first().map_or(0, |suffix| suffix.needle.len())
    }

    /// Returns `true` if `after` starts with any suffix needle.
    pub(crate) fn matches(&self, after: &[u8]) -> bool {
        self.0.iter().any(|entry| {
            after.len() >= entry.needle.len() && entry.needle.iter().zip(after).all(|(a, b)| a == b)
        })
    }

    /// Yields candidate boundary positions within `remaining`, searching backwards.
    pub(crate) fn positions<'a>(
        &'a self,
        path: &'a str,
        offset: usize,
        max: usize,
        limit: Option<usize>,
    ) -> impl Iterator<Item = usize> + 'a {
        let remaining = &path.as_bytes()[offset..];
        let cap = limit.map_or(max, |limit| limit.min(max));

        self.0.iter().flat_map(move |suffix| {
            let mut end = (cap + suffix.needle.len()).min(remaining.len());

            core::iter::from_fn(move || {
                loop {
                    let position = suffix.finder.rfind(&remaining[..end])?;
                    if position == 0 {
                        return None;
                    }

                    end = position;

                    if path.is_char_boundary(offset + position) {
                        return Some(position);
                    }
                }
            })
        })
    }

    /// Updates the suffixes from the static children of a node.
    pub(crate) fn update<S, T>(
        node: &mut Node<S, T>,
        current: &mut Vec<u8>,
        seen: &mut HashSet<Vec<u8>, FxBuildHasher>,
    ) {
        let new = collect(&node.static_children, current, seen);
        let unchanged = node.suffixes.0.len() == new.len()
            && node
                .suffixes
                .0
                .iter()
                .zip(&new)
                .all(|(entry, bytes)| *entry.needle == *bytes.as_slice());

        if !unchanged {
            node.suffixes = Self(
                new.into_iter()
                    .map(|bytes| {
                        let finder = FinderRev::new(&bytes).into_owned();
                        Suffix {
                            needle: bytes.into_boxed_slice(),
                            finder,
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
        }
    }
}

/// Collects static suffixes from children for parameter matching.
fn collect<T>(
    children: &[Node<StaticState, T>],
    current: &mut Vec<u8>,
    seen: &mut HashSet<Vec<u8>, FxBuildHasher>,
) -> Vec<Vec<u8>> {
    seen.clear();
    collect_recursive(children, current, seen);

    let mut suffixes: Vec<Vec<u8>> = seen.iter().cloned().collect();
    suffixes.sort_by_key(|suffix| Reverse(suffix.len()));
    suffixes
}

fn collect_recursive<T>(
    children: &[Node<StaticState, T>],
    current: &mut Vec<u8>,
    seen: &mut HashSet<Vec<u8>, FxBuildHasher>,
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

        collect_recursive(&child.static_children, current, seen);
        current.truncate(current.len() - child.state.prefix.len());
    }
}
