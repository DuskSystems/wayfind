use super::{Node, NodeData};
use crate::{errors::SearchError, router::StoredConstraint};
use std::{collections::HashMap, sync::Arc};

/// Stores data from a successful router match.
#[derive(Debug, Eq, PartialEq)]
pub struct Match<'router, 'path, T> {
    /// The matching route.
    pub route: Arc<str>,

    /// A reference to the matching route data.
    pub data: &'router T,

    /// Key-value pairs of parameters, extracted from the route.
    pub parameters: Vec<Parameter<'router, 'path>>,
}

/// A key-value parameter pair.
///
/// The key of the parameter is tied to the lifetime of the router, since it is a ref to the prefix of a given node.
/// Meanwhile, the value is extracted from the path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter<'router, 'path> {
    pub key: &'router str,
    pub value: &'path str,
}

impl<T> Node<T> {
    /// Searches for a matching route in the node tree.
    ///
    /// This method traverses the tree to find a route node that matches the given path, collecting parameters along the way.
    /// We try nodes in the order: static, dynamic, wildcard, then end wildcard.
    pub fn search<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut Vec<Parameter<'router, 'path>>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Result<Option<&'router Self>, SearchError> {
        if path.is_empty() {
            return if self.data.is_some() {
                Ok(Some(self))
            } else {
                Ok(None)
            };
        }

        if let Some(search) = self.search_static(path, parameters, constraints)? {
            return Ok(Some(search));
        }

        if let Some(search) = self.search_dynamic(path, parameters, constraints)? {
            return Ok(Some(search));
        }

        if let Some(search) = self.search_wildcard(path, parameters, constraints)? {
            return Ok(Some(search));
        }

        if let Some(search) = self.search_end_wildcard(path, parameters, constraints)? {
            return Ok(Some(search));
        }

