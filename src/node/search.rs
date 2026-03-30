use alloc::collections::BTreeSet;

use smallvec::SmallVec;

use crate::node::reachable::NeedleCache;
use crate::node::{Data, Node};

/// Memoizes failed searches.
struct Visited(BTreeSet<(usize, usize)>);

impl Visited {
    const fn new() -> Self {
        Self(BTreeSet::new())
    }

    fn contains<S, T>(&self, node: &Node<S, T>, offset: usize) -> bool {
        let ptr = core::ptr::from_ref(node) as usize;
        self.0.contains(&(ptr, offset))
    }

    fn mark<S, T>(&mut self, node: &Node<S, T>, offset: usize) {
        let ptr = core::ptr::from_ref(node) as usize;
        self.0.insert((ptr, offset));
    }
}

/// Per-search state.
pub(crate) struct Search<'r, 'p> {
    /// Failed searches.
    visited: Visited,

    /// Cached needle positions.
    needles: NeedleCache,

    /// Key-value pairs of matched parameters.
    pub parameters: SmallVec<[(&'r str, &'p str); 4]>,
}

impl Search<'_, '_> {
    pub(crate) fn new() -> Self {
        Self {
            visited: Visited::new(),
            needles: NeedleCache::new(),
            parameters: SmallVec::new(),
        }
    }
}

impl<S, T> Node<S, T> {
    /// Searches for a matching template in the node tree.
    pub(crate) fn search<'r, 'p>(
        &'r self,
        search: &mut Search<'r, 'p>,
        path: &'p str,
    ) -> Option<&'r Data<T>> {
        self.search_at(search, path, 0)
    }

    fn search_at<'r, 'p>(
        &'r self,
        search: &mut Search<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        if offset == path.len() {
            return self.data.as_ref();
        }

        if !self.bounds.matches(path.len() - offset) {
            return None;
        }

        if let Some(result) = self.search_static(search, path, offset) {
            return Some(result);
        }

        if let Some(result) = if self.flags.dynamic_segment_only() {
            self.search_dynamic_segment(search, path, offset)
        } else {
            self.search_dynamic_inline(search, path, offset)
        } {
            return Some(result);
        }

        if let Some(result) = if self.flags.wildcard_segment_only() {
            self.search_wildcard_segment(search, path, offset)
        } else {
            self.search_wildcard_inline(search, path, offset)
        } {
            return Some(result);
        }

        self.search_end_wildcard(search, path, offset)
    }

    /// Matches static children by prefix byte comparison.
    fn search_static<'r, 'p>(
        &'r self,
        search: &mut Search<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let remaining = &path.as_bytes()[offset..];
        let first = *remaining.first()?;

        for child in &self.static_children {
            if child.state.first != first {
                continue;
            }

            if remaining.len() >= child.state.prefix.len()
                && child
                    .state
                    .prefix
                    .iter()
                    .zip(remaining)
                    .all(|(a, b)| a == b)
            {
                let end = offset + child.state.prefix.len();
                if let Some(data) = child.search_at(search, path, end) {
                    return Some(data);
                }
            }
        }

        None
    }

    /// Matches segment dynamic parameters like `/<name>/`.
    fn search_dynamic_segment<'r, 'p>(
        &'r self,
        search: &mut Search<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        if self.dynamic_children.is_empty() {
            return None;
        }

        let remaining = &path.as_bytes()[offset..];
        let limit = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());
        if limit == 0 {
            return None;
        }

        let boundary = offset + limit;
        let segment = &path[offset..boundary];

        for child in &self.dynamic_children {
            if remaining.len() - limit < child.bounds.shortest() {
                continue;
            }

            if !child.reachable.check(path, offset, &mut search.needles) {
                continue;
            }

            search.parameters.push((&child.state.name, segment));

            if let Some(result) = child.search_at(search, path, boundary) {
                return Some(result);
            }

            search.parameters.pop();
        }

        None
    }

    /// Matches inline dynamic parameters like `/<name>.txt`.
    fn search_dynamic_inline<'r, 'p>(
        &'r self,
        search: &mut Search<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let remaining = &path.as_bytes()[offset..];
        let limit = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());

        for child in &self.dynamic_children {
            if remaining.len() <= child.bounds.shortest() {
                continue;
            }

            if !child.reachable.check(path, offset, &mut search.needles) {
                continue;
            }

            let max = remaining.len() - child.bounds.shortest();
            for position in child.suffixes.positions(path, offset, max, Some(limit)) {
                let boundary = offset + position;

                if search.visited.contains(child, boundary) {
                    continue;
                }

                search
                    .parameters
                    .push((&child.state.name, &path[offset..boundary]));

                if let Some(result) = child.search_at(search, path, boundary) {
                    return Some(result);
                }

                search.parameters.pop();
                search.visited.mark(child, boundary);
            }
        }

        if limit > 0 {
            let boundary = offset + limit;

            for child in &self.dynamic_children {
                if remaining.len() - limit < child.bounds.shortest() {
                    continue;
                }

                if !child.reachable.check(path, offset, &mut search.needles) {
                    continue;
                }

                if search.visited.contains(child, boundary) {
                    continue;
                }

                search
                    .parameters
                    .push((&child.state.name, &path[offset..boundary]));

                if let Some(result) = child.search_at(search, path, boundary) {
                    return Some(result);
                }

                search.parameters.pop();
                search.visited.mark(child, boundary);
            }
        }

        None
    }

    /// Matches segment wildcard parameters like `/<*path>/help`.
    fn search_wildcard_segment<'r, 'p>(
        &'r self,
        search: &mut Search<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            if remaining.len() < child.bounds.shortest() {
                continue;
            }

            if !child.reachable.check(path, offset, &mut search.needles) {
                continue;
            }

            let max = remaining.len() - child.bounds.shortest();
            let positions = core::iter::successors(Some(max), |&position| {
                memchr::memrchr(b'/', &remaining[..position])
            });

            for position in positions.take_while(|&position| position > 0) {
                let after = &remaining[position..];
                if !child.suffixes.matches(after) {
                    continue;
                }

                let boundary = offset + position;
                if search.visited.contains(child, boundary) {
                    continue;
                }

                search
                    .parameters
                    .push((&child.state.name, &path[offset..boundary]));

                if let Some(result) = child.search_at(search, path, boundary) {
                    return Some(result);
                }

                search.parameters.pop();
                search.visited.mark(child, boundary);
            }
        }

        None
    }

    /// Matches inline wildcard parameters like `/<*path>.html`.
    fn search_wildcard_inline<'r, 'p>(
        &'r self,
        search: &mut Search<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            if remaining.len() <= child.bounds.shortest() {
                continue;
            }

            if !child.reachable.check(path, offset, &mut search.needles) {
                continue;
            }

            let max = remaining.len() - child.bounds.shortest();
            for position in child.suffixes.positions(path, offset, max, None) {
                let boundary = offset + position;

                if search.visited.contains(child, boundary) {
                    continue;
                }

                search
                    .parameters
                    .push((&child.state.name, &path[offset..boundary]));

                if let Some(result) = child.search_at(search, path, boundary) {
                    return Some(result);
                }

                search.parameters.pop();
                search.visited.mark(child, boundary);
            }
        }

        None
    }

    /// Matches end wildcard parameters like `/<*path>`.
    fn search_end_wildcard<'r, 'p>(
        &'r self,
        search: &mut Search<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        if let Some(child) = &self.end_wildcard {
            search.parameters.push((&child.state.name, &path[offset..]));
            return child.data.as_ref();
        }

        None
    }
}
