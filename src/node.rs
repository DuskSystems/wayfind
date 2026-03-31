use alloc::borrow::ToOwned as _;
use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::ToString as _;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::fmt;

use smallvec::SmallVec;

use crate::bounds::Bounds;
use crate::parser::{Part, Template};
use crate::reachable::{NeedleCache, Reachable};
use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};
use crate::suffixes::Suffixes;

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

/// Data stored at a node that matches a template.
#[derive(Clone, Debug)]
pub(crate) struct Data<T> {
    /// The associated data.
    pub data: T,

    /// This node's template.
    pub template: Box<str>,
}

#[derive(Clone, Debug)]
pub(crate) enum DynamicSearch {
    /// All children are whole segments.
    Segment,
    /// Children may have inline suffixes.
    Inline,
}

#[derive(Clone, Debug)]
pub(crate) enum WildcardSearch {
    /// All children are whole segments.
    Segment,
    /// Children may have inline suffixes.
    Inline,
}

/// Represents a node in the tree structure.
#[derive(Clone, Debug)]
pub(crate) struct Node<S, T> {
    pub state: S,
    pub data: Option<Data<T>>,

    pub static_children: Vec<Node<StaticState, T>>,
    pub dynamic_children: Vec<Node<DynamicState, T>>,
    pub wildcard_children: Vec<Node<WildcardState, T>>,
    pub end_wildcard: Option<EndWildcardState<T>>,

    pub bounds: Bounds,
    pub reachable: Reachable,
    pub suffixes: Suffixes,

    pub dynamic_search: DynamicSearch,
    pub wildcard_search: WildcardSearch,
}

impl<S, T> Node<S, T> {
    /// Creates a new empty node.
    #[must_use]
    pub(crate) fn new(state: S) -> Self {
        Self {
            state,
            data: None,

            static_children: Vec::new(),
            dynamic_children: Vec::new(),
            wildcard_children: Vec::new(),
            end_wildcard: None,

            bounds: Bounds::default(),
            reachable: Reachable::default(),
            suffixes: Suffixes::default(),

            dynamic_search: DynamicSearch::Segment,
            wildcard_search: WildcardSearch::Segment,
        }
    }

    /// Inserts a new route into the node tree with associated data.
    /// Recursively traverses the node tree, creating new nodes as necessary.
    pub(crate) fn insert(&mut self, template: &mut Template<'_>, data: Data<T>) {
        if let Some(part) = template.parts.pop() {
            match part {
                Part::Static { prefix } => self.insert_static(template, data, prefix),
                Part::Dynamic { name } => self.insert_dynamic(template, data, name),
                Part::Wildcard { name } if template.parts.is_empty() => {
                    self.insert_end_wildcard(data, name);
                }
                Part::Wildcard { name } => self.insert_wildcard(template, data, name),
            }
        } else {
            self.data = Some(data);
        }
    }

    fn insert_static(&mut self, template: &mut Template<'_>, data: Data<T>, prefix: &[u8]) {
        if let Some(child) = self
            .static_children
            .iter_mut()
            .find(|child| child.state.prefix[0] == prefix[0])
        {
            let common_prefix = prefix
                .iter()
                .zip(&child.state.prefix)
                .take_while(|&(a, b)| a == b)
                .count();

            // If the new prefix matches or extends the existing prefix, insert directly.
            if common_prefix >= child.state.prefix.len() {
                if common_prefix >= prefix.len() {
                    child.insert(template, data);
                } else {
                    child.insert_static(template, data, &prefix[common_prefix..]);
                }

                return;
            }

            // Not a clean insert, need to split the existing child node.
            let new_child_a = Node {
                state: StaticState::new(&child.state.prefix[common_prefix..]),
                data: child.data.take(),

                static_children: core::mem::take(&mut child.static_children),
                dynamic_children: core::mem::take(&mut child.dynamic_children),
                wildcard_children: core::mem::take(&mut child.wildcard_children),
                end_wildcard: core::mem::take(&mut child.end_wildcard),

                dynamic_search: child.dynamic_search.clone(),
                wildcard_search: child.wildcard_search.clone(),
                bounds: child.bounds.clone(),
                reachable: core::mem::take(&mut child.reachable),
                suffixes: core::mem::take(&mut child.suffixes),
            };

            let new_child_b = Node::new(StaticState::new(&prefix[common_prefix..]));

            child.state = StaticState::new(&child.state.prefix[..common_prefix]);

            if prefix[common_prefix..].is_empty() {
                child.static_children = vec![new_child_a];
                child.insert(template, data);
            } else {
                child.static_children = vec![new_child_a, new_child_b];
                child.static_children[1].insert(template, data);
            }

            return;
        }

        let mut child = Node::new(StaticState::new(prefix));
        child.insert(template, data);
        self.static_children.push(child);
    }

    fn insert_dynamic(&mut self, template: &mut Template<'_>, data: Data<T>, name: &str) {
        if let Some(child) = self
            .dynamic_children
            .iter_mut()
            .find(|child| *child.state.name == *name)
        {
            child.insert(template, data);
        } else {
            let mut child = Node::new(DynamicState::new(name));
            child.insert(template, data);
            self.dynamic_children.push(child);
        }
    }

