use smallvec::SmallVec;

use crate::node::{Node, NodeData};

impl<S> Node<S> {
    /// Searches for a matching template in the node tree.
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

    /// Matches static children by prefix byte comparison.
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

    /// Matches segment dynamic parameters like `/<name>/`.
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
        if segment_end == 0 {
            return None;
        }

        let segment = &path[offset..offset + segment_end];

        for child in &self.dynamic_children {
            if remaining.len() - segment_end < child.shortest {
                continue;
            }

            parameters.push((&child.state.name, segment));

            if let Some(result) = child.search_at(path, offset + segment_end, parameters) {
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
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];
        let segment_end = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());

        for child in &self.dynamic_children {
            let max_position = remaining.len().saturating_sub(child.shortest);
            if max_position == 0 {
                continue;
            }

            for suffix in &child.state.suffixes {
                let mut search_end =
                    (segment_end.min(max_position) + suffix.len()).min(remaining.len());
                while let Some(position) = memchr::memmem::rfind(&remaining[..search_end], suffix) {
                    if position == 0 {
                        break;
                    }

                    if path.is_char_boundary(offset + position) {
                        let parameter = &path[offset..offset + position];
                        parameters.push((&child.state.name, parameter));

                        if let Some(result) = child.search_at(path, offset + position, parameters) {
                            return Some(result);
                        }

                        parameters.pop();
                    }

                    search_end = position;
                }
            }
        }

        if segment_end > 0 {
            for child in &self.dynamic_children {
                if remaining.len() - segment_end < child.shortest {
                    continue;
                }

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

    /// Matches segment wildcard parameters like `/<*path>/help`.
    fn search_wildcard_segment<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            if !child.tail.is_empty() && !remaining.ends_with(&child.tail) {
                continue;
            }

            let max_position = remaining.len().saturating_sub(child.shortest);
            let mut end = Some(max_position);
            while let Some(position) = end {
                if position == 0 {
                    break;
                }

                let after = &remaining[position..];
                if child
                    .state
                    .suffixes
                    .iter()
                    .any(|suffix| after.starts_with(suffix))
                {
                    let parameter = &path[offset..offset + position];
                    parameters.push((&child.state.name, parameter));

                    if let Some(result) = child.search_at(path, offset + position, parameters) {
                        return Some(result);
                    }

                    parameters.pop();
                }

                end = memchr::memrchr(b'/', &remaining[..position]);
            }
        }

        None
    }

    /// Matches inline wildcard parameters like `/<*path>.html`.
    #[inline(never)]
    fn search_wildcard_inline<'r, 'p>(
        &'r self,
        path: &'p str,
        offset: usize,
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        let path_bytes = path.as_bytes();
        let remaining = &path_bytes[offset..];

        for child in &self.wildcard_children {
            if !child.tail.is_empty() && !remaining.ends_with(&child.tail) {
                continue;
            }

            let max_position = remaining.len().saturating_sub(child.shortest);
            if max_position == 0 {
                continue;
            }

            for suffix in &child.state.suffixes {
                let mut search_end = (max_position + suffix.len()).min(remaining.len());
                while let Some(position) = memchr::memmem::rfind(&remaining[..search_end], suffix) {
                    if position == 0 {
                        break;
                    }

                    if path.is_char_boundary(offset + position) {
                        let parameter = &path[offset..offset + position];
                        parameters.push((&child.state.name, parameter));

                        if let Some(result) = child.search_at(path, offset + position, parameters) {
                            return Some(result);
                        }

                        parameters.pop();
                    }

                    search_end = position;
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
