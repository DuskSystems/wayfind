use smallvec::SmallVec;

use crate::node::{Node, NodeData};

impl<S> Node<S> {
    /// Searches for a matching template in the node tree.
    ///
    /// This method traverses the tree to find a route node that matches the given path, collecting parameters along the way.
    ///
    /// We try nodes in the order:
    /// - statics
    /// - dynamics
    /// - wildcards
    /// - end wildcards
    pub fn search<'r, 'p>(
        &'r self,
        path: &'p str,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        self.search_at(path, 0, parameters)
    }

    fn search_at<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        if offset == path.len() {
            return self.data.as_ref();
        }

        if let Some(search) = self.search_static(path, offset, parameters) {
            return Some(search);
        }

        if let Some(search) = {
            if self.dynamic_segment_only {
                self.search_dynamic_segment(path, offset, parameters)
            } else {
                self.search_dynamic_inline(path, offset, parameters)
            }
        } {
            return Some(search);
        }

        if let Some(search) = {
            if self.wildcard_segment_only {
                self.search_wildcard_segment(path, offset, parameters)
            } else {
                self.search_wildcard_inline(path, offset, parameters)
            }
        } {
            return Some(search);
        }

        self.search_end_wildcard(path, offset, parameters)
    }

    /// Simple byte comparison between the path and stored static prefixes.
    fn search_static<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        let path_bytes = path.as_bytes();
        let remaining = &path_bytes[offset..];
        let first = *remaining.first()?;

        for child in &self.static_children {
            if child.state.first != first {
                continue;
            }

            // This was previously a "starts_with" call, but turns out this is much faster.
            if remaining.len() >= child.state.prefix.len()
                && child
                    .state
                    .prefix
                    .iter()
                    .zip(remaining)
                    .all(|(a, b)| a == b)
            {
                let current = offset + child.state.prefix.len();
                if let Some(data) = child.search_at(path, current, parameters) {
                    return Some(data);
                }
            }
        }

        None
    }

    /// Fast dynamic path for segmented templates like `/<segment>`.
    /// This lets use skip ahead to the next `/` (or end of path), rather than walk byte by byte.
    fn search_dynamic_segment<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        if self.dynamic_children.is_empty() {
            return None;
        }

        let remaining = &path.as_bytes()[offset..];
        let segment_end = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());
        let segment = &path[offset..offset + segment_end];

        for child in &self.dynamic_children {
            parameters.push((&child.state.name, segment));

            if let Some(result) = child.search_at(path, offset + segment_end, parameters) {
                return Some(result);
            }

            parameters.pop();
        }

        None
    }

    /// Greedy dynamic path for complex templates like `/<name>.txt`.
    /// Finds the last occurrence of the separator and splits there.
    fn search_dynamic_inline<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];
        let segment_end = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());
        let segment = &remaining[..segment_end];

        for child in &self.dynamic_children {
            for suffix in &child.static_suffixes {
                let Some(position) = memchr::memmem::rfind(segment, suffix) else {
                    continue;
                };

                if position == 0 {
                    continue;
                }

                let parameter = &path[offset..offset + position];
                parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(path, offset + position, parameters) {
                    return Some(result);
                }

                parameters.pop();
            }

            if !segment.is_empty() {
                let parameter = &path[offset..offset + segment_end];
                parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(path, offset + segment_end, parameters) {
                    return Some(result);
                }

                parameters.pop();
            }
        }

        None
    }

    /// Fast wildcard path for segmented templates like `/<*path>`.
    /// This lets us search segment by segment, rather than byte by byte.
    fn search_wildcard_segment<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        let path_bytes = path.as_bytes();

        for child in &self.wildcard_children {
            let mut consumed = 0;
            let mut current = offset;
            let mut section_end = false;

            while current < path_bytes.len() {
                let remaining = &path_bytes[current..];

                if section_end {
                    consumed += 1;
                }

                let segment_end = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());

                if segment_end == 0 {
                    consumed += 1;
                    section_end = false;
                } else {
                    consumed += segment_end;
                    section_end = true;
                }

                let segment = if path_bytes[offset..offset + consumed].ends_with(b"/") {
                    &path[offset..offset + consumed - 1]
                } else {
                    &path[offset..offset + consumed]
                };

                parameters.push((&child.state.name, segment));

                if let Some(result) = child.search_at(path, current + segment_end, parameters) {
                    return Some(result);
                }

                parameters.pop();

                if segment_end == remaining.len() {
                    break;
                }

                current += segment_end + 1;
            }
        }

        None
    }

    /// Greedy wildcard path for complex templates like `/<*name>.txt`.
    /// Finds the last occurrence of the separator and splits there.
    fn search_wildcard_inline<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            for suffix in &child.static_suffixes {
                let Some(position) = memchr::memmem::rfind(remaining, suffix) else {
                    continue;
                };

                if position == 0 {
                    continue;
                }

                let parameter = &path[offset..offset + position];
                parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(path, offset + position, parameters) {
                    return Some(result);
                }

                parameters.pop();
            }

            if !remaining.is_empty() {
                let parameter = &path[offset..];
                parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(path, path.len(), parameters) {
                    return Some(result);
                }

                parameters.pop();
            }
        }

        None
    }

    /// If there's anything else, it matches.
    fn search_end_wildcard<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        if let Some(child) = &self.end_wildcard {
            parameters.push((&child.state.name, &path[offset..]));
            return child.data.as_ref();
        }

        None
    }
}
