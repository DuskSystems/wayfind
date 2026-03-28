use alloc::vec;
use alloc::vec::Vec;

use smallvec::SmallVec;

use crate::node::{Node, NodeData};

/// Bitset for memoizing failed searches.
pub(crate) struct Visited {
    bits: Vec<u64>,
    stride: usize,
}

impl Visited {
    fn new(count: usize, len: usize) -> Self {
        let stride = len + 1;

        let total = count.saturating_mul(stride);
        let blocks = total.div_ceil(64);

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
            .is_some_and(|&word| word & (1 << bit) != 0)
    }

    fn insert(&mut self, id: usize, offset: usize) {
        let index = id * self.stride + offset;

        let block = index / 64;
        let bit = index % 64;

        if let Some(word) = self.bits.get_mut(block) {
            *word |= 1 << bit;
        }
    }
}

/// Per search state.
pub(crate) struct SearchState<'r, 'p> {
    pub path: &'p str,
    pub parameters: SmallVec<[(&'r str, &'p str); 4]>,
    pub visited: Option<Visited>,
    pub count: usize,
}

impl<'p> SearchState<'_, 'p> {
    pub(crate) fn new(path: &'p str, count: usize) -> Self {
        Self {
            path,
            parameters: SmallVec::new(),
            visited: None,
            count,
        }
    }

    fn enable_visited(&mut self) {
        if self.visited.is_none() {
            self.visited = Some(Visited::new(self.count, self.path.len()));
        }
    }
}

