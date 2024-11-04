use super::{Data, Node, State};
use crate::{
    errors::{EncodingError, SearchError},
    router::{Parameter, Parameters, StoredConstraint},
};
use rustc_hash::FxHashMap;
use smallvec::smallvec;

impl<'r, T, S: State> Node<'r, T, S> {
    /// Searches for a matching route in the node tree.
    ///
    /// This method traverses the tree to find a route node that matches the given path, collecting parameters along the way.
    /// We try nodes in the order: static, dynamic, wildcard, then end wildcard.
    pub fn search<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r Data<'r, T>, usize)>, SearchError> {
        if path.is_empty() {
            return Ok(self.data.as_ref().map(|data| (data, self.priority)));
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
        constraints: &FxHashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r Data<'r, T>, usize)>, SearchError> {
        for child in self.static_children.iter() {
            if path.len() >= child.state.prefix.len()
                && child.state.prefix.iter().zip(path).all(|(a, b)| a == b)
            {
                let remaining_path = &path[child.state.prefix.len()..];
                if let Some((data, priority)) =
                    child.search(remaining_path, parameters, constraints)?
                {
                    return Ok(Some((data, priority)));
                }
            }
        }

        Ok(None)
    }

    fn search_dynamic<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r Data<'r, T>, usize)>, SearchError> {
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
        constraints: &FxHashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r Data<'r, T>, usize)>, SearchError> {
        for child in self.dynamic_children.iter() {
            let mut consumed = 0;

            let mut best_match: Option<(&'r Data<'r, T>, usize)> = None;
            let mut best_match_parameters = smallvec![];

            while consumed < path.len() {
                if path[consumed] == b'/' {
                    break;
                }

                consumed += 1;

                let segment = &path[..consumed];
                if !Self::check_constraint(child.state.constraint.as_ref(), segment, constraints) {
                    continue;
                }

                let mut current_parameters = parameters.clone();
                current_parameters.push(Parameter {
                    key: &child.state.name,
                    value: std::str::from_utf8(segment).map_err(|_| EncodingError::Utf8Error {
                        input: String::from_utf8_lossy(segment).to_string(),
                    })?,
                });

                let Some((data, priority)) =
                    child.search(&path[consumed..], &mut current_parameters, constraints)?
                else {
                    continue;
                };

                if best_match.map_or(true, |(_, best_priority)| priority >= best_priority) {
                    best_match = Some((data, priority));
                    best_match_parameters = current_parameters;
                }
            }

            if let Some(result) = best_match {
                *parameters = best_match_parameters;
                return Ok(Some(result));
            }
        }

        Ok(None)
    }

    /// Can only handle simple dynamic routes like `/{segment}/`.
    fn search_dynamic_segment<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r Data<'r, T>, usize)>, SearchError> {
        for child in self.dynamic_children.iter() {
            let segment_end = path.iter().position(|&b| b == b'/').unwrap_or(path.len());

            let segment = &path[..segment_end];
            if !Self::check_constraint(child.state.constraint.as_ref(), segment, constraints) {
                continue;
            }

            parameters.push(Parameter {
                key: &child.state.name,
                value: std::str::from_utf8(segment).map_err(|_| EncodingError::Utf8Error {
                    input: String::from_utf8_lossy(segment).to_string(),
                })?,
            });

            if let Some(result) = child.search(&path[segment_end..], parameters, constraints)? {
                return Ok(Some(result));
            }

            parameters.pop();
        }

        Ok(None)
    }

    fn search_wildcard<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r Data<'r, T>, usize)>, SearchError> {
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
        constraints: &FxHashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r Data<'r, T>, usize)>, SearchError> {
        for child in self.wildcard_children.iter() {
            let mut consumed = 0;

            let mut best_match: Option<(&'r Data<'r, T>, usize)> = None;
            let mut best_match_parameters = smallvec![];

            while consumed < path.len() {
                consumed += 1;

                let segment = &path[..consumed];
                if !Self::check_constraint(child.state.constraint.as_ref(), segment, constraints) {
                    continue;
                }

                let mut current_parameters = parameters.clone();
                current_parameters.push(Parameter {
                    key: &child.state.name,
                    value: std::str::from_utf8(segment).map_err(|_| EncodingError::Utf8Error {
                        input: String::from_utf8_lossy(segment).to_string(),
                    })?,
                });

                let Some((data, priority)) =
                    child.search(&path[consumed..], &mut current_parameters, constraints)?
                else {
                    continue;
                };

                if best_match.map_or(true, |(_, best_priority)| priority >= best_priority) {
                    best_match = Some((data, priority));
                    best_match_parameters = current_parameters;
                }
            }

            if let Some(result) = best_match {
                *parameters = best_match_parameters;
                return Ok(Some(result));
            }
        }

        Ok(None)
    }

    /// Can only handle simple wildcard routes like `/{*segment}/`.
    fn search_wildcard_segment<'p>(
        &'r self,
        path: &'p [u8],
        parameters: &mut Parameters<'r, 'p>,
        constraints: &FxHashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r Data<'r, T>, usize)>, SearchError> {
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

                if !Self::check_constraint(child.state.constraint.as_ref(), segment, constraints) {
                    break;
                }

                parameters.push(Parameter {
                    key: &child.state.name,
                    value: std::str::from_utf8(segment).map_err(|_| EncodingError::Utf8Error {
                        input: String::from_utf8_lossy(segment).to_string(),
                    })?,
                });

                if let Some(result) =
                    child.search(&remaining_path[segment_end..], parameters, constraints)?
                {
                    return Ok(Some(result));
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
        constraints: &FxHashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r Data<'r, T>, usize)>, SearchError> {
        for child in self.end_wildcard_children.iter() {
            if !Self::check_constraint(child.state.constraint.as_ref(), path, constraints) {
                continue;
            }

            parameters.push(Parameter {
                key: &child.state.name,
                value: std::str::from_utf8(path).map_err(|_| EncodingError::Utf8Error {
                    input: String::from_utf8_lossy(path).to_string(),
                })?,
            });

            return Ok(child.data.as_ref().map(|data| (data, child.priority)));
        }

        Ok(None)
    }

    fn check_constraint(
        constraint: Option<&String>,
        segment: &[u8],
        constraints: &FxHashMap<&'r str, StoredConstraint>,
    ) -> bool {
        let Some(constraint) = constraint else {
            return true;
        };

        let Some(constraint) = constraints.get(constraint.as_str()) else {
            unreachable!()
        };

        let Ok(segment) = std::str::from_utf8(segment) else {
            return false;
        };

        (constraint.check)(segment)
    }
}
