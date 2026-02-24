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

        if path.len().checked_sub(offset)? < self.shortest {
            return None;
        }

        if let Some(result) = self.search_static(path, offset, parameters) {
            return Some(result);
        }

        if let Some(result) = if self.dynamic_segment_only {
            self.search_dynamic_segment(path, offset, parameters)
        } else {
            self.search_dynamic_inline(path, offset, parameters)
        } {
            return Some(result);
        }

        if let Some(result) = if self.wildcard_segment_only {
            self.search_wildcard_segment(path, offset, parameters)
        } else {
            self.search_wildcard_inline(path, offset, parameters)
        } {
            return Some(result);
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
        let remaining = path.as_bytes().get(offset..)?;
        let first = *remaining.first()?;

        let child = self
            .static_children
            .iter()
            .find(|child| child.state.first == first)?;

        if remaining.starts_with(&child.state.prefix) {
            let end = offset.checked_add(child.state.prefix.len())?;
            return child.search_at(path, end, parameters);
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

        let remaining = path.as_bytes().get(offset..)?;

        let limit = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());
        if limit == 0 {
            return None;
        }

        let end = offset.checked_add(limit)?;
        let segment = path.get(offset..end)?;

        for child in &self.dynamic_children {
            if remaining.len().saturating_sub(limit) < child.shortest {
                continue;
            }

            if !child.tails.is_empty() && !child.tails.iter().any(|tail| remaining.ends_with(tail))
            {
                continue;
            }

            parameters.push((&child.state.name, segment));

            if let Some(result) = child.search_at(path, end, parameters) {
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
        let remaining = path.as_bytes().get(offset..)?;
        let limit = memchr::memchr(b'/', remaining).unwrap_or(remaining.len());

        for child in &self.dynamic_children {
            if !child.tails.is_empty() && !child.tails.iter().any(|tail| remaining.ends_with(tail))
            {
                continue;
            }

            let max = remaining.len().saturating_sub(child.shortest);
            if max == 0 {
                continue;
            }

            for suffix in &child.state.suffixes {
                let mut end = limit
                    .min(max)
                    .saturating_add(suffix.needle().len())
                    .min(remaining.len());

                while let Some(position) = suffix.rfind(remaining.get(..end)?) {
                    if position == 0 {
                        break;
                    }

                    let path_end = offset.checked_add(position)?;
                    if path.is_char_boundary(path_end) {
                        let parameter = path.get(offset..path_end)?;
                        parameters.push((&child.state.name, parameter));

                        if let Some(result) = child.search_at(path, path_end, parameters) {
                            return Some(result);
                        }

                        parameters.pop();
                    }

                    end = position;
                }
            }
        }

        if limit != 0 {
            let segment_end = offset.checked_add(limit)?;
            let segment = path.get(offset..segment_end)?;

            for child in &self.dynamic_children {
                if remaining.len().saturating_sub(limit) < child.shortest {
                    continue;
                }

                if !child.tails.is_empty()
                    && !child.tails.iter().any(|tail| remaining.ends_with(tail))
                {
                    continue;
                }

                parameters.push((&child.state.name, segment));

                if let Some(result) = child.search_at(path, segment_end, parameters) {
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
        let remaining = path.as_bytes().get(offset..)?;

        for child in &self.wildcard_children {
            if !child.tails.is_empty() && !child.tails.iter().any(|tail| remaining.ends_with(tail))
            {
                continue;
            }

            let max = remaining.len().saturating_sub(child.shortest);

            let positions = core::iter::successors(Some(max), |&position| {
                memchr::memrchr(b'/', remaining.get(..position)?)
            });

            for position in positions.take_while(|&position| position > 0) {
                let after = remaining.get(position..)?;
                if !child
                    .state
                    .suffixes
                    .iter()
                    .any(|finder| after.starts_with(finder.needle()))
                {
                    continue;
                }

                let end = offset.checked_add(position)?;

                let parameter = path.get(offset..end)?;
                parameters.push((&child.state.name, parameter));

                if let Some(result) = child.search_at(path, end, parameters) {
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
        let remaining = path.as_bytes().get(offset..)?;

        for child in &self.wildcard_children {
            if !child.tails.is_empty() && !child.tails.iter().any(|tail| remaining.ends_with(tail))
            {
                continue;
            }

            let max = remaining.len().saturating_sub(child.shortest);
            if max == 0 {
                continue;
            }

            for suffix in &child.state.suffixes {
                let mut end = max
                    .saturating_add(suffix.needle().len())
                    .min(remaining.len());

                while let Some(position) = suffix.rfind(remaining.get(..end)?) {
                    if position == 0 {
                        break;
                    }

                    let path_end = offset.checked_add(position)?;
                    if path.is_char_boundary(path_end) {
                        let parameter = path.get(offset..path_end)?;
                        parameters.push((&child.state.name, parameter));

                        if let Some(result) = child.search_at(path, path_end, parameters) {
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
            parameters.push((&child.name, path.get(offset..)?));
            return Some(&child.data);
        }

        None
    }
}
