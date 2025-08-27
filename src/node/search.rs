use smallvec::{SmallVec, smallvec};

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
        path: &'p [u8],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        if path.is_empty() {
            return self.data.as_ref();
        }

        if let Some(search) = self.search_static(path, parameters) {
            return Some(search);
        }

        if let Some(search) = {
            if self.dynamic_segment_only {
                self.search_dynamic_segment(path, parameters)
            } else {
                self.search_dynamic_inline(path, parameters)
            }
        } {
            return Some(search);
        }

        if let Some(search) = {
            if self.wildcard_segment_only {
                self.search_wildcard_segment(path, parameters)
            } else {
                self.search_wildcard_inline(path, parameters)
            }
        } {
            return Some(search);
        }

        if let Some(search) = self.search_end_wildcard(path, parameters) {
            return Some(search);
        }

        None
    }

    /// Simple byte comparison between the path and stored static prefixes.
    fn search_static<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        for child in &self.static_children {
            // This was previously a "starts_with" call, but turns out this is much faster.
            if path.len() >= child.state.prefix.len()
                && child.state.prefix.iter().zip(path).all(|(a, b)| a == b)
            {
                let remaining_path = &path[child.state.prefix.len()..];
                if let Some(data) = child.search(remaining_path, parameters) {
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
        path: &'p [u8],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        let segment_end = path.iter().position(|&b| b == b'/').unwrap_or(path.len());
        let segment = &path[..segment_end];
        let path = &path[segment_end..];

        for child in &self.dynamic_children {
            parameters.push((&child.state.name, core::str::from_utf8(segment).ok()?));

            if let Some(result) = child.search(path, parameters) {
                return Some(result);
            }

            parameters.pop();
        }

        None
    }

    /// Slower dynamic path for complex templates like `/<name>.<extension>`.
    /// Must try each byte to consider all possible permutations.
    /// Prefers the most specific match.
    fn search_dynamic_inline<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        for child in &self.dynamic_children {
            let mut consumed = 0;

            let mut best_match: Option<&'r NodeData> = None;
            let mut best_match_parameters = smallvec![];

            while consumed < path.len() {
                if path[consumed] == b'/' {
                    break;
                }

                consumed += 1;

                let segment = &path[..consumed];

                let mut current_parameters = parameters.clone();
                current_parameters.push((&child.state.name, core::str::from_utf8(segment).ok()?));

                let Some(data) = child.search(&path[consumed..], &mut current_parameters) else {
                    continue;
                };

                if best_match.is_none_or(|best| data.specificity >= best.specificity) {
                    best_match = Some(data);
                    best_match_parameters = current_parameters;
                }
            }

            if let Some(result) = best_match {
                *parameters = best_match_parameters;
                return Some(result);
            }
        }

        None
    }

    /// Fast wildcard path for segmented templates like `/<*path>`.
    /// This lets us search segment by segment, rather than byte by byte.
    fn search_wildcard_segment<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        for child in &self.wildcard_children {
            let mut consumed = 0;
            let mut remaining_path = path;
            let mut section_end = false;

            while !remaining_path.is_empty() {
                if section_end {
                    consumed += 1;
                }

                let segment_end = remaining_path
                    .iter()
                    .position(|&b| b == b'/')
                    .unwrap_or(remaining_path.len());

                if segment_end == 0 {
                    consumed += 1;
                    section_end = false;
                } else {
                    consumed += segment_end;
                    section_end = true;
                }

                let segment = if path[..consumed].ends_with(b"/") {
                    &path[..consumed - 1]
                } else {
                    &path[..consumed]
                };

                parameters.push((&child.state.name, core::str::from_utf8(segment).ok()?));

                if let Some(result) = child.search(&remaining_path[segment_end..], parameters) {
                    return Some(result);
                }

                parameters.pop();

                if segment_end == remaining_path.len() {
                    break;
                }

                remaining_path = &remaining_path[segment_end + 1..];
            }
        }

        None
    }

    /// Slower wildcard path for complex templates like `/<*name>.txt`.
    /// Must try each byte, since the wildcard ends mid-segment.
    /// Prefers the most specific match.
    fn search_wildcard_inline<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        for child in &self.wildcard_children {
            let mut consumed = 0;

            let mut best_match: Option<&'r NodeData> = None;
            let mut best_match_parameters = smallvec![];

            while consumed < path.len() {
                consumed += 1;

                let segment = &path[..consumed];

                let mut current_parameters = parameters.clone();
                current_parameters.push((&child.state.name, core::str::from_utf8(segment).ok()?));

                let Some(data) = child.search(&path[consumed..], &mut current_parameters) else {
                    continue;
                };

                if best_match.is_none_or(|best| data.specificity >= best.specificity) {
                    best_match = Some(data);
                    best_match_parameters = current_parameters;
                }
            }

            if let Some(result) = best_match {
                *parameters = best_match_parameters;
                return Some(result);
            }
        }

        None
    }

    /// If there's anything else, it matches.
    fn search_end_wildcard<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut SmallVec<[(&'r str, &'p str); 4]>,
    ) -> Option<&'r NodeData> {
        if let Some(child) = &self.end_wildcard {
            parameters.push((&child.state.name, core::str::from_utf8(path).ok()?));
            return child.data.as_ref();
        }

        None
    }
}
