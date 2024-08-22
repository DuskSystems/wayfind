use super::{Node, NodeData};
use crate::router::StoredConstraint;
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Match<'router, 'path, T> {
    pub data: &'router NodeData<T>,
    pub parameters: SmallVec<[Parameter<'router, 'path>; 4]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter<'router, 'path> {
    pub key: &'router str,
    pub value: &'path str,
}

impl<T> Node<T> {
    pub fn search<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut SmallVec<[Parameter<'router, 'path>; 4]>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Option<&'router Self> {
        if path.is_empty() {
            return if self.data.is_some() {
                Some(self)
            } else {
                None
            };
        }

        if let Some(search) = self.search_static(path, parameters, constraints) {
            return Some(search);
        }

        if let Some(search) = self.search_dynamic(path, parameters, constraints) {
            return Some(search);
        }

        if let Some(search) = self.search_wildcard(path, parameters, constraints) {
            return Some(search);
        }

        if let Some(search) = self.search_end_wildcard(path, parameters, constraints) {
            return Some(search);
        }

        None
    }

    fn search_static<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut SmallVec<[Parameter<'router, 'path>; 4]>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Option<&'router Self> {
        for static_child in &self.static_children {
            // NOTE: This was previously a "starts_with" call, but turns out this is much faster.
            if path.len() >= static_child.prefix.len()
                && static_child.prefix.iter().zip(path).all(|(a, b)| a == b)
            {
                let remaining_path = &path[static_child.prefix.len()..];
                if let Some(node_data) =
                    static_child.search(remaining_path, parameters, constraints)
                {
                    return Some(node_data);
                }
            }
        }

        None
    }

    fn search_dynamic<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut SmallVec<[Parameter<'router, 'path>; 4]>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Option<&'router Self> {
        if self.quick_dynamic {
            self.search_dynamic_segment(path, parameters, constraints)
        } else {
            self.search_dynamic_inline(path, parameters, constraints)
        }
    }

    // Dynamic with support for inline dynamic sections, e.g. `{name}.{extension}`
    // NOTE: Parameters are greedy in nature:
    //   Route: `{name}.{extension}`
    //   Path: `my.long.file.txt`
    //   Name: `my.long.file`
    //   Ext: `txt`
    fn search_dynamic_inline<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut SmallVec<[Parameter<'router, 'path>; 4]>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Option<&'router Self> {
        for dynamic_child in &self.dynamic_children {
            let mut consumed = 0;

            let mut last_match = None;
            let mut last_match_parameters = smallvec![];

            while consumed < path.len() {
                if path[consumed] == b'/' {
                    break;
                }

                consumed += 1;

                let segment = &path[..consumed];
                if !Self::check_constraint(dynamic_child, segment, constraints) {
                    continue;
                }

                let mut current_parameters = parameters.clone();
                current_parameters.push(Parameter {
                    key: unsafe { std::str::from_utf8_unchecked(&dynamic_child.prefix) },
                    value: unsafe { std::str::from_utf8_unchecked(segment) },
                });

                if let Some(node_data) =
                    dynamic_child.search(&path[consumed..], &mut current_parameters, constraints)
                {
                    last_match = Some(node_data);
                    last_match_parameters = current_parameters;
                }
            }

            if let Some(node_data) = last_match {
                *parameters = last_match_parameters;
                return Some(node_data);
            }
        }

        None
    }

    // Doesn't support inline dynamic sections, e.g. `{name}.{extension}`, only `/{segment}/`
    fn search_dynamic_segment<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut SmallVec<[Parameter<'router, 'path>; 4]>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Option<&'router Self> {
        for dynamic_child in &self.dynamic_children {
            let segment_end = path.iter().position(|&b| b == b'/').unwrap_or(path.len());

            let segment = &path[..segment_end];
            if !Self::check_constraint(dynamic_child, segment, constraints) {
                continue;
            }

            parameters.push(Parameter {
                key: unsafe { std::str::from_utf8_unchecked(&dynamic_child.prefix) },
                value: unsafe { std::str::from_utf8_unchecked(segment) },
            });

            if let Some(node_data) =
                dynamic_child.search(&path[segment_end..], parameters, constraints)
            {
                return Some(node_data);
            }

            parameters.pop();
        }

        None
    }

    fn search_wildcard<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut SmallVec<[Parameter<'router, 'path>; 4]>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Option<&'router Self> {
        for wildcard_child in &self.wildcard_children {
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

                if !Self::check_constraint(wildcard_child, segment, constraints) {
                    break;
                }

                parameters.push(Parameter {
                    key: unsafe { std::str::from_utf8_unchecked(&wildcard_child.prefix) },
                    value: unsafe { std::str::from_utf8_unchecked(segment) },
                });

                if let Some(node_data) =
                    wildcard_child.search(&remaining_path[segment_end..], parameters, constraints)
                {
                    return Some(node_data);
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

    fn search_end_wildcard<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut SmallVec<[Parameter<'router, 'path>; 4]>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Option<&'router Self> {
        for end_wildcard in &self.end_wildcard_children {
            if !Self::check_constraint(end_wildcard, path, constraints) {
                continue;
            }

            parameters.push(Parameter {
                key: unsafe { std::str::from_utf8_unchecked(&end_wildcard.prefix) },
                value: unsafe { std::str::from_utf8_unchecked(path) },
            });

            return if end_wildcard.data.is_some() {
                Some(end_wildcard)
            } else {
                None
            };
        }

        None
    }

    fn check_constraint(
        node: &Self,
        segment: &[u8],
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> bool {
        let name = match &node.constraint {
            Some(name) => name,
            _ => {
                return true;
            }
        };

        let constraint = constraints.get(name).map_or_else(
            || {
                // FIXME: Should be an error?
                unreachable!();
            },
            |constraint| constraint,
        );

        let segment = match std::str::from_utf8(segment) {
            Ok(segment) => segment,
            _ => {
                return false;
            }
        };

        (constraint.check)(segment)
    }
}
