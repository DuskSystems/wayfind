use super::{node::Node, state::State, AuthorityData};
use crate::{
    errors::{AuthoritySearchError, EncodingError},
    router::authority::{AuthorityParameters, StoredConstraint},
};
use smallvec::smallvec;
use std::collections::HashMap;

impl<'r, S: State> Node<'r, S> {
    pub fn search<'p>(
        &'r self,
        authority: &'p [u8],
        parameters: &mut AuthorityParameters<'r, 'p>,
        constraints: &HashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r AuthorityData<'r>, usize)>, AuthoritySearchError> {
        if authority.is_empty() {
            return Ok(self.data.as_ref().map(|data| (data, self.priority)));
        }

        if let Some(search) = self.search_static(authority, parameters, constraints)? {
            return Ok(Some(search));
        }

        if let Some(search) = self.search_dynamic(authority, parameters, constraints)? {
            return Ok(Some(search));
        }

        if let Some(search) = self.search_wildcard(authority, parameters, constraints)? {
            return Ok(Some(search));
        }

        if let Some(search) = self.search_end_wildcard(authority, parameters, constraints)? {
            return Ok(Some(search));
        }

        Ok(None)
    }

    fn search_static<'p>(
        &'r self,
        authority: &'p [u8],
        parameters: &mut AuthorityParameters<'r, 'p>,
        constraints: &HashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r AuthorityData<'r>, usize)>, AuthoritySearchError> {
        for child in self.static_children.iter() {
            if authority.len() >= child.state.prefix.len()
                && child
                    .state
                    .prefix
                    .iter()
                    .zip(authority)
                    .all(|(a, b)| a == b)
            {
                let remaining = &authority[child.state.prefix.len()..];
                if let Some((data, priority)) = child.search(remaining, parameters, constraints)? {
                    return Ok(Some((data, priority)));
                }
            }
        }

        Ok(None)
    }

    fn search_dynamic<'p>(
        &'r self,
        authority: &'p [u8],
        parameters: &mut AuthorityParameters<'r, 'p>,
        constraints: &HashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r AuthorityData<'r>, usize)>, AuthoritySearchError> {
        if self.dynamic_children_shortcut {
            self.search_dynamic_segment(authority, parameters, constraints)
        } else {
            self.search_dynamic_inline(authority, parameters, constraints)
        }
    }

    fn search_dynamic_inline<'p>(
        &'r self,
        authority: &'p [u8],
        parameters: &mut AuthorityParameters<'r, 'p>,
        constraints: &HashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r AuthorityData<'r>, usize)>, AuthoritySearchError> {
        for child in self.dynamic_children.iter() {
            let mut consumed = 0;

            let mut best_match: Option<(&'r AuthorityData<'r>, usize)> = None;
            let mut best_match_parameters = smallvec![];

            while consumed < authority.len() {
                if authority[consumed] == b'.' {
                    break;
                }

                consumed += 1;

                let segment = &authority[..consumed];
                if !Self::check_constraint(child.state.constraint.as_ref(), segment, constraints) {
                    continue;
                }

                let mut current_parameters = parameters.clone();
                current_parameters.push((
                    &child.state.name,
                    std::str::from_utf8(segment).map_err(|_| EncodingError::Utf8Error {
                        input: String::from_utf8_lossy(segment).to_string(),
                    })?,
                ));

                let Some((data, priority)) =
                    child.search(&authority[consumed..], &mut current_parameters, constraints)?
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
                return Ok(Some(result));
            }
        }

        Ok(None)
    }

    fn search_dynamic_segment<'p>(
        &'r self,
        authority: &'p [u8],
        parameters: &mut AuthorityParameters<'r, 'p>,
        constraints: &HashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r AuthorityData<'r>, usize)>, AuthoritySearchError> {
        for child in self.dynamic_children.iter() {
            let segment_end = authority
                .iter()
                .position(|&b| b == b'.')
                .unwrap_or(authority.len());

            let segment = &authority[..segment_end];
            if !Self::check_constraint(child.state.constraint.as_ref(), segment, constraints) {
                continue;
            }

            parameters.push((
                &child.state.name,
                std::str::from_utf8(segment).map_err(|_| EncodingError::Utf8Error {
                    input: String::from_utf8_lossy(segment).to_string(),
                })?,
            ));

            if let Some(result) =
                child.search(&authority[segment_end..], parameters, constraints)?
            {
                return Ok(Some(result));
            }

            parameters.pop();
        }

        Ok(None)
    }

    fn search_wildcard<'p>(
        &'r self,
        authority: &'p [u8],
        parameters: &mut AuthorityParameters<'r, 'p>,
        constraints: &HashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r AuthorityData<'r>, usize)>, AuthoritySearchError> {
        if self.wildcard_children_shortcut {
            self.search_wildcard_segment(authority, parameters, constraints)
        } else {
            self.search_wildcard_inline(authority, parameters, constraints)
        }
    }

    fn search_wildcard_inline<'p>(
        &'r self,
        authority: &'p [u8],
        parameters: &mut AuthorityParameters<'r, 'p>,
        constraints: &HashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r AuthorityData<'r>, usize)>, AuthoritySearchError> {
        for child in self.wildcard_children.iter() {
            let mut consumed = 0;

            let mut best_match: Option<(&'r AuthorityData<'r>, usize)> = None;
            let mut best_match_parameters = smallvec![];

            while consumed < authority.len() {
                consumed += 1;

                let segment = &authority[..consumed];
                if !Self::check_constraint(child.state.constraint.as_ref(), segment, constraints) {
                    continue;
                }

                let mut current_parameters = parameters.clone();
                current_parameters.push((
                    &child.state.name,
                    std::str::from_utf8(segment).map_err(|_| EncodingError::Utf8Error {
                        input: String::from_utf8_lossy(segment).to_string(),
                    })?,
                ));

                let Some((data, priority)) =
                    child.search(&authority[consumed..], &mut current_parameters, constraints)?
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
                return Ok(Some(result));
            }
        }

        Ok(None)
    }

    fn search_wildcard_segment<'p>(
        &'r self,
        authority: &'p [u8],
        parameters: &mut AuthorityParameters<'r, 'p>,
        constraints: &HashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r AuthorityData<'r>, usize)>, AuthoritySearchError> {
        for child in self.wildcard_children.iter() {
            let mut consumed = 0;
            let mut remaining = authority;
            let mut section_end = false;

            while !remaining.is_empty() {
                if section_end {
                    consumed += 1;
                }

                let segment_end = remaining
                    .iter()
                    .position(|&b| b == b'.')
                    .unwrap_or(remaining.len());

                if segment_end == 0 {
                    consumed += 1;
                    section_end = false;
                } else {
                    consumed += segment_end;
                    section_end = true;
                }

                let segment = if authority[..consumed].ends_with(b".") {
                    &authority[..consumed - 1]
                } else {
                    &authority[..consumed]
                };

                if !Self::check_constraint(child.state.constraint.as_ref(), segment, constraints) {
                    break;
                }

                parameters.push((
                    &child.state.name,
                    std::str::from_utf8(segment).map_err(|_| EncodingError::Utf8Error {
                        input: String::from_utf8_lossy(segment).to_string(),
                    })?,
                ));

                if let Some(result) =
                    child.search(&remaining[segment_end..], parameters, constraints)?
                {
                    return Ok(Some(result));
                }

                parameters.pop();

                if segment_end == remaining.len() {
                    break;
                }

                remaining = &remaining[segment_end + 1..];
            }
        }

        Ok(None)
    }

    fn search_end_wildcard<'p>(
        &'r self,
        authority: &'p [u8],
        parameters: &mut AuthorityParameters<'r, 'p>,
        constraints: &HashMap<&'r str, StoredConstraint>,
    ) -> Result<Option<(&'r AuthorityData<'r>, usize)>, AuthoritySearchError> {
        for child in self.end_wildcard_children.iter() {
            if !Self::check_constraint(child.state.constraint.as_ref(), authority, constraints) {
                continue;
            }

            parameters.push((
                &child.state.name,
                std::str::from_utf8(authority).map_err(|_| EncodingError::Utf8Error {
                    input: String::from_utf8_lossy(authority).to_string(),
                })?,
            ));

            return Ok(child.data.as_ref().map(|data| (data, child.priority)));
        }

        Ok(None)
    }

    fn check_constraint(
        constraint: Option<&String>,
        segment: &[u8],
        constraints: &HashMap<&'r str, StoredConstraint>,
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