impl<S> Node<S> {
    /// Searches for a matching template in the node tree.
    pub(crate) fn search<'r>(&'r self, state: &mut SearchState<'r, '_>) -> Option<&'r NodeData> {
        self.search_at(0, state)
    }

    fn search_at<'r>(
        &'r self,
        offset: usize,
        state: &mut SearchState<'r, '_>,
    ) -> Option<&'r NodeData> {
        if let Some(visited) = &state.visited {
            if visited.contains(self.id, offset) {
                return None;
            }
        }

        let result = self.search_inner(offset, state);

        if result.is_none() {
            if let Some(visited) = &mut state.visited {
                visited.insert(self.id, offset);
            }
        }

        result
    }

    fn search_inner<'r>(
        &'r self,
        offset: usize,
        state: &mut SearchState<'r, '_>,
    ) -> Option<&'r NodeData> {
        if offset >= state.path.len() {
            return if offset == state.path.len() {
                self.data.as_ref()
            } else {
                None
            };
        }

        if state.path.len() - offset < self.shortest {
            return None;
        }

        if let Some(search) = self.search_static(offset, state) {
            return Some(search);
        }

        if let Some(search) = if self.flags.is_dynamic_segment_only() {
            self.search_dynamic_segment(offset, state)
        } else {
            self.search_dynamic_inline(offset, state)
        } {
            return Some(search);
        }

        if let Some(search) = if self.flags.is_wildcard_segment_only() {
            self.search_wildcard_segment(offset, state)
        } else {
            self.search_wildcard_inline(offset, state)
        } {
            return Some(search);
        }

        if let Some(child) = &self.end_wildcard {
            state.parameters.push((&child.name, &state.path[offset..]));
            return Some(&child.data);
        }

        None
    }

    /// Matches static children by prefix byte comparison.
    fn search_static<'r>(
        &'r self,
        offset: usize,
        state: &mut SearchState<'r, '_>,
    ) -> Option<&'r NodeData> {
        let remaining = &state.path.as_bytes()[offset..];
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
                if let Some(data) = child.search_at(end, state) {
                    return Some(data);
                }
            }
        }

        None
    }

    /// Matches segment dynamic parameters like `/<name>/`.
    fn search_dynamic_segment<'r>(
        &'r self,
        offset: usize,
        state: &mut SearchState<'r, '_>,
    ) -> Option<&'r NodeData> {
        if self.dynamic_children.is_empty() {
            return None;
        }

        let remaining = &state.path.as_bytes()[offset..];
        let limit = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());
        if limit == 0 {
            return None;
        }

        let segment = &state.path[offset..offset + limit];

        for child in &self.dynamic_children {
            if remaining.len() - limit < child.shortest {
                continue;
            }

            if !child.tails.is_empty()
                && !child.tails.iter().any(|tail| {
                    remaining.len() >= tail.len()
                        && tail
                            .iter()
                            .rev()
                            .zip(remaining.iter().rev())
                            .all(|(a, b)| a == b)
                })
            {
                continue;
            }

            state.enable_visited();
            state.parameters.push((&child.state.name, segment));

            if let Some(result) = child.search_at(offset + limit, state) {
                return Some(result);
            }

            state.parameters.pop();
        }

        None
    }

    /// Matches inline dynamic parameters like `/<name>.txt`.
    fn search_dynamic_inline<'r>(
        &'r self,
        offset: usize,
        state: &mut SearchState<'r, '_>,
    ) -> Option<&'r NodeData> {
        let remaining = &state.path.as_bytes()[offset..];
        let limit = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());

        for child in &self.dynamic_children {
            if remaining.len() <= child.shortest {
                continue;
            }

            if !child.tails.is_empty()
                && !child.tails.iter().any(|tail| {
                    remaining.len() >= tail.len()
                        && tail
                            .iter()
                            .rev()
                            .zip(remaining.iter().rev())
                            .all(|(a, b)| a == b)
                })
            {
                continue;
            }

            state.enable_visited();
            let max = remaining.len() - child.shortest;

            for suffix in &child.state.suffixes {
                let mut end = (limit.min(max) + suffix.needle().len()).min(remaining.len());

                while let Some(position) = suffix.rfind(&remaining[..end]) {
                    if position == 0 {
                        break;
                    }

                    if state.path.is_char_boundary(offset + position) {
                        let parameter = &state.path[offset..offset + position];
                        state.parameters.push((&child.state.name, parameter));

                        if let Some(result) = child.search_at(offset + position, state) {
                            return Some(result);
                        }

                        state.parameters.pop();
                    }

                    end = position;
                }
            }
        }

        if limit > 0 {
            for child in &self.dynamic_children {
                if remaining.len() - limit < child.shortest {
                    continue;
                }

                if !child.tails.is_empty()
                    && !child.tails.iter().any(|tail| {
                        remaining.len() >= tail.len()
                            && tail
                                .iter()
                                .rev()
                                .zip(remaining.iter().rev())
                                .all(|(a, b)| a == b)
                    })
                {
                    continue;
                }

                let parameter = &state.path[offset..offset + limit];
                state.parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(offset + limit, state) {
                    return Some(result);
                }

                state.parameters.pop();
            }
        }

        None
    }

    /// Matches segment wildcard parameters like `/<*path>/help`.
    fn search_wildcard_segment<'r>(
        &'r self,
        offset: usize,
        state: &mut SearchState<'r, '_>,
    ) -> Option<&'r NodeData> {
        let remaining = &state.path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            if remaining.len() < child.shortest {
                continue;
            }

            if !child.tails.is_empty()
                && !child.tails.iter().any(|tail| {
                    remaining.len() >= tail.len()
                        && tail
                            .iter()
                            .rev()
                            .zip(remaining.iter().rev())
                            .all(|(a, b)| a == b)
                })
            {
                continue;
            }

            state.enable_visited();
            let max = remaining.len() - child.shortest;

            let positions = core::iter::successors(Some(max), |&position| {
                memchr::memrchr(b'/', &remaining[..position])
            });

            for position in positions.take_while(|&position| position > 0) {
                let after = &remaining[position..];
                if !child.state.suffixes.iter().any(|finder| {
                    let needle = finder.needle();
                    after.len() >= needle.len() && needle.iter().zip(after).all(|(a, b)| a == b)
                }) {
                    continue;
                }

                let parameter = &state.path[offset..offset + position];
                state.parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(offset + position, state) {
                    return Some(result);
                }

                state.parameters.pop();
            }
        }

        None
    }

    /// Matches inline wildcard parameters like `/<*path>.html`.
    fn search_wildcard_inline<'r>(
        &'r self,
        offset: usize,
        state: &mut SearchState<'r, '_>,
    ) -> Option<&'r NodeData> {
        let remaining = &state.path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            if remaining.len() <= child.shortest {
                continue;
            }

            if !child.tails.is_empty()
                && !child.tails.iter().any(|tail| {
                    remaining.len() >= tail.len()
                        && tail
                            .iter()
                            .rev()
                            .zip(remaining.iter().rev())
                            .all(|(a, b)| a == b)
                })
            {
                continue;
            }

            state.enable_visited();
            let max = remaining.len() - child.shortest;

            for suffix in &child.state.suffixes {
                let mut end = (max + suffix.needle().len()).min(remaining.len());

                while let Some(position) = suffix.rfind(&remaining[..end]) {
                    if position == 0 {
                        break;
                    }

                    if state.path.is_char_boundary(offset + position) {
                        let parameter = &state.path[offset..offset + position];
                        state.parameters.push((&child.state.name, parameter));

                        if let Some(result) = child.search_at(offset + position, state) {
                            return Some(result);
                        }

                        state.parameters.pop();
                    }

                    end = position;
                }
            }
        }

        None
    }
}
