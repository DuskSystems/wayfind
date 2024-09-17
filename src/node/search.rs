use super::{Data, Node};
use crate::{errors::SearchError, router::StoredConstraint};
use std::{collections::HashMap, sync::Arc};

/// Stores data from a successful router match.
#[derive(Debug, Eq, PartialEq)]
pub struct Match<'router, 'path, T> {
    /// The matching route.
    pub route: Arc<str>,

    /// The expanded route, if applicable.
    pub expanded: Option<Arc<str>>,

    /// A reference to the matching route data.
    pub data: &'router T,

    /// Key-value pairs of parameters, extracted from the route.
    pub parameters: Vec<Parameter<'router, 'path>>,
}

/// A key-value parameter pair.
///
/// The key of the parameter is tied to the lifetime of the router, since it is a ref to the prefix of a given node.
/// Meanwhile, the value is extracted from the path.
#[derive(Clone, Eq, PartialEq)]
pub struct Parameter<'router, 'path> {
    pub key: &'router str,
    pub value: &'path str,
}

impl<'router, 'path> std::fmt::Debug for Parameter<'router, 'path> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=\"{}\"", self.key, self.value)
    }
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
        #[cfg(feature = "tracing")]
        tracing::debug!(
            path = ?String::from_utf8_lossy(path),
            "Searching for path in node tree"
        );

        if path.is_empty() {
            #[cfg(feature = "tracing")]
            tracing::debug!("Path is empty, checking if current node has data");

            return if self.data.is_some() {
                #[cfg(feature = "tracing")]
                tracing::debug!("Found matching node with data");

                Ok(Some(self))
            } else {
                #[cfg(feature = "tracing")]
                tracing::debug!("No matching node found");

                Ok(None)
            };
        }

        if let Some(search) = self.search_static(path, parameters, constraints)? {
            #[cfg(feature = "tracing")]
            tracing::debug!("Found matching static node");

            return Ok(Some(search));
        }

        if let Some(search) = self.search_dynamic(path, parameters, constraints)? {
            #[cfg(feature = "tracing")]
            tracing::debug!("Found matching dynamic node");

            return Ok(Some(search));
        }

        if let Some(search) = self.search_wildcard(path, parameters, constraints)? {
            #[cfg(feature = "tracing")]
            tracing::debug!("Found matching wildcard node");

            return Ok(Some(search));
        }

        if let Some(search) = self.search_end_wildcard(path, parameters, constraints)? {
            #[cfg(feature = "tracing")]
            tracing::debug!("Found matching end wildcard node");

            return Ok(Some(search));
        }

        #[cfg(feature = "tracing")]
        tracing::debug!("No matching node found");

        Ok(None)
    }

    fn search_static<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut Vec<Parameter<'router, 'path>>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Result<Option<&'router Self>, SearchError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            path = ?String::from_utf8_lossy(path),
            "Searching static children"
        );

        for static_child in self.static_children.iter() {
            // This was previously a "starts_with" call, but turns out this is much faster.
            if path.len() >= static_child.prefix.len()
                && static_child.prefix.iter().zip(path).all(|(a, b)| a == b)
            {
                #[cfg(feature = "tracing")]
                tracing::debug!(
                    prefix = ?String::from_utf8_lossy(&static_child.prefix),
                    "Found matching static prefix"
                );

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
        #[cfg(feature = "tracing")]
        tracing::debug!(
            path = ?String::from_utf8_lossy(path),
            "Searching dynamic children"
        );

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
        #[cfg(feature = "tracing")]
        tracing::debug!(
            path = ?String::from_utf8_lossy(path),
            "Searching dynamic children inline"
        );

        for dynamic_child in self.dynamic_children.iter() {
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

                #[cfg(feature = "tracing")]
                tracing::debug!(
                    current_route_length = node.route_length(),
                    best_route_length = best_match.map(Self::route_length),
                    "Comparing current match to best match"
                );

                if let Some(best) = &best_match {
                    if node.route_length() >= best.route_length() {
                        #[cfg(feature = "tracing")]
                        tracing::debug!("Found better or equal length match");

                        best_match = Some(node);
                        best_match_parameters = current_parameters;
                    } else {
                        #[cfg(feature = "tracing")]
                        tracing::debug!("Current match is shorter, keeping previous best match");
                    }
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("First match found, setting as best match");

                    best_match = Some(node);
                    best_match_parameters = current_parameters;
                }
            }

            if let Some(node) = best_match {
                #[cfg(feature = "tracing")]
                tracing::debug!(
                    route_length = node.route_length(),
                    "Found best matching dynamic node"
                );

                *parameters = best_match_parameters;
                return Ok(Some(node));
            }
        }

        #[cfg(feature = "tracing")]
        tracing::debug!("No matching dynamic node found");

        Ok(None)
    }

    fn route_length(&self) -> usize {
        let Some(data) = self.data.as_ref() else {
            return 0;
        };

        match data {
            Data::Inline { route, .. } => route.len(),
            Data::Shared { expanded, .. } => expanded.len(),
        }
    }

    /// Can only handle simple dynamic routes like `/{segment}/`.
    fn search_dynamic_segment<'router, 'path>(
        &'router self,
        path: &'path [u8],
        parameters: &mut Vec<Parameter<'router, 'path>>,
        constraints: &HashMap<Vec<u8>, StoredConstraint>,
    ) -> Result<Option<&'router Self>, SearchError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            path = ?String::from_utf8_lossy(path),
            "Searching dynamic children by segment"
        );

        for dynamic_child in self.dynamic_children.iter() {
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
                #[cfg(feature = "tracing")]
                tracing::debug!("Found matching dynamic segment node");

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
        #[cfg(feature = "tracing")]
        tracing::debug!(
            path = ?String::from_utf8_lossy(path),
            "Searching wildcard children"
        );

        for wildcard_child in self.wildcard_children.iter() {
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
                    #[cfg(feature = "tracing")]
                    tracing::debug!("Found matching wildcard node");

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
        #[cfg(feature = "tracing")]
        tracing::debug!(
            path = ?String::from_utf8_lossy(path),
            "Searching end wildcard children"
        );

        for end_wildcard_child in self.end_wildcard_children.iter() {
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

            #[cfg(feature = "tracing")]
            tracing::debug!("Found matching end wildcard node");

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

        #[cfg(feature = "tracing")]
        tracing::debug!(
            constraint = ?String::from_utf8_lossy(name),
            segment = segment,
            "Checking constraint"
        );

        let result = (constraint.check)(segment);

        #[cfg(feature = "tracing")]
        tracing::debug!(
            constraint = ?String::from_utf8_lossy(name),
            segment = segment,
            result = result,
            "Constraint check result"
        );

        result
    }
}
