use core::cmp::Ordering;

use hashbrown::HashMap;
use smallvec::smallvec;

use crate::{
    node::{Node, NodeData},
    router::{Parameters, StoredConstraint},
    state::NodeState,
};

impl<S: NodeState> Node<S> {
    /// Searches for a matching template in the node tree.
    ///
    /// This method traverses the tree to find a route node that matches the given path, collecting parameters along the way.
    ///
    /// We try nodes in the order:
    /// - statics
    /// - dynamics constrained
    /// - dynamics
    /// - wildcards constrained
    /// - wildcards
    /// - end wildcards constrained
    /// - end wildcards
    pub fn search<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<&'static str, StoredConstraint>,
    ) -> Option<&'r NodeData> {
        if path.is_empty() {
            return self.data.as_ref();
        }

        if let Some(search) = self.search_static(path, parameters, constraints) {
            return Some(search);
        }

        if let Some(search) = {
            if self.dynamic_children_shortcut {
                self.search_dynamic_constrained_segment(path, parameters, constraints)
            } else {
                self.search_dynamic_constrained_inline(path, parameters, constraints)
            }
        } {
            return Some(search);
        }

        if let Some(search) = {
            if self.dynamic_children_shortcut {
                self.search_dynamic_segment(path, parameters, constraints)
            } else {
                self.search_dynamic_inline(path, parameters, constraints)
            }
        } {
            return Some(search);
        }

        if let Some(search) = {
            if self.wildcard_children_shortcut {
                self.search_wildcard_constrained_segment(path, parameters, constraints)
            } else {
                self.search_wildcard_constrained_inline(path, parameters, constraints)
            }
        } {
            return Some(search);
        }

        if let Some(search) = {
            if self.wildcard_children_shortcut {
                self.search_wildcard_segment(path, parameters, constraints)
            } else {
                self.search_wildcard_inline(path, parameters, constraints)
            }
        } {
            return Some(search);
        }

        if let Some(search) = self.search_end_wildcard_constrained(path, parameters, constraints) {
            return Some(search);
        }

        if let Some(search) = self.search_end_wildcard(path, parameters) {
            return Some(search);
        }

        None
    }

    fn search_static<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<&'static str, StoredConstraint>,
    ) -> Option<&'r NodeData> {
        for child in &self.static_children {
            if path.len() >= child.state.prefix.len()
                && child.state.prefix.iter().zip(path).all(|(a, b)| a == b)
            {
                let remaining_path = &path[child.state.prefix.len()..];
                if let Some(data) = child.search(remaining_path, parameters, constraints) {
                    return Some(data);
                }
            }
        }

        None
    }

    /// Can only handle simple dynamic templates like `/{segment}/`.
    fn search_dynamic_constrained_segment<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<&'static str, StoredConstraint>,
    ) -> Option<&'r NodeData> {
        for child in &self.dynamic_constrained_children {
            let segment_end = path.iter().position(|&b| b == b'/').unwrap_or(path.len());

            let segment = &path[..segment_end];
            if !Self::check_constraint(Some(&child.state.constraint), segment, constraints) {
                continue;
            }

            parameters.push((&child.state.name, core::str::from_utf8(segment).ok()?));

            if let Some(result) = child.search(&path[segment_end..], parameters, constraints) {
                return Some(result);
            }

            parameters.pop();
        }

        None
    }

    /// Can handle complex dynamic templates like `{name}.{extension}`.
    fn search_dynamic_constrained_inline<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<&'static str, StoredConstraint>,
    ) -> Option<&'r NodeData> {
        for child in &self.dynamic_constrained_children {
            let mut consumed = 0;

            let mut best_match: Option<&'r NodeData> = None;
            let mut best_match_parameters = smallvec![];

            while consumed < path.len() {
                if path[consumed] == b'/' {
                    break;
                }

                consumed += 1;

                let segment = &path[..consumed];
                if !Self::check_constraint(Some(&child.state.constraint), segment, constraints) {
                    continue;
                }

                let mut current_parameters = parameters.clone();
                current_parameters.push((&child.state.name, core::str::from_utf8(segment).ok()?));

                let Some(data) =
                    child.search(&path[consumed..], &mut current_parameters, constraints)
                else {
                    continue;
                };

                if best_match.is_none_or(|best| match data.depth.cmp(&best.depth) {
                    Ordering::Greater => true,
                    Ordering::Equal => data.length >= best.length,
                    Ordering::Less => false,
                }) {
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

    /// Can only handle simple dynamic templates like `/{segment}/`.
    fn search_dynamic_segment<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<&'static str, StoredConstraint>,
    ) -> Option<&'r NodeData> {
        for child in &self.dynamic_children {
            let segment_end = path.iter().position(|&b| b == b'/').unwrap_or(path.len());

            let segment = &path[..segment_end];

            parameters.push((&child.state.name, core::str::from_utf8(segment).ok()?));

            if let Some(result) = child.search(&path[segment_end..], parameters, constraints) {
                return Some(result);
            }

            parameters.pop();
        }

        None
    }

    /// Can handle complex dynamic templates like `{name}.{extension}`.
    fn search_dynamic_inline<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<&'static str, StoredConstraint>,
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

                let Some(data) =
                    child.search(&path[consumed..], &mut current_parameters, constraints)
                else {
                    continue;
                };

                if best_match.is_none_or(|best| match data.depth.cmp(&best.depth) {
                    Ordering::Greater => true,
                    Ordering::Equal => data.length >= best.length,
                    Ordering::Less => false,
                }) {
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

    /// Can only handle simple wildcard templates like `/{*segment}/`.
    fn search_wildcard_constrained_segment<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<&'static str, StoredConstraint>,
    ) -> Option<&'r NodeData> {
        for child in &self.wildcard_constrained_children {
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

                if !Self::check_constraint(Some(&child.state.constraint), segment, constraints) {
                    break;
                }

                parameters.push((&child.state.name, core::str::from_utf8(segment).ok()?));

                if let Some(result) =
                    child.search(&remaining_path[segment_end..], parameters, constraints)
                {
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

    /// Can handle complex wildcard templates like `/{*name}.{extension}`.
    fn search_wildcard_constrained_inline<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<&'static str, StoredConstraint>,
    ) -> Option<&'r NodeData> {
        for child in &self.wildcard_constrained_children {
            let mut consumed = 0;

            let mut best_match: Option<&'r NodeData> = None;
            let mut best_match_parameters = smallvec![];

            while consumed < path.len() {
                consumed += 1;

                let segment = &path[..consumed];
                if !Self::check_constraint(Some(&child.state.constraint), segment, constraints) {
                    continue;
                }

                let mut current_parameters = parameters.clone();
                current_parameters.push((&child.state.name, core::str::from_utf8(segment).ok()?));

                let Some(data) =
                    child.search(&path[consumed..], &mut current_parameters, constraints)
                else {
                    continue;
                };

                if best_match.is_none_or(|best| match data.depth.cmp(&best.depth) {
                    Ordering::Greater => true,
                    Ordering::Equal => data.length >= best.length,
                    Ordering::Less => false,
                }) {
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

    /// Can only handle simple wildcard templates like `/{*segment}/`.
    fn search_wildcard_segment<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<&'static str, StoredConstraint>,
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

                if let Some(result) =
                    child.search(&remaining_path[segment_end..], parameters, constraints)
                {
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

    /// Can handle complex wildcard templates like `/{*name}.{extension}`.
    fn search_wildcard_inline<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<&'static str, StoredConstraint>,
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

                let Some(data) =
                    child.search(&path[consumed..], &mut current_parameters, constraints)
                else {
                    continue;
                };

                if best_match.is_none_or(|best| match data.depth.cmp(&best.depth) {
                    Ordering::Greater => true,
                    Ordering::Equal => data.length >= best.length,
                    Ordering::Less => false,
                }) {
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

    fn search_end_wildcard_constrained<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<&'static str, StoredConstraint>,
    ) -> Option<&'r NodeData> {
        for child in &self.end_wildcard_constrained_children {
            if !Self::check_constraint(Some(&child.state.constraint), path, constraints) {
                continue;
            }

            parameters.push((&child.state.name, core::str::from_utf8(path).ok()?));
            return child.data.as_ref();
        }

        None
    }

    fn search_end_wildcard<'r, 'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
    ) -> Option<&'r NodeData> {
        if let Some(child) = self.end_wildcard_children.iter().next() {
            parameters.push((&child.state.name, core::str::from_utf8(path).ok()?));
            return child.data.as_ref();
        }

        None
    }

    fn check_constraint(
        constraint: Option<&str>,
        segment: &[u8],
        constraints: &HashMap<&str, StoredConstraint>,
    ) -> bool {
        let Some(constraint) = constraint else {
            return true;
        };

        let constraint = constraints.get(constraint).unwrap();
        let Ok(segment) = core::str::from_utf8(segment) else {
            return false;
        };

        (constraint.check)(segment)
    }
}