    fn insert_wildcard(&mut self, template: &mut Template<'_>, data: Data<T>, name: &str) {
        if let Some(child) = self
            .wildcard_children
            .iter_mut()
            .find(|child| *child.state.name == *name)
        {
            child.insert(template, data);
        } else {
            let mut child = Node::new(WildcardState::new(name));
            child.insert(template, data);
            self.wildcard_children.push(child);
        }
    }

    fn insert_end_wildcard(&mut self, data: Data<T>, name: &str) {
        self.end_wildcard = Some(EndWildcardState::new(name, data));
    }

    /// Checks if a template conflicts with an existing template.
    /// Handles both direct and structural conflicts.
    pub(crate) fn conflict(&self, parts: &[Part<'_>]) -> Option<&Data<T>> {
        let Some((part, remaining)) = parts.split_last() else {
            return self.data.as_ref();
        };

        match part {
            Part::Static { prefix } => self.conflict_static(remaining, prefix),
            Part::Dynamic { .. } => self.conflict_dynamic(remaining),
            Part::Wildcard { .. } if remaining.is_empty() => self.conflict_end_wildcard(),
            Part::Wildcard { .. } => self.conflict_wildcard(remaining),
        }
    }

    fn conflict_static(&self, parts: &[Part<'_>], prefix: &[u8]) -> Option<&Data<T>> {
        for child in &self.static_children {
            if prefix.len() >= child.state.prefix.len()
                && child.state.prefix.iter().zip(prefix).all(|(a, b)| a == b)
            {
                let remaining_prefix = &prefix[child.state.prefix.len()..];
                if remaining_prefix.is_empty() {
                    if let Some(data) = child.conflict(parts) {
                        return Some(data);
                    }
                } else if let Some(data) = child.conflict_static(parts, remaining_prefix) {
                    return Some(data);
                }
            }
        }

        None
    }

    fn conflict_dynamic(&self, parts: &[Part<'_>]) -> Option<&Data<T>> {
        for child in &self.dynamic_children {
            if let Some(data) = child.conflict(parts) {
                return Some(data);
            }
        }

        None
    }

    fn conflict_wildcard(&self, parts: &[Part<'_>]) -> Option<&Data<T>> {
        for child in &self.wildcard_children {
            if let Some(data) = child.conflict(parts) {
                return Some(data);
            }
        }

        None
    }

    fn conflict_end_wildcard(&self) -> Option<&Data<T>> {
        let child = self.end_wildcard.as_ref()?;
        Some(&child.data)
    }

    /// Optimizes the tree.
    pub(crate) fn optimize(&mut self) {
        let mut needles = BTreeMap::new();
        self.optimize_inner(&mut needles);
    }

    fn optimize_inner(&mut self, needles: &mut BTreeMap<Box<[u8]>, usize>) {
        for child in &mut self.static_children {
            child.optimize_inner(needles);
        }

        let mut seen = BTreeSet::new();
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

        self.dynamic_search = if self.dynamic_children.iter().all(|node| {
            node.dynamic_children.is_empty()
                && node.wildcard_children.is_empty()
                && node.end_wildcard.is_none()
                && node
                    .static_children
                    .iter()
                    .all(|child| child.state.prefix.first() == Some(&b'/'))
        }) {
            DynamicSearch::Segment
        } else {
            DynamicSearch::Inline
        };

        self.wildcard_search = if self.wildcard_children.iter().all(|node| {
            node.dynamic_children.is_empty()
                && node.wildcard_children.is_empty()
                && node.end_wildcard.is_none()
                && node
                    .static_children
                    .iter()
                    .all(|child| child.state.prefix.first() == Some(&b'/'))
        }) {
            WildcardSearch::Segment
        } else {
            WildcardSearch::Inline
        };

        self.bounds = Bounds::new(self);
        self.reachable = Reachable::compute(self, needles);
    }

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

        let length = path.len() - offset;
        if length < self.bounds.shortest() || length > self.bounds.longest() {
            return None;
        }

        if let Some(result) = self.search_static(search, path, offset) {
            return Some(result);
        }

        let dynamic = match self.dynamic_search {
            DynamicSearch::Segment => self.search_dynamic_segment(search, path, offset),
            DynamicSearch::Inline => self.search_dynamic_inline(search, path, offset),
        };

        if let Some(result) = dynamic {
            return Some(result);
        }

        let wildcard = match self.wildcard_search {
            WildcardSearch::Segment => self.search_wildcard_segment(search, path, offset),
            WildcardSearch::Inline => self.search_wildcard_inline(search, path, offset),
        };

        if let Some(result) = wildcard {
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
            search.parameters.push((&child.name, &path[offset..]));
            return Some(&child.data);
        }

        None
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

            if let Some(wildcard) = &node.end_wildcard {
                let branch = if key.is_empty() { "" } else { "╰─ " };
                writeln!(f, "{padding}{branch}{wildcard}")?;
            }

            Ok(())
        }

        display_node(f, self, "", true, true)
    }
}
