use core::ptr;

use smallvec::SmallVec;

use crate::node::{Node, NodeData};

/// Shared search state.
pub(crate) struct SearchContext<'r, 'p> {
    pub parameters: SmallVec<[(&'r str, &'p str); 4]>,
    pub visited: SmallVec<[(usize, usize); 8]>,
}

impl SearchContext<'_, '_> {
    pub(crate) fn new() -> Self {
        Self {
            parameters: SmallVec::new(),
            visited: SmallVec::new(),
        }
    }
}

impl<S> Node<S> {
    /// Searches for a matching template in the node tree.
    pub(crate) fn search<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
    ) -> Option<&'r NodeData> {
        self.search_at(ctx, path, 0)
    }

    fn search_at<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r NodeData> {
        if offset >= path.len() {
            return if offset == path.len() {
                self.data.as_ref()
            } else {
                None
            };
        }

        if path.len() - offset < self.shortest {
            return None;
        }

        if let Some(search) = self.search_static(ctx, path, offset) {
            return Some(search);
        }

        if let Some(search) = if self.dynamic_segment_only {
            self.search_dynamic_segment(ctx, path, offset)
        } else {
            self.search_dynamic_inline(ctx, path, offset)
        } {
            return Some(search);
        }

        if let Some(search) = if self.wildcard_segment_only {
            self.search_wildcard_segment(ctx, path, offset)
        } else {
            self.search_wildcard_inline(ctx, path, offset)
        } {
            return Some(search);
        }

        self.search_end_wildcard(ctx, path, offset)
    }

    /// Matches static children by prefix byte comparison.
    fn search_static<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r NodeData> {
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
                if let Some(data) = child.search_at(ctx, path, end) {
                    return Some(data);
                }
            }
        }

        None
    }

    /// Matches segment dynamic parameters like `/<name>/`.
    fn search_dynamic_segment<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r NodeData> {
        if self.dynamic_children.is_empty() {
            return None;
        }

        let remaining = &path.as_bytes()[offset..];
        let limit = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());
        if limit == 0 {
            return None;
        }

        let end = offset + limit;
        let segment = &path[offset..end];

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

            ctx.parameters.push((&child.state.name, segment));

            if let Some(result) = child.search_at(ctx, path, end) {
                return Some(result);
            }

            ctx.parameters.pop();
        }

        None
    }

    /// Matches inline dynamic parameters like `/<name>.txt`.
    fn search_dynamic_inline<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];
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

            let id = ptr::from_ref(child) as usize;
            let max = remaining.len() - child.shortest;

            for suffix in &child.state.suffixes {
                let mut cursor = (limit.min(max) + suffix.needle().len()).min(remaining.len());

                while let Some(position) = suffix.rfind(&remaining[..cursor]) {
                    if position == 0 {
                        break;
                    }

                    let boundary = offset + position;
                    if path.is_char_boundary(boundary) && !ctx.visited.contains(&(id, boundary)) {
                        let parameter = &path[offset..boundary];
                        ctx.parameters.push((&child.state.name, parameter));

                        if let Some(result) = child.search_at(ctx, path, boundary) {
                            return Some(result);
                        }

                        ctx.parameters.pop();
                        ctx.visited.push((id, boundary));
                    }

                    cursor = position;
                }
            }
        }

        if limit > 0 {
            let end = offset + limit;

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

                let parameter = &path[offset..end];
                ctx.parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(ctx, path, end) {
                    return Some(result);
                }

                ctx.parameters.pop();
            }
        }

        None
    }

    /// Matches segment wildcard parameters like `/<*path>/help`.
    fn search_wildcard_segment<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];

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

            let id = ptr::from_ref(child) as usize;
            let max = remaining.len() - child.shortest;

            let positions = core::iter::successors(Some(max), |&position| {
                memchr::memrchr(b'/', &remaining[..position])
            });

            for position in positions.take_while(|&position| position > 0) {
                let boundary = offset + position;
                if ctx.visited.contains(&(id, boundary)) {
                    continue;
                }

                let after = &remaining[position..];
                if !child.state.suffixes.iter().any(|finder| {
                    let needle = finder.needle();
                    after.len() >= needle.len() && needle.iter().zip(after).all(|(a, b)| a == b)
                }) {
                    continue;
                }

                let parameter = &path[offset..boundary];
                ctx.parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(ctx, path, boundary) {
                    return Some(result);
                }

                ctx.parameters.pop();
                ctx.visited.push((id, boundary));
            }
        }

        None
    }

    /// Matches inline wildcard parameters like `/<*path>.html`.
    fn search_wildcard_inline<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];

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

            let id = ptr::from_ref(child) as usize;
            let max = remaining.len() - child.shortest;

            for suffix in &child.state.suffixes {
                let mut cursor = (max + suffix.needle().len()).min(remaining.len());

                while let Some(position) = suffix.rfind(&remaining[..cursor]) {
                    if position == 0 {
                        break;
                    }

                    let boundary = offset + position;
                    if path.is_char_boundary(boundary) && !ctx.visited.contains(&(id, boundary)) {
                        let parameter = &path[offset..boundary];
                        ctx.parameters.push((&child.state.name, parameter));

                        if let Some(result) = child.search_at(ctx, path, boundary) {
                            return Some(result);
                        }

                        ctx.parameters.pop();
                        ctx.visited.push((id, boundary));
                    }

                    cursor = position;
                }
            }
        }

        None
    }

    /// Matches end wildcard parameters like `/<*path>`.
    fn search_end_wildcard<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r NodeData> {
        if let Some(child) = &self.end_wildcard {
            ctx.parameters.push((&child.name, &path[offset..]));
            return Some(&child.data);
        }

        None
    }
}
