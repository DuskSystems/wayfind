use super::Node;
use crate::matches::Parameter;
use std::collections::HashMap;

impl<T> Node<T> {
    pub fn matches<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut Vec<Parameter<'k, 'v>>,
        constraints: &HashMap<Vec<u8>, fn(&str) -> bool>,
    ) -> Option<&'k Self> {
        if path.is_empty() {
            return if self.data.is_some() {
                Some(self)
            } else {
                None
            };
        }

        if let Some(matches) = self.matches_static(path, parameters, constraints) {
            return Some(matches);
        }

        if let Some(matches) = self.matches_dynamic(path, parameters, constraints) {
            return Some(matches);
        }

        if let Some(matches) = self.matches_wildcard(path, parameters, constraints) {
            return Some(matches);
        }

        if let Some(matches) = self.matches_end_wildcard(path, parameters, constraints) {
            return Some(matches);
        }

        None
    }

    fn matches_static<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut Vec<Parameter<'k, 'v>>,
        constraints: &HashMap<Vec<u8>, fn(&str) -> bool>,
    ) -> Option<&'k Self> {
        for static_child in &self.static_children {
            // NOTE: This was previously a "starts_with" call, but turns out this is much faster.
            if path.len() >= static_child.prefix.len()
                && static_child.prefix.iter().zip(path).all(|(a, b)| a == b)
            {
                let remaining_path = &path[static_child.prefix.len()..];
                if let Some(node_data) =
                    static_child.matches(remaining_path, parameters, constraints)
                {
                    return Some(node_data);
                }
            }
        }

        None
    }

    fn matches_dynamic<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut Vec<Parameter<'k, 'v>>,
        constraints: &HashMap<Vec<u8>, fn(&str) -> bool>,
    ) -> Option<&'k Self> {
        if self.quick_dynamic {
            self.matches_dynamic_segment(path, parameters, constraints)
        } else {
            self.matches_dynamic_inline(path, parameters, constraints)
        }
    }

    // Dynamic with support for inline dynamic sections, e.g. `{name}.{extension}`
    // NOTE: Parameters are greedy in nature:
    //   Route: `{name}.{extension}`
    //   Path: `my.long.file.txt`
    //   Name: `my.long.file`
    //   Ext: `txt`
    fn matches_dynamic_inline<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut Vec<Parameter<'k, 'v>>,
        constraints: &HashMap<Vec<u8>, fn(&str) -> bool>,
    ) -> Option<&'k Self> {
        for dynamic_child in &self.dynamic_children {
            let mut consumed = 0;

            let mut last_match = None;
            let mut last_match_parameters = vec![];

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
                    key: &dynamic_child.prefix,
                    value: segment,
                });

                if let Some(node_data) =
                    dynamic_child.matches(&path[consumed..], &mut current_parameters, constraints)
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
    fn matches_dynamic_segment<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut Vec<Parameter<'k, 'v>>,
        constraints: &HashMap<Vec<u8>, fn(&str) -> bool>,
    ) -> Option<&'k Self> {
        for dynamic_child in &self.dynamic_children {
            let segment_end = path.iter().position(|&b| b == b'/').unwrap_or(path.len());

            let segment = &path[..segment_end];
            if !Self::check_constraint(dynamic_child, segment, constraints) {
                continue;
            }

            parameters.push(Parameter {
                key: &dynamic_child.prefix,
                value: segment,
            });

            if let Some(node_data) =
                dynamic_child.matches(&path[segment_end..], parameters, constraints)
            {
                return Some(node_data);
            }

            parameters.pop();
        }

        None
    }

    fn matches_wildcard<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut Vec<Parameter<'k, 'v>>,
        constraints: &HashMap<Vec<u8>, fn(&str) -> bool>,
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

                if !Self::check_constraint(wildcard_child, segment, constraints) {
                    break;
                }

                parameters.push(Parameter {
                    key: &wildcard_child.prefix,
                    value: segment,
                });

                if let Some(node_data) =
                    wildcard_child.matches(&remaining_path[segment_end..], parameters, constraints)
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

    fn matches_end_wildcard<'k, 'v>(
        &'k self,
        path: &'v [u8],
        parameters: &mut Vec<Parameter<'k, 'v>>,
        constraints: &HashMap<Vec<u8>, fn(&str) -> bool>,
    ) -> Option<&'k Self> {
        for end_wildcard in &self.end_wildcard_children {
            if !Self::check_constraint(end_wildcard, path, constraints) {
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

    fn check_constraint(
        node: &Self,
        segment: &[u8],
        constraints: &HashMap<Vec<u8>, fn(&str) -> bool>,
    ) -> bool {
        let Some(name) = &node.constraint else {
            return true;
        };

        let Some(constraint) = constraints.get(name) else {
            // FIXME: Should be an error?
            unreachable!();
        };

        let Ok(segment) = std::str::from_utf8(segment) else {
            return false;
        };

        (constraint)(segment)
    }
}