        Ok(None)
    }

    fn search_static<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut Vec<Parameter<'router, 'path>>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Result<Option<&'router Self>, SearchError> {
        for static_child in &self.static_children {
            // This was previously a "starts_with" call, but turns out this is much faster.
            if path.len() >= static_child.prefix.len()
                && static_child.prefix.iter().zip(path).all(|(a, b)| a == b)
            {
                let remaining_path = &path[static_child.prefix.len()..];
                if let Some(node) = static_child.search(remaining_path, parameters, constraints)? {
                    return Ok(Some(node));
                }
            }
        }

        Ok(None)
    }

    fn search_dynamic<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut Vec<Parameter<'router, 'path>>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Result<Option<&'router Self>, SearchError> {
        if self.quick_dynamic {
            self.search_dynamic_segment(path, parameters, constraints)
        } else {
            self.search_dynamic_inline(path, parameters, constraints)
        }
    }

    /// Can handle complex dynamic routes like `{name}.{extension}`.
    /// It uses a greedy matching approach for parameters.
    /// We also prefer longer routes to shorter routes.
    fn search_dynamic_inline<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut Vec<Parameter<'router, 'path>>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Result<Option<&'router Self>, SearchError> {
        for dynamic_child in &self.dynamic_children {
            let mut consumed = 0;

            // Often the last match, except for when we have multiple options on the same branch.
            // In that case, the longer route is chosen.
            let mut best_match: Option<&Self> = None;
            let mut best_match_parameters = vec![];

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
                    key: std::str::from_utf8(&dynamic_child.prefix).map_err(|_| {
                        SearchError::Utf8Error {
                            key: String::from_utf8_lossy(&dynamic_child.prefix).to_string(),
                            value: String::from_utf8_lossy(segment).to_string(),
                        }
                    })?,
                    value: std::str::from_utf8(segment).map_err(|_| SearchError::Utf8Error {
                        key: String::from_utf8_lossy(&dynamic_child.prefix).to_string(),
                        value: String::from_utf8_lossy(segment).to_string(),
                    })?,
                });

                let Some(node) = dynamic_child.search(
                    &path[consumed..],
                    &mut current_parameters,
                    constraints,
                )?
                else {
                    continue;
                };

                if let Some(best) = &best_match {
                    if node.route_length() >= best.route_length() {
                        best_match = Some(node);
                        best_match_parameters = current_parameters;
                    }
                } else {
                    best_match = Some(node);
                    best_match_parameters = current_parameters;
                }
            }

            if let Some(node) = best_match {
                *parameters = best_match_parameters;
                return Ok(Some(node));
            }
        }

        Ok(None)
    }

    fn route_length(&self) -> usize {
        let Some(data) = self.data.as_ref() else {
            return 0;
        };

        match data {
            NodeData::Inline { route, .. } | NodeData::Reference(route) => route.len(),
        }
    }

    /// Can only handle simple dynamic routes like `/{segment}/`.
    fn search_dynamic_segment<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut Vec<Parameter<'router, 'path>>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Result<Option<&'router Self>, SearchError> {
        for dynamic_child in &self.dynamic_children {
            let segment_end = path.iter().position(|&b| b == b'/').unwrap_or(path.len());

            let segment = &path[..segment_end];
            if !Self::check_constraint(dynamic_child, segment, constraints) {
                continue;
            }

            parameters.push(Parameter {
                key: std::str::from_utf8(&dynamic_child.prefix).map_err(|_| {
                    SearchError::Utf8Error {
                        key: String::from_utf8_lossy(&dynamic_child.prefix).to_string(),
                        value: String::from_utf8_lossy(segment).to_string(),
                    }
                })?,
                value: std::str::from_utf8(segment).map_err(|_| SearchError::Utf8Error {
                    key: String::from_utf8_lossy(&dynamic_child.prefix).to_string(),
                    value: String::from_utf8_lossy(segment).to_string(),
                })?,
            });

            if let Some(node) =
                dynamic_child.search(&path[segment_end..], parameters, constraints)?
            {
                return Ok(Some(node));
            }

            parameters.pop();
        }

        Ok(None)
    }

    fn search_wildcard<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut Vec<Parameter<'router, 'path>>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Result<Option<&'router Self>, SearchError> {
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
                    key: std::str::from_utf8(&wildcard_child.prefix).map_err(|_| {
                        SearchError::Utf8Error {
                            key: String::from_utf8_lossy(&wildcard_child.prefix).to_string(),
                            value: String::from_utf8_lossy(segment).to_string(),
                        }
                    })?,
                    value: std::str::from_utf8(segment).map_err(|_| SearchError::Utf8Error {
                        key: String::from_utf8_lossy(&wildcard_child.prefix).to_string(),
                        value: String::from_utf8_lossy(segment).to_string(),
                    })?,
                });

                if let Some(node) = wildcard_child.search(
                    &remaining_path[segment_end..],
                    parameters,
                    constraints,
                )? {
                    return Ok(Some(node));
                }

                parameters.pop();

                if segment_end == remaining_path.len() {
                    break;
                }

                remaining_path = &remaining_path[segment_end + 1..];
            }
        }

        Ok(None)
    }

    fn search_end_wildcard<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut Vec<Parameter<'router, 'path>>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Result<Option<&'router Self>, SearchError> {
        for end_wildcard_child in &self.end_wildcard_children {
            if !Self::check_constraint(end_wildcard_child, path, constraints) {
                continue;
            }

            parameters.push(Parameter {
                key: std::str::from_utf8(&end_wildcard_child.prefix).map_err(|_| {
                    SearchError::Utf8Error {
                        key: String::from_utf8_lossy(&end_wildcard_child.prefix).to_string(),
                        value: String::from_utf8_lossy(path).to_string(),
                    }
                })?,
                value: std::str::from_utf8(path).map_err(|_| SearchError::Utf8Error {
                    key: String::from_utf8_lossy(&end_wildcard_child.prefix).to_string(),
                    value: String::from_utf8_lossy(path).to_string(),
                })?,
            });

            return if end_wildcard_child.data.is_some() {
                Ok(Some(end_wildcard_child))
            } else {
                Ok(None)
            };
        }

        Ok(None)
    }

    fn check_constraint(
        node: &Self,
        segment: &[u8],
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> bool {
        let Some(name) = &node.constraint else {
            return true;
        };

        let constraint = constraints.get(name).unwrap();
        let Ok(segment) = std::str::from_utf8(segment) else {
            return false;
        };

        (constraint.check)(segment)
    }
}
