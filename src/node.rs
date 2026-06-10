use alloc::borrow::ToOwned as _;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString as _;
use core::fmt;

use hashbrown::{HashMap, HashSet};
use rustc_hash::FxBuildHasher;
use smallvec::SmallVec;

use crate::bounds::Bounds;
use crate::needle::NeedleCache;
use crate::reachable::Reachable;
use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};
use crate::suffixes::Suffixes;

/// Per-search state.
pub(crate) struct SearchContext<'r, 'p> {
    needles: NeedleCache,

    /// Boundaries already tried and failed.
    misses: HashSet<(usize, usize), FxBuildHasher>,

    /// Boundaries where every higher boundary has already been tried.
    caps: HashMap<usize, usize, FxBuildHasher>,

    pub parameters: SmallVec<[(&'r str, &'p str); 4]>,
}

impl SearchContext<'_, '_> {
    pub(crate) fn new() -> Self {
        Self {
            needles: NeedleCache::new(),
            misses: HashSet::default(),
            caps: HashMap::default(),
            parameters: SmallVec::new(),
        }
    }

    /// Caps a boundary scan to exclude everything an earlier visit covered.
    fn cap(&self, node: usize, offset: usize, max: usize) -> usize {
        match self.caps.get(&node) {
            Some(&current) => max.min(current.saturating_sub(offset + 1)),
            None => max,
        }
    }

    /// Lowers a node's cap after a visit fails.
    fn lower(&mut self, node: usize, offset: usize) {
        if self.misses.is_empty() {
            return;
        }

        let current = self.caps.entry(node).or_insert(usize::MAX);
        *current = (*current).min(offset + 1);
    }
}

/// Data stored at a leaf node.
#[derive(Clone, Debug)]
pub(crate) struct Data<T> {
    pub data: T,
    pub template: Box<str>,
}

/// Node children search approach.
#[derive(Clone, Debug)]
pub(crate) enum SearchMode {
    /// All children are whole segments.
    Segment,
    /// Children may have inline suffixes.
    Inline,
}

/// An immutable node in the search tree.
#[derive(Clone, Debug)]
pub(crate) struct Node<S, T> {
    pub state: S,
    pub data: Option<Data<T>>,

    pub static_children: Box<[Node<StaticState, T>]>,
    pub dynamic_children: Box<[Node<DynamicState, T>]>,
    pub wildcard_children: Box<[Node<WildcardState, T>]>,
    pub end_wildcard: Option<EndWildcardState<T>>,

    pub bounds: Bounds,
    pub reachable: Reachable,
    pub suffixes: Suffixes,

    pub dynamic_search: SearchMode,
    pub wildcard_search: SearchMode,
}

impl<S, T> Node<S, T> {
    pub(crate) fn has_parameters(&self) -> bool {
        !self.dynamic_children.is_empty()
            || !self.wildcard_children.is_empty()
            || self.end_wildcard.is_some()
    }

    pub(crate) fn is_segment_only(&self) -> bool {
        !self.has_parameters()
            && self
                .static_children
                .iter()
                .all(|child| child.state.prefix.first() == Some(&b'/'))
    }

