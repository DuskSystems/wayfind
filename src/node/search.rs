use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use smallvec::SmallVec;

use crate::node::{Node, NodeData, Reachable};

/// Bitset for memoizing failed searches.
struct Visited {
    bits: Vec<u64>,
    stride: usize,
}

impl Visited {
    fn new(count: usize, len: usize) -> Self {
        let stride = len + 1;

        let bits = count.saturating_mul(stride);
        let blocks = bits.div_ceil(64);

        Self {
            bits: vec![0_u64; blocks],
            stride,
        }
    }

    fn contains(&self, id: usize, offset: usize) -> bool {
        let index = id * self.stride + offset;

        let block = index / 64;
        let bit = index % 64;

        self.bits
            .get(block)
            .is_some_and(|&word| word & (1_u64 << bit) != 0)
    }

    fn insert(&mut self, id: usize, offset: usize) {
        let index = id * self.stride + offset;

        let block = index / 64;
        let bit = index % 64;

        if let Some(word) = self.bits.get_mut(block) {
            *word |= 1_u64 << bit;
        }
    }
}

/// Lazily allocated cache for reachability checks and slash positions.
struct SearchCache {
    contains: Vec<Option<usize>>,
    slashes: Option<Vec<usize>>,
}

/// Per-search mutable state.
///
/// The `visited` bitset is stored directly for fast access in the backtracking
/// hot loop. The rest is boxed behind `SearchCache` to keep the struct small.
pub(crate) struct SearchState {
    count: usize,
    needles: usize,
    visited: Option<Visited>,
    cache: Option<Box<SearchCache>>,
}

impl SearchState {
    pub(crate) const fn new(count: usize, needles: usize) -> Self {
        Self {
            count,
            needles,
            visited: None,
            cache: None,
        }
    }

    #[cold]
    fn enable_visited(&mut self, path_len: usize) {
        if self.visited.is_none() {
            self.visited = Some(Visited::new(self.count, path_len));
        }
    }

    #[cold]
    fn cache(&mut self) -> &mut SearchCache {
        let needles = self.needles;
        self.cache.get_or_insert_with(|| {
            Box::new(SearchCache {
                contains: vec![None; needles],
                slashes: None,
            })
        })
    }
}

