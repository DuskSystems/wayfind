use super::{Node, ParameterConstraint};
use crate::{constraints::request::RequestConstraint, matches::Parameter};
use http::Request;
use smallvec::{smallvec, SmallVec};

impl<T, R> Node<T, R> {
    pub fn path_matches<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut SmallVec<[Parameter<'k, 'v>; 4]>,
    ) -> Option<&'k Self> {
        if path.is_empty() {
            return if self.data.is_some() { Some(self) } else { None };
        }

        if let Some(matches) = self.path_matches_static(path, parameters) {
            return Some(matches);
        }

        if let Some(matches) = self.path_matches_dynamic(path, parameters) {
            return Some(matches);
        }

        if let Some(matches) = self.path_matches_wildcard(path, parameters) {
            return Some(matches);
        }

        if let Some(matches) = self.path_matches_end_wildcard(path, parameters) {
            return Some(matches);
        }

        None
    }

    fn path_matches_static<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut SmallVec<[Parameter<'k, 'v>; 4]>,
    ) -> Option<&'k Self> {
        for static_child in &self.static_children {
            // NOTE: This was previously a "starts_with" call, but turns out this is much faster.
            if path.len() >= static_child.prefix.len()
                && static_child
                    .prefix
                    .iter()
                    .zip(path)
                    .all(|(a, b)| a == b)
            {
                let remaining_path = &path[static_child.prefix.len()..];
                if let Some(node) = static_child.path_matches(remaining_path, parameters) {
                    return Some(node);
                }
            }
        }

        None
    }

    fn path_matches_dynamic<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut SmallVec<[Parameter<'k, 'v>; 4]>,
    ) -> Option<&'k Self> {
        if self.quick_dynamic {
            self.path_matches_dynamic_segment(path, parameters)
        } else {
            self.path_matches_dynamic_inline(path, parameters)
        }
    }

    // Dynamic with support for inline dynamic sections, e.g. `<name>.<extension>`
    // NOTE: Parameters are greedy in nature:
    //   Route: `<name>.<extension>`
    //   Path: `my.long.file.txt`
    //   Name: `my.long.file`
    //   Ext: `txt`
    fn path_matches_dynamic_inline<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut SmallVec<[Parameter<'k, 'v>; 4]>,
    ) -> Option<&'k Self> {
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
                if !Self::check_parameter_constraints(dynamic_child, segment) {
                    continue;
                }

                let mut current_parameters = parameters.clone();
                current_parameters.push(Parameter {
                    key: &dynamic_child.prefix,
                    value: segment,
                });

                if let Some(node) = dynamic_child.path_matches(&path[consumed..], &mut current_parameters) {
                    last_match = Some(node);
                    last_match_parameters = current_parameters;
                }
            }

            if let Some(node) = last_match {
                *parameters = last_match_parameters;
                return Some(node);
            }
        }

        None
    }

    // Doesn't support inline dynamic sections, e.g. `<name>.<extension>`, only `/<segment>/`
    fn path_matches_dynamic_segment<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut SmallVec<[Parameter<'k, 'v>; 4]>,
    ) -> Option<&'k Self> {
        for dynamic_child in &self.dynamic_children {
            let segment_end = path
                .iter()
                .position(|&b| b == b'/')
                .unwrap_or(path.len());

            let segment = &path[..segment_end];
            if !Self::check_parameter_constraints(dynamic_child, segment) {
                continue;
            }

            parameters.push(Parameter {
                key: &dynamic_child.prefix,
                value: segment,
            });

            if let Some(node) = dynamic_child.path_matches(&path[segment_end..], parameters) {
                return Some(node);
            }

            parameters.pop();
        }

        None
    }

    fn path_matches_wildcard<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut SmallVec<[Parameter<'k, 'v>; 4]>,
    ) -> Option<&'k Self> {
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

                if !Self::check_parameter_constraints(wildcard_child, segment) {
                    break;
                }

                parameters.push(Parameter {
                    key: &wildcard_child.prefix,
                    value: segment,
                });

                if let Some(node) = wildcard_child.path_matches(&remaining_path[segment_end..], parameters) {
                    return Some(node);
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

    fn path_matches_end_wildcard<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut SmallVec<[Parameter<'k, 'v>; 4]>,
    ) -> Option<&'k Self> {
        for end_wildcard in &self.end_wildcard_children {
            if !Self::check_parameter_constraints(end_wildcard, path) {
                continue;
            }

            parameters.push(Parameter {
                key: &end_wildcard.prefix,
                value: path,
            });

            return if end_wildcard.data.is_some() {
                Some(end_wildcard)
            } else {
                None
            };
        }

        None
    }

    pub fn check_parameter_constraints(node: &Self, segment: &[u8]) -> bool {
        if node.parameter_constraints.is_empty() {
            return true;
        }

        let Ok(segment) = std::str::from_utf8(segment) else {
            return false;
        };

        node.parameter_constraints
            .iter()
            .all(|parameter_constraint| match parameter_constraint {
                ParameterConstraint::Regex(regex) => {
                    let Some(captures) = regex.captures(segment) else {
                        return false;
                    };

                    let Some(matches) = captures.get(0) else {
                        return false;
                    };

                    matches.start() == 0 && matches.end() == segment.len()
                }
                ParameterConstraint::Function(function) => function(segment),
            })
    }

    pub fn check_request_constraints(node: &Self, request: &Request<R>) -> bool {
        if node.request_constraints.is_empty() {
            return true;
        }

        node.request_constraints
            .iter()
            .all(|request_constraints| match request_constraints {
                RequestConstraint::Function(function) => function(request),
            })
    }
}