    pub(crate) fn search<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
    ) -> Option<&'r Data<T>> {
        self.search_at(ctx, path, 0)
    }

    fn search_at<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        if offset == path.len() {
            return self.data.as_ref();
        }

        let length = path.len() - offset;
        if length < self.bounds.shortest() || length > self.bounds.longest() {
            return None;
        }

        if let Some(result) = self.search_static(ctx, path, offset) {
            return Some(result);
        }

        if !self.has_parameters() || !path.is_char_boundary(offset) {
            return None;
        }

        let dynamic = match self.dynamic_search {
            SearchMode::Segment => self.search_dynamic_segment(ctx, path, offset),
            SearchMode::Inline => self.search_dynamic_inline(ctx, path, offset),
        };

        if let Some(result) = dynamic {
            return Some(result);
        }

        let wildcard = match self.wildcard_search {
            SearchMode::Segment => self.search_wildcard_segment(ctx, path, offset),
            SearchMode::Inline => self.search_wildcard_inline(ctx, path, offset),
        };

        if let Some(result) = wildcard {
            return Some(result);
        }

        self.search_end_wildcard(ctx, path, offset)
    }

    fn search_static<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.static_children {
            if remaining.len() >= child.state.prefix.len()
                && child
                    .state
                    .prefix
                    .iter()
                    .zip(remaining)
                    .all(|(a, b)| a == b)
            {
                let end = offset + child.state.prefix.len();
                if let Some(data) = child.search_at(ctx, path, end) {
                    return Some(data);
                }
            }
        }

        None
    }

    fn search_dynamic_segment<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
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

            let ptr = core::ptr::from_ref(child) as usize;
            if ctx.misses.contains(&(ptr, boundary)) {
                continue;
            }

            if !child.reachable.check(&mut ctx.needles, path, offset) {
                continue;
            }

            ctx.parameters.push((&child.state.name, segment));

            if let Some(result) = child.search_at(ctx, path, boundary) {
                return Some(result);
            }

            ctx.parameters.pop();
            ctx.misses.insert((ptr, boundary));
        }

        None
    }

    #[inline(never)]
    fn search_dynamic_inline<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let remaining = &path.as_bytes()[offset..];
        let limit = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());

        for child in &self.dynamic_children {
            if remaining.len() <= child.bounds.shortest() {
                continue;
            }

            if !child.reachable.check(&mut ctx.needles, path, offset) {
                continue;
            }

            let ptr = core::ptr::from_ref(child) as usize;
            let max = remaining.len() - child.bounds.shortest();
            let cap = ctx.cap(ptr, offset, limit.min(max));

            // Try boundaries with known suffix.
            for position in child.suffixes.positions(path, offset, cap) {
                let boundary = offset + position;
                if ctx.misses.contains(&(ptr, boundary)) {
                    continue;
                }

                ctx.parameters
                    .push((&child.state.name, &path[offset..boundary]));

                if let Some(result) = child.search_at(ctx, path, boundary) {
                    return Some(result);
                }

                ctx.parameters.pop();
                ctx.misses.insert((ptr, boundary));
            }

            ctx.lower(ptr, offset);

            // Try full segment boundary.
            if limit > 0 && remaining.len() - limit >= child.bounds.shortest() {
                let boundary = offset + limit;
                if ctx.misses.insert((ptr, boundary)) {
                    ctx.parameters
                        .push((&child.state.name, &path[offset..boundary]));

                    if let Some(result) = child.search_at(ctx, path, boundary) {
                        return Some(result);
                    }

                    ctx.parameters.pop();
                }
            }
        }

        None
    }

    #[inline(never)]
    fn search_wildcard_segment<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            if remaining.len() <= child.bounds.shortest() {
                continue;
            }

            if !child.reachable.check(&mut ctx.needles, path, offset) {
                continue;
            }

            let ptr = core::ptr::from_ref(child) as usize;
            let max = remaining.len() - child.bounds.shortest();
            let cap = ctx.cap(ptr, offset, max);
            let upper = (cap + 1).min(remaining.len());

            let initial = memchr::memrchr(b'/', &remaining[..upper]);
            let positions = core::iter::successors(initial, |&position| {
                memchr::memrchr(b'/', &remaining[..position])
            });

            for position in positions.take_while(|&position| position > 0) {
                let after = &remaining[position..];
                if !child.suffixes.accepts(after) {
                    continue;
                }

                let boundary = offset + position;
                if ctx.misses.contains(&(ptr, boundary)) {
                    continue;
                }

                ctx.parameters
                    .push((&child.state.name, &path[offset..boundary]));

                if let Some(result) = child.search_at(ctx, path, boundary) {
                    return Some(result);
                }

                ctx.parameters.pop();
                ctx.misses.insert((ptr, boundary));
            }

            ctx.lower(ptr, offset);
        }

        None
    }

    #[inline(never)]
    fn search_wildcard_inline<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            if remaining.len() <= child.bounds.shortest() {
                continue;
            }

            if !child.reachable.check(&mut ctx.needles, path, offset) {
                continue;
            }

            let ptr = core::ptr::from_ref(child) as usize;
            let max = remaining.len() - child.bounds.shortest();
            let cap = ctx.cap(ptr, offset, max);

            for position in child.suffixes.positions(path, offset, cap) {
                let boundary = offset + position;
                if ctx.misses.contains(&(ptr, boundary)) {
                    continue;
                }

                ctx.parameters
                    .push((&child.state.name, &path[offset..boundary]));

                if let Some(result) = child.search_at(ctx, path, boundary) {
                    return Some(result);
                }

                ctx.parameters.pop();
                ctx.misses.insert((ptr, boundary));
            }

            ctx.lower(ptr, offset);
        }

        None
    }

    fn search_end_wildcard<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let child = self.end_wildcard.as_ref()?;
        ctx.parameters.push((&child.name, &path[offset..]));
        Some(&child.data)
    }
}

impl<S: fmt::Display, T> fmt::Display for Node<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn display_node<S: fmt::Display, T>(
            f: &mut fmt::Formatter<'_>,
            node: &Node<S, T>,
            padding: &str,
            is_root: bool,
            is_last: bool,
        ) -> fmt::Result {
            let key = node.state.to_string();
            if !key.is_empty() {
                if is_root {
                    writeln!(f, "{key}")?;
                } else {
                    let branch = if is_last { "╰─" } else { "├─" };
                    writeln!(f, "{padding}{branch} {key}")?;
                }
            }

            let padding = if !is_root && !key.is_empty() {
                if is_last {
                    format!("{padding}   ")
                } else {
                    format!("{padding}│  ")
                }
            } else {
                padding.to_owned()
            };

            let mut count = node.static_children.len()
                + node.dynamic_children.len()
                + node.wildcard_children.len()
                + usize::from(node.end_wildcard.is_some());

            for child in &node.static_children {
                count -= 1;
                display_node(f, child, &padding, key.is_empty(), count == 0)?;
            }

            for child in &node.dynamic_children {
                count -= 1;
                display_node(f, child, &padding, key.is_empty(), count == 0)?;
            }

            for child in &node.wildcard_children {
                count -= 1;
                display_node(f, child, &padding, key.is_empty(), count == 0)?;
            }

            if let Some(child) = &node.end_wildcard {
                let branch = if key.is_empty() { "" } else { "╰─ " };
                writeln!(f, "{padding}{branch}{child}")?;
            }

            Ok(())
        }

        display_node(f, self, "", true, true)
    }
}
