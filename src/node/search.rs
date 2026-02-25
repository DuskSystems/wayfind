use smallvec::SmallVec;

use crate::node::{Node, NodeData};
use crate::state::StaticState;

impl<S> Node<S> {
    /// Searches for a matching template in the node tree.
    pub fn search<'r, 'p>(
        &'r self,
        path: &'p str,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        let slashes: SmallVec<[usize; 8]> = memchr::memchr_iter(b'/', path.as_bytes()).collect();
        self.search_at(path, 0, &slashes, parameters)
    }

    fn search_at<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        slashes: &[usize],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
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

        let remaining = &path.as_bytes()[offset..];
        let first = *remaining.first()?;

        if let Some(child) = self.static_children.iter().find(|child| {
            child.state.first == first
                && remaining.len() >= child.state.prefix.len()
                && remaining[..child.state.prefix.len()] == *child.state.prefix
        }) {
            let end = offset + child.state.prefix.len();

            if self.static_only {
                return child.search_static_chain(path, end, slashes, parameters);
            }

            if let Some(data) = child.search_at(path, end, slashes, parameters) {
                return Some(data);
            }
        }

        self.search_dynamic_wildcard_end(path, offset, slashes, parameters)
    }

    /// Searches dynamic, wildcard, and end wildcard children.
    fn search_dynamic_wildcard_end<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        slashes: &[usize],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        if let Some(search) = if self.dynamic_segment_only {
            self.search_dynamic_segment(path, offset, slashes, parameters)
        } else {
            self.search_dynamic_inline(path, offset, slashes, parameters)
        } {
            return Some(search);
        }

        if let Some(search) = if self.wildcard_segment_only {
            self.search_wildcard_segment(path, offset, slashes, parameters)
        } else {
            self.search_wildcard_inline(path, offset, slashes, parameters)
        } {
            return Some(search);
        }

        self.search_end_wildcard(path, offset, parameters)
    }

    /// Matches segment dynamic parameters like `/<name>/`.
    fn search_dynamic_segment<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        slashes: &[usize],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        if self.dynamic_children.is_empty() {
            return None;
        }

        let remaining = &path.as_bytes()[offset..];
        let idx = slashes.partition_point(|&slash| slash < offset);
        let limit = slashes
            .get(idx)
            .map_or(remaining.len(), |slash| slash - offset);
        if limit == 0 {
            return None;
        }

        let segment = &path[offset..offset + limit];

        for child in &self.dynamic_children {
            if remaining.len() - limit < child.shortest {
                continue;
            }

            if !child.tails.is_empty() && !child.tails.iter().any(|tail| remaining.ends_with(tail))
            {
                continue;
            }

            parameters.push((&child.state.name, segment));

            if let Some(result) = child.search_at(path, offset + limit, slashes, parameters) {
                return Some(result);
            }

            parameters.pop();
        }

        None
    }

    /// Matches inline dynamic parameters like `/<name>.txt`.
    fn search_dynamic_inline<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        slashes: &[usize],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];
        let idx = slashes.partition_point(|&slash| slash < offset);
        let limit = slashes
            .get(idx)
            .map_or(remaining.len(), |slash| slash - offset);

        for child in &self.dynamic_children {
            if remaining.len() <= child.shortest {
                continue;
            }

            if !child.tails.is_empty() && !child.tails.iter().any(|tail| remaining.ends_with(tail))
            {
                continue;
            }

            let max = remaining.len() - child.shortest;

            for suffix in &child.state.suffixes {
                let mut end = (limit.min(max) + suffix.needle().len()).min(remaining.len());

                while let Some(position) = suffix.rfind(&remaining[..end]) {
                    if position == 0 {
                        break;
                    }

                    if path.is_char_boundary(offset + position) {
                        let parameter = &path[offset..offset + position];
                        parameters.push((&child.state.name, parameter));

                        if let Some(result) =
                            child.search_at(path, offset + position, slashes, parameters)
                        {
                            return Some(result);
                        }

                        parameters.pop();
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
                    && !child.tails.iter().any(|tail| remaining.ends_with(tail))
                {
                    continue;
                }

                let parameter = &path[offset..offset + limit];
                parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(path, offset + limit, slashes, parameters) {
                    return Some(result);
                }

                parameters.pop();
            }
        }

        None
    }

    /// Matches segment wildcard parameters like `/<*path>/help`.
    fn search_wildcard_segment<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        slashes: &[usize],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];
        let idx_start = slashes.partition_point(|&slash| slash < offset);

        for child in &self.wildcard_children {
            if remaining.len() < child.shortest {
                continue;
            }

            if !child.tails.is_empty() && !child.tails.iter().any(|tail| remaining.ends_with(tail))
            {
                continue;
            }

            let max = remaining.len() - child.shortest;

            let idx_end = slashes.partition_point(|&slash| slash < offset + max);
            let slash_positions = slashes[idx_start..idx_end]
                .iter()
                .rev()
                .map(|&slash| slash - offset);

            let positions = core::iter::once(max).chain(slash_positions);

            for position in positions.take_while(|&position| position > 0) {
                let after = &remaining[position..];
                if !child
                    .state
                    .suffixes
                    .iter()
                    .any(|finder| after.starts_with(finder.needle()))
                {
                    continue;
                }

                let parameter = &path[offset..offset + position];
                parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(path, offset + position, slashes, parameters)
                {
                    return Some(result);
                }

                parameters.pop();
            }
        }

        None
    }

    /// Matches inline wildcard parameters like `/<*path>.html`.
    fn search_wildcard_inline<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        slashes: &[usize],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            if remaining.len() <= child.shortest {
                continue;
            }

            if !child.tails.is_empty() && !child.tails.iter().any(|tail| remaining.ends_with(tail))
            {
                continue;
            }

            let max = remaining.len() - child.shortest;

            for suffix in &child.state.suffixes {
                let mut end = (max + suffix.needle().len()).min(remaining.len());

                while let Some(position) = suffix.rfind(&remaining[..end]) {
                    if position == 0 {
                        break;
                    }

                    if path.is_char_boundary(offset + position) {
                        let parameter = &path[offset..offset + position];
                        parameters.push((&child.state.name, parameter));

                        if let Some(result) =
                            child.search_at(path, offset + position, slashes, parameters)
                        {
                            return Some(result);
                        }

                        parameters.pop();
                    }

                    end = position;
                }
            }
        }

        None
    }

    /// Matches end wildcard parameters like `/<*path>`.
    fn search_end_wildcard<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        if let Some(child) = &self.end_wildcard {
            parameters.push((&child.name, &path[offset..]));
            return Some(&child.data);
        }

        None
    }
}

impl Node<StaticState> {
    fn search_static_chain<'r, 'p>(
        &'r self,
        path: &'p str,
        mut offset: usize,
        slashes: &[usize],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        let mut node = self;

        loop {
            if offset >= path.len() {
                return if offset == path.len() {
                    node.data.as_ref()
                } else {
                    None
                };
            }

            if path.len() - offset < node.shortest {
                return None;
            }

            let remaining = &path.as_bytes()[offset..];
            let first = *remaining.first()?;

            let static_match = node.static_children.iter().find(|child| {
                child.state.first == first
                    && remaining.len() >= child.state.prefix.len()
                    && remaining[..child.state.prefix.len()] == *child.state.prefix
            });

            if let Some(child) = static_match {
                let end = offset + child.state.prefix.len();

                if node.static_only {
                    offset = end;
                    node = child;
                    continue;
                }

                if let Some(data) = child.search_at(path, end, slashes, parameters) {
                    return Some(data);
                }
            }

            if node.static_only {
                return None;
            }

            return node.search_dynamic_wildcard_end(path, offset, slashes, parameters);
        }
    }
}
