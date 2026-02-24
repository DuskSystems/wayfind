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

        if let Some(search) = self.search_static(path, offset, parameters) {
            return Some(search);
        }

        if let Some(search) = if self.dynamic_segment_only {
            self.search_dynamic_segment(path, offset, parameters)
        } else {
            self.search_dynamic_inline(path, offset, parameters)
        } {
            return Some(search);
        }

        if let Some(search) = if self.wildcard_segment_only {
            self.search_wildcard_segment(path, offset, parameters)
        } else {
            self.search_wildcard_inline(path, offset, parameters)
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
                if let Some(data) = child.search_at(path, end, parameters) {
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
        let limit = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());
        if limit == 0 {
            return None;
        }

        let segment = &path[offset..offset + limit];

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

            parameters.push((&child.state.name, segment));

            if let Some(result) = child.search_at(path, offset + limit, parameters) {
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
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
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

                        if let Some(result) = child.search_at(path, offset + position, parameters) {
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

                let parameter = &path[offset..offset + limit];
                parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(path, offset + limit, parameters) {
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

                let parameter = &path[offset..offset + position];
                parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(path, offset + position, parameters) {
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
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
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

                        if let Some(result) = child.search_at(path, offset + position, parameters) {
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
