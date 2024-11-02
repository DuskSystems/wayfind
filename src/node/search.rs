use super::Node;
use crate::{
    errors::{EncodingError, SearchError},
    router::{Parameter, Parameters, StoredConstraint},
};
use rustc_hash::FxHashMap;
use smallvec::smallvec;

impl<'r, T> Node<'r, T> {
    /// Searches for a matching route in the node tree.
    ///
    /// This method traverses the tree to find a route node that matches the given path, collecting parameters along the way.
    /// We try nodes in the order: static, dynamic, wildcard, then end wildcard.
    pub fn search<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r [u8], StoredConstraint>,
    ) -> Result<Option<&'r Self>, SearchError> {
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

    fn search_static<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r [u8], StoredConstraint>,
    ) -> Result<Option<&'r Self>, SearchError> {
        for child in self.static_children.iter() {
            // This was previously a "starts_with" call, but turns out this is much faster.
            if path.len() >= child.prefix.len()
                && child.prefix.iter().zip(path).all(|(a, b)| a == b)
            {
                let remaining_path = &path[child.prefix.len()..];
                if let Some(node) = child.search(remaining_path, parameters, constraints)? {
                    return Ok(Some(node));
                }
            }
        }

        Ok(None)
    }

    fn search_dynamic<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r [u8], StoredConstraint>,
    ) -> Result<Option<&'r Self>, SearchError> {
        if self.dynamic_children_shortcut {
            self.search_dynamic_segment(path, parameters, constraints)
        } else {
            self.search_dynamic_inline(path, parameters, constraints)
        }
    }

    /// Can handle complex dynamic routes like `{name}.{extension}`.
    fn search_dynamic_inline<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r [u8], StoredConstraint>,
    ) -> Result<Option<&'r Self>, SearchError> {
        for child in self.dynamic_children.iter() {
            let mut consumed = 0;

            let mut best_match: Option<&Self> = None;
            let mut best_match_parameters = smallvec![];

            while consumed < path.len() {
                if path[consumed] == b'/' {
                    break;
                }

                consumed += 1;

                let segment = &path[..consumed];
                if !Self::check_constraint(child, segment, constraints) {
                    continue;
                }

                let mut current_parameters = parameters.clone();
                current_parameters.push(Parameter {
                    key: std::str::from_utf8(&child.prefix).map_err(|_| {
                        EncodingError::Utf8Error {
                            input: String::from_utf8_lossy(&child.prefix).to_string(),
                        }
                    })?,
                    value: std::str::from_utf8(segment).map_err(|_| EncodingError::Utf8Error {
                        input: String::from_utf8_lossy(segment).to_string(),
                    })?,
                });

                let Some(node) =
                    child.search(&path[consumed..], &mut current_parameters, constraints)?
                else {
                    continue;
                };

                if best_match.map_or(true, |best| node.priority >= best.priority) {
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

    /// Can only handle simple dynamic routes like `/{segment}/`.
    fn search_dynamic_segment<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r [u8], StoredConstraint>,
    ) -> Result<Option<&'r Self>, SearchError> {
        for child in self.dynamic_children.iter() {
            let segment_end = path.iter().position(|&b| b == b'/').unwrap_or(path.len());

            let segment = &path[..segment_end];
            if !Self::check_constraint(child, segment, constraints) {
                continue;
            }

            parameters.push(Parameter {
                key: std::str::from_utf8(&child.prefix).map_err(|_| EncodingError::Utf8Error {
                    input: String::from_utf8_lossy(&child.prefix).to_string(),
                })?,
                value: std::str::from_utf8(segment).map_err(|_| EncodingError::Utf8Error {
                    input: String::from_utf8_lossy(segment).to_string(),
                })?,
            });

            if let Some(node) = child.search(&path[segment_end..], parameters, constraints)? {
                return Ok(Some(node));
            }

            parameters.pop();
        }

        Ok(None)
    }

    fn search_wildcard<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r [u8], StoredConstraint>,
    ) -> Result<Option<&'r Self>, SearchError> {
        if self.wildcard_children_shortcut {
            self.search_wildcard_segment(path, parameters, constraints)
        } else {
            self.search_wildcard_inline(path, parameters, constraints)
        }
    }

    /// Can handle complex wildcard routes like `/{*name}.{extension}`.
    fn search_wildcard_inline<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r [u8], StoredConstraint>,
    ) -> Result<Option<&'r Self>, SearchError> {
        for child in self.wildcard_children.iter() {
            let mut consumed = 0;

            let mut best_match: Option<&Self> = None;
            let mut best_match_parameters = smallvec![];

            while consumed < path.len() {
                consumed += 1;

                let segment = &path[..consumed];
                if !Self::check_constraint(child, segment, constraints) {
                    continue;
                }

                let mut current_parameters = parameters.clone();
                current_parameters.push(Parameter {
                    key: std::str::from_utf8(&child.prefix).map_err(|_| {
                        EncodingError::Utf8Error {
                            input: String::from_utf8_lossy(&child.prefix).to_string(),
                        }
                    })?,
                    value: std::str::from_utf8(segment).map_err(|_| EncodingError::Utf8Error {
                        input: String::from_utf8_lossy(segment).to_string(),
                    })?,
                });

                let Some(node) =
                    child.search(&path[consumed..], &mut current_parameters, constraints)?
                else {
                    continue;
                };

                if best_match.map_or(true, |best| node.priority >= best.priority) {
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

    /// Can only handle simple wildcard routes like `/{*segment}/`.
    fn search_wildcard_segment<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r [u8], StoredConstraint>,
    ) -> Result<Option<&'r Self>, SearchError> {
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

                if !Self::check_constraint(child, segment, constraints) {
                    break;
                }

                parameters.push(Parameter {
                    key: std::str::from_utf8(&child.prefix).map_err(|_| {
                        EncodingError::Utf8Error {
                            input: String::from_utf8_lossy(&child.prefix).to_string(),
                        }
                    })?,
                    value: std::str::from_utf8(segment).map_err(|_| EncodingError::Utf8Error {
                        input: String::from_utf8_lossy(segment).to_string(),
                    })?,
                });

                if let Some(node) =
                    child.search(&remaining_path[segment_end..], parameters, constraints)?
                {
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

    fn search_end_wildcard<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r [u8], StoredConstraint>,
    ) -> Result<Option<&'r Self>, SearchError> {
        for child in self.end_wildcard_children.iter() {
            if !Self::check_constraint(child, path, constraints) {
                continue;
            }

            parameters.push(Parameter {
                key: std::str::from_utf8(&child.prefix).map_err(|_| EncodingError::Utf8Error {
                    input: String::from_utf8_lossy(&child.prefix).to_string(),
                })?,
                value: std::str::from_utf8(path).map_err(|_| EncodingError::Utf8Error {
                    input: String::from_utf8_lossy(path).to_string(),
                })?,
            });

            return if child.data.is_some() {
                Ok(Some(child))
            } else {
                Ok(None)
            };
        }

        Ok(None)
    }

    fn check_constraint(
        node: &Self,
        segment: &[u8],
        constraints: &FxHashMap<&'r [u8], StoredConstraint>,
    ) -> bool {
        let Some(name) = &node.constraint else {
            return true;
        };

        let constraint = constraints.get(name.as_ref()).unwrap();
        let Ok(segment) = std::str::from_utf8(segment) else {
            return false;
        };

        (constraint.check)(segment)
    }
}