impl<S> Node<S> {
    /// Searches for a matching template in the node tree.
    pub(crate) fn search<'r, 'p>(
        &'r self,
        path: &'p str,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
        state: &mut SearchState,
    ) -> Option<&'r NodeData> {
        self.search_at(path, 0, parameters, state)
    }

    fn search_at<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
        state: &mut SearchState,
    ) -> Option<&'r NodeData> {
        if offset >= path.len() {
            return if offset == path.len() {
                self.data.as_ref()
            } else {
                None
            };
        }

        let remaining_len = path.len() - offset;

        if remaining_len < self.shortest || remaining_len > self.longest {
            return None;
        }

        if let Some(search) = self.search_static(path, offset, parameters, state) {
            return Some(search);
        }

        if let Some(search) = if self.flags.is_dynamic_segment_only() {
            self.search_dynamic_segment(path, offset, parameters, state)
        } else {
            self.search_dynamic_inline(path, offset, parameters, state)
        } {
            return Some(search);
        }

        if let Some(search) = if self.flags.is_wildcard_segment_only() {
            self.search_wildcard_segment(path, offset, parameters, state)
        } else {
            self.search_wildcard_inline(path, offset, parameters, state)
        } {
            return Some(search);
        }

        if let Some(child) = self.end_wildcard.as_deref() {
            parameters.push((&child.state.name, &path[offset..]));
            return child.data.as_ref();
        }

        None
    }

    /// Matches static children by prefix byte comparison.
    fn search_static<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
        state: &mut SearchState,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];
        let first = *remaining.first()?;

        for child in &self.static_children {
            if child.state.first != first {
                continue;
            }

            let prefix = &child.state.prefix;
            if remaining.len() >= prefix.len() && prefix.iter().zip(remaining).all(|(a, b)| a == b)
            {
                let end = offset + prefix.len();
                if let Some(data) = child.search_at(path, end, parameters, state) {
                    return Some(data);
                }
            }
        }

        None
    }

    /// Matches segment dynamic parameters like `/<name>/`.
    fn search_dynamic_segment<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
        state: &mut SearchState,
    ) -> Option<&'r NodeData> {
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
            if remaining.len() - limit < child.shortest {
                continue;
            }

            if !child.has_matching_tail(remaining) {
                continue;
            }

            parameters.push((&child.state.name, segment));

            if let Some(result) = child.search_at(path, boundary, parameters, state) {
                return Some(result);
            }

            parameters.pop();
        }

        None
    }

    /// Matches inline dynamic parameters like `/<name>.txt`.
    #[inline(never)]
    fn search_dynamic_inline<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
        state: &mut SearchState,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];
        let limit = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());

        for child in &self.dynamic_children {
            if remaining.len() <= child.shortest {
                continue;
            }

            if !child.is_reachable(path, offset, state) {
                continue;
            }

            let needs_cache = child.flags.is_needs_cache();
            if needs_cache {
                state.enable_visited(path.len());
            }

            if let Some(result) = child.try_suffix_positions(
                path,
                offset,
                Some(limit),
                needs_cache,
                &child.state.name,
                parameters,
                state,
            ) {
                return Some(result);
            }
        }

        if limit > 0 {
            let boundary = offset + limit;

            for child in &self.dynamic_children {
                if remaining.len() - limit < child.shortest {
                    continue;
                }

                if let Some(visited) = &state.visited
                    && visited.contains(child.id, boundary)
                {
                    continue;
                }

                if !child.is_reachable(path, offset, state) {
                    continue;
                }

                let child_needs_cache = child.flags.is_needs_cache();

                if let Some(result) = child.try_boundary(
                    path,
                    offset,
                    boundary,
                    child_needs_cache,
                    &child.state.name,
                    parameters,
                    state,
                ) {
                    return Some(result);
                }
            }
        }

        None
    }

    /// Matches segment wildcard parameters like `/<*path>/help`.
    fn search_wildcard_segment<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
        state: &mut SearchState,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            if remaining.len() < child.shortest {
                continue;
            }

            if !child.has_matching_tail(remaining) {
                continue;
            }

            let needs_cache = child.flags.is_needs_cache();
            let max = remaining.len() - child.shortest;

            if needs_cache {
                state.enable_visited(path.len());
            }

            let abs_max = offset + max;
            let bytes = path.as_bytes();
            let slashes = state
                .cache()
                .slashes
                .get_or_insert_with(|| memchr::memchr_iter(b'/', bytes).collect());
            let start = slashes.partition_point(|&pos| pos < offset);
            let end = slashes.partition_point(|&pos| pos < abs_max);

            if max > 0
                && child.suffixes.iter().any(|entry| {
                    let after = &remaining[max..];
                    after.len() >= entry.needle.len()
                        && entry.needle.iter().zip(after).all(|(a, b)| a == b)
                })
                && let Some(result) = child.try_boundary(
                    path,
                    offset,
                    offset + max,
                    needs_cache,
                    &child.state.name,
                    parameters,
                    state,
                )
            {
                return Some(result);
            }

            for slash_index in (start..end).rev() {
                let cache = state.cache();
                let Some(slashes) = &cache.slashes else {
                    break;
                };
                let pos = slashes[slash_index];
                let relative = pos - offset;
                if relative == 0 || relative == max {
                    continue;
                }

                let after = &remaining[relative..];
                if !child.suffixes.iter().any(|entry| {
                    after.len() >= entry.needle.len()
                        && entry.needle.iter().zip(after).all(|(a, b)| a == b)
                }) {
                    continue;
                }

                if let Some(result) = child.try_boundary(
                    path,
                    offset,
                    offset + relative,
                    needs_cache,
                    &child.state.name,
                    parameters,
                    state,
                ) {
                    return Some(result);
                }
            }
        }

        None
    }

    /// Matches inline wildcard parameters like `/<*path>.html`.
    fn search_wildcard_inline<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
        state: &mut SearchState,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            if remaining.len() <= child.shortest {
                continue;
            }

            if !child.is_reachable(path, offset, state) {
                continue;
            }

            let needs_cache = child.flags.is_needs_cache();
            if needs_cache {
                state.enable_visited(path.len());
            }

            if let Some(result) = child.try_suffix_positions(
                path,
                offset,
                None,
                needs_cache,
                &child.state.name,
                parameters,
                state,
            ) {
                return Some(result);
            }
        }

        None
    }

    /// Returns `true` if the remaining path ends with any of this node's tail suffixes.
    fn has_matching_tail(&self, remaining: &[u8]) -> bool {
        self.tails.is_empty()
            || self.tails.iter().any(|tail| {
                remaining.len() >= tail.len()
                    && tail
                        .iter()
                        .rev()
                        .zip(remaining.iter().rev())
                        .all(|(a, b)| a == b)
            })
    }

    fn is_reachable(&self, path: &str, offset: usize, state: &mut SearchState) -> bool {
        if !self.tails.is_empty() {
            return self.has_matching_tail(&path.as_bytes()[offset..]);
        }

        if self.reachable.is_empty() {
            return true;
        }

        self.is_reachable_check(offset, &path.as_bytes()[offset..], path, state)
    }

    #[cold]
    fn is_reachable_check(
        &self,
        offset: usize,
        remaining: &[u8],
        path: &str,
        state: &mut SearchState,
    ) -> bool {
        let bytes = path.as_bytes();
        let mut group_passed = true;

        let cache = state.cache();
        let contains = &mut cache.contains;

        for check in &*self.reachable {
            match check {
                Reachable::End => {
                    if group_passed {
                        return true;
                    }

                    group_passed = true;
                }
                Reachable::Suffix(suffix) => {
                    if group_passed {
                        group_passed = remaining.len() >= suffix.len()
                            && suffix
                                .iter()
                                .rev()
                                .zip(remaining.iter().rev())
                                .all(|(a, b)| a == b);
                    }
                }
                Reachable::Contains {
                    needle, needle_id, ..
                } => {
                    if group_passed {
                        let rightmost = *contains[*needle_id].get_or_insert_with(|| {
                            memchr::memmem::rfind(bytes, needle).unwrap_or(usize::MAX)
                        });

                        group_passed = rightmost != usize::MAX && rightmost >= offset;
                    }
                }
                Reachable::Flexible => {}
            }
        }

        false
    }

    /// Attempts to match at a single boundary position.
    ///
    /// Handles the visited-check, parameter push/pop, and recursion.
    /// The `name` parameter is the name of the parameter being matched, passed
    /// explicitly because `self` is generic and may not have a `name` field.
    #[inline]
    #[expect(
        clippy::too_many_arguments,
        reason = "name must be passed explicitly for generic nodes"
    )]
    fn try_boundary<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        boundary: usize,
        needs_cache: bool,
        name: &'r str,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
        state: &mut SearchState,
    ) -> Option<&'r NodeData> {
        if !path.is_char_boundary(boundary) {
            return None;
        }

        if needs_cache
            && let Some(visited) = &state.visited
            && visited.contains(self.id, boundary)
        {
            return None;
        }

        let parameter = &path[offset..boundary];
        parameters.push((name, parameter));

        if let Some(result) = self.search_at(path, boundary, parameters, state) {
            return Some(result);
        }

        parameters.pop();

        if needs_cache && let Some(visited) = &mut state.visited {
            visited.insert(self.id, boundary);
        }

        None
    }

    /// Iterates suffix needle positions in reverse to find candidate boundaries.
    ///
    /// The `limit` constrains the upper bound: `Some(segment_limit)` for dynamic
    /// parameters (which cannot cross `/`), `None` for wildcard parameters.
    /// The `name` parameter is the name of the parameter being matched.
    #[expect(
        clippy::too_many_arguments,
        reason = "name must be passed explicitly for generic nodes"
    )]
    fn try_suffix_positions<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        limit: Option<usize>,
        needs_cache: bool,
        name: &'r str,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
        state: &mut SearchState,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];
        let max = remaining.len() - self.shortest;

        for suffix in &*self.suffixes {
            let needle_len = suffix.needle.len();
            let capped_max = limit.map_or(max, |segment_limit| segment_limit.min(max));
            let mut end = (capped_max + needle_len).min(remaining.len());

            while let Some(position) = suffix.finder.rfind(&remaining[..end]) {
                if position == 0 {
                    break;
                }

                let boundary = offset + position;
                if let Some(result) =
                    self.try_boundary(path, offset, boundary, needs_cache, name, parameters, state)
                {
                    return Some(result);
                }

                end = position;
            }
        }

        None
    }
}
