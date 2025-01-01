use super::{node::Node, state::State};
use crate::node::Config;
use smallvec::{smallvec, SmallVec};
use std::{collections::HashMap, sync::Arc};

pub type Parameters<'r, 'p> = SmallVec<[(&'r str, &'p [u8]); 4]>;

#[derive(Clone)]
pub struct StoredConstraint {
    pub type_name: &'static str,
    pub check: fn(&str) -> bool,
}

impl<'r, C: Config, S: State> Node<C, S> {
    /// Searches for a matching route in the node tree.
    ///
    /// This method traverses the tree to find a route node that matches the given path, collecting parameters along the way.
    /// We try nodes in the order: static, dynamic, wildcard, then end wildcard.
    pub fn search<'p>(
        &'r self,
        key: Option<usize>,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<Arc<str>, StoredConstraint>,
    ) -> Option<(&'r C::Data, usize)>
    where
        'p: 'r,
    {
        if path.is_empty() {
            return self.data.get(&key).map(|data| (data, self.priority));
        }

        if let Some(search) = self.search_static(key, path, parameters, constraints) {
            return Some(search);
        }

        if let Some(search) = self.search_dynamic(key, path, parameters, constraints) {
            return Some(search);
        }

        if let Some(search) = self.search_wildcard(key, path, parameters, constraints) {
            return Some(search);
        }

        if let Some(search) = self.search_end_wildcard(key, path, parameters, constraints) {
            return Some(search);
        }

        None
    }

    fn search_static<'p>(
        &'r self,
        key: Option<usize>,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<Arc<str>, StoredConstraint>,
    ) -> Option<(&'r C::Data, usize)>
    where
        'p: 'r,
    {
        for child in self.static_children.iter() {
            if path.len() >= child.state.prefix.len()
                && child.state.prefix.iter().zip(path).all(|(a, b)| a == b)
            {
                let remaining_path = &path[child.state.prefix.len()..];
                if let Some((data, priority)) =
                    child.search(key, remaining_path, parameters, constraints)
                {
                    return Some((data, priority));
                }
            }
        }

        None
    }

    fn search_dynamic<'p>(
        &'r self,
        key: Option<usize>,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<Arc<str>, StoredConstraint>,
    ) -> Option<(&'r C::Data, usize)>
    where
        'p: 'r,
    {
        if self.dynamic_children_shortcut {
            self.search_dynamic_segment(key, path, parameters, constraints)
        } else {
            self.search_dynamic_inline(key, path, parameters, constraints)
        }
    }

    /// Can handle complex dynamic routes like `{name}.{extension}`.
    fn search_dynamic_inline<'p>(
        &'r self,
        key: Option<usize>,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<Arc<str>, StoredConstraint>,
    ) -> Option<(&'r C::Data, usize)>
    where
        'p: 'r,
    {
        for child in self.dynamic_children.iter() {
            let mut consumed = 0;

            let mut best_match: Option<(&'r C::Data, usize)> = None;
            let mut best_match_parameters = smallvec![];

            while consumed < path.len() {
                if path[consumed] == C::DELIMITER {
                    break;
                }

                consumed += 1;

                let segment = &path[..consumed];
                if !Self::check_constraint(child.state.constraint.as_ref(), segment, constraints) {
                    continue;
                }

                let mut current_parameters = parameters.clone();
                current_parameters.push((&child.state.name, segment));

                let Some((data, priority)) =
                    child.search(key, &path[consumed..], &mut current_parameters, constraints)
                else {
                    continue;
                };

                if best_match.is_none_or(|(_, best_priority)| priority >= best_priority) {
                    best_match = Some((data, priority));
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

    /// Can only handle simple dynamic routes like `/{segment}/`.
    fn search_dynamic_segment<'p>(
        &'r self,
        key: Option<usize>,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<Arc<str>, StoredConstraint>,
    ) -> Option<(&'r C::Data, usize)>
    where
        'p: 'r,
    {
        for child in self.dynamic_children.iter() {
            let segment_end = path
                .iter()
                .position(|&b| b == C::DELIMITER)
                .unwrap_or(path.len());

            let segment = &path[..segment_end];
            if !Self::check_constraint(child.state.constraint.as_ref(), segment, constraints) {
                continue;
            }

            parameters.push((&child.state.name, segment));

            if let Some(result) = child.search(key, &path[segment_end..], parameters, constraints) {
                return Some(result);
            }

            parameters.pop();
        }

        None
    }

    fn search_wildcard<'p>(
        &'r self,
        key: Option<usize>,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<Arc<str>, StoredConstraint>,
    ) -> Option<(&'r C::Data, usize)>
    where
        'p: 'r,
    {
        if self.wildcard_children_shortcut {
            self.search_wildcard_segment(key, path, parameters, constraints)
        } else {
            self.search_wildcard_inline(key, path, parameters, constraints)
        }
    }

    /// Can handle complex wildcard routes like `/{*name}.{extension}`.
    fn search_wildcard_inline<'p>(
        &'r self,
        key: Option<usize>,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<Arc<str>, StoredConstraint>,
    ) -> Option<(&'r C::Data, usize)>
    where
        'p: 'r,
    {
        for child in self.wildcard_children.iter() {
            let mut consumed = 0;

            let mut best_match: Option<(&'r C::Data, usize)> = None;
            let mut best_match_parameters = smallvec![];

            while consumed < path.len() {
                consumed += 1;

                let segment = &path[..consumed];
                if !Self::check_constraint(child.state.constraint.as_ref(), segment, constraints) {
                    continue;
                }

                let mut current_parameters = parameters.clone();
                current_parameters.push((&child.state.name, segment));

                let Some((data, priority)) =
                    child.search(key, &path[consumed..], &mut current_parameters, constraints)
                else {
                    continue;
                };

                if best_match.is_none_or(|(_, best_priority)| priority >= best_priority) {
                    best_match = Some((data, priority));
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

    /// Can only handle simple wildcard routes like `/{*segment}/`.
    fn search_wildcard_segment<'p>(
        &'r self,
        key: Option<usize>,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<Arc<str>, StoredConstraint>,
    ) -> Option<(&'r C::Data, usize)>
    where
        'p: 'r,
    {
        for child in self.wildcard_children.iter() {
            let mut consumed = 0;
            let mut remaining_path = path;
            let mut section_end = false;

            while !remaining_path.is_empty() {
                if section_end {
                    consumed += 1;
                }

                let segment_end = remaining_path
                    .iter()
                    .position(|&b| b == C::DELIMITER)
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

                if !Self::check_constraint(child.state.constraint.as_ref(), segment, constraints) {
                    break;
                }

                parameters.push((&child.state.name, segment));

                if let Some(result) =
                    child.search(key, &remaining_path[segment_end..], parameters, constraints)
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

    fn search_end_wildcard<'p>(
        &'r self,
        key: Option<usize>,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &HashMap<Arc<str>, StoredConstraint>,
    ) -> Option<(&'r C::Data, usize)>
    where
        'p: 'r,
    {
        for child in self.end_wildcard_children.iter() {
            if !Self::check_constraint(child.state.constraint.as_ref(), path, constraints) {
                continue;
            }

            parameters.push((&child.state.name, path));
            return child.data.get(&key).map(|data| (data, self.priority));
        }

        None
    }

    fn check_constraint(
        constraint: Option<&String>,
        segment: &[u8],
        constraints: &HashMap<Arc<str>, StoredConstraint>,
    ) -> bool {
        let Some(constraint) = constraint else {
            return true;
        };

        let constraint = constraints.get(constraint.as_str()).unwrap();
        let Ok(segment) = std::str::from_utf8(segment) else {
            return false;
        };

        (constraint.check)(segment)
    }
}
