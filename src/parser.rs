use alloc::string::{String, ToString as _};
use alloc::vec;
use alloc::vec::Vec;

use smallvec::{SmallVec, smallvec};

use crate::errors::TemplateError;

/// Characters that are not allowed in parameter names.
const INVALID_PARAM_CHARS: [u8; 4] = [b'*', b'<', b'>', b'/'];

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) enum Part<'a> {
    Static { prefix: &'a [u8] },
    Dynamic { name: &'a str },
    Wildcard { name: &'a str },
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct Template<'a> {
    pub parts: Vec<Part<'a>>,
}

impl<'a> Template<'a> {
    pub(crate) fn new(input: &'a [u8]) -> Result<Self, TemplateError> {
        if input.is_empty() {
            return Err(TemplateError::Empty);
        }

        if input[0] != b'/' {
            return Err(TemplateError::MissingLeadingSlash);
        }

        let mut parts = vec![];
        let mut cursor = 0;

        let mut seen_parameters: SmallVec<[(&str, usize); 4]> = smallvec![];

        while cursor < input.len() {
            match input[cursor] {
                b'<' => {
                    let (part, next) = Self::parse_parameter_part(input, cursor)?;

                    // Check for touching parameters.
                    if let Some((_, last)) = seen_parameters.last() {
                        if cursor == *last {
                            return Err(TemplateError::TouchingParameters);
                        }
                    }

                    // Check for duplicate names.
                    if let Part::Dynamic { name } | Part::Wildcard { name } = &part {
                        if seen_parameters.iter().any(|(existing, _)| existing == name) {
                            return Err(TemplateError::DuplicateParameter {
                                name: String::from(*name),
                            });
                        }

                        seen_parameters.push((name, next));
                    }

                    parts.push(part);
                    cursor = next;
                }
                b'>' => {
                    return Err(TemplateError::UnbalancedAngle);
                }
                _ => {
                    let (part, next_cursor) = Self::parse_static_part(input, cursor);

                    parts.push(part);
                    cursor = next_cursor;
                }
            }
        }

        parts.reverse();

        Ok(Self { parts })
    }

    fn parse_static_part(input: &'a [u8], cursor: usize) -> (Part<'a>, usize) {
        let end = memchr::memchr2(b'<', b'>', &input[cursor..])
            .map_or(input.len(), |position| cursor + position);

        let prefix = &input[cursor..end];
        (Part::Static { prefix }, end)
    }

    fn parse_parameter_part(
        input: &'a [u8],
        cursor: usize,
    ) -> Result<(Part<'a>, usize), TemplateError> {
        let start = cursor + 1;
        let end = memchr::memchr(b'>', &input[start..])
            .map(|position| start + position)
            .ok_or(TemplateError::UnbalancedAngle)?;

        let content = &input[start..end];
        if content.is_empty() {
            return Err(TemplateError::EmptyParameter);
        }

        let is_wildcard = content.starts_with(b"*");
        let name_bytes = if is_wildcard { &content[1..] } else { content };

        if is_wildcard && name_bytes.is_empty() {
            return Err(TemplateError::EmptyWildcard);
        }

        if name_bytes.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
            return Err(TemplateError::InvalidParameter {
                name: String::from_utf8_lossy(name_bytes).to_string(),
            });
        }

        let name: &'a str =
            core::str::from_utf8(name_bytes).map_err(|_err| TemplateError::InvalidParameter {
                name: String::from_utf8_lossy(name_bytes).to_string(),
            })?;

        let part = if is_wildcard {
            Part::Wildcard { name }
        } else {
            Part::Dynamic { name }
        };

        Ok((part, end + 1))
    }
}

#[cfg(test)]
mod tests {
    use similar_asserts::assert_eq;

    use super::*;

    #[test]
    fn parser_static_route() {
        assert_eq!(
            Template::new(b"/abcd"),
            Ok(Template {
                parts: vec![Part::Static { prefix: b"/abcd" }],
            }),
        );
    }

    #[test]
    fn parser_dynamic_route() {
        assert_eq!(
            Template::new(b"/<name>"),
            Ok(Template {
                parts: vec![
                    Part::Dynamic { name: "name" },
                    Part::Static { prefix: b"/" },
                ],
            }),
        );
    }

    #[test]
    fn parser_wildcard_route() {
        assert_eq!(
            Template::new(b"/<*wildcard>"),
            Ok(Template {
                parts: vec![
                    Part::Wildcard { name: "wildcard" },
                    Part::Static { prefix: b"/" },
                ],
            }),
        );
    }

    #[test]
    fn parser_route_with_wildcard_at_end() {
        assert_eq!(
            Template::new(b"/files/<*path>"),
            Ok(Template {
                parts: vec![
                    Part::Wildcard { name: "path" },
                    Part::Static { prefix: b"/files/" },
                ],
            }),
        );
    }

    #[test]
    fn parser_error_empty() {
        let error = Template::new(b"").unwrap_err();
        insta::assert_snapshot!(error, @"empty template");
    }

    #[test]
    fn parser_error_empty_parameter() {
        let error = Template::new(b"/users/<>").unwrap_err();
        insta::assert_snapshot!(error, @"empty parameter name");
    }

    #[test]
    fn parser_error_missing_leading_slash() {
        let error = Template::new(b"abc").unwrap_err();
        insta::assert_snapshot!(error, @"missing leading slash");
    }

    #[test]
    fn parser_error_unbalanced_angle_opening() {
        let error = Template::new(b"/users/<id/profile").unwrap_err();
        insta::assert_snapshot!(error, @"unbalanced angle bracket");
    }

    #[test]
    fn parser_error_unbalanced_angle_closing() {
        let error = Template::new(b"/users/id>/profile").unwrap_err();
        insta::assert_snapshot!(error, @"unbalanced angle bracket");
    }

    #[test]
    fn parser_error_invalid_parameter() {
        let error = Template::new(b"/users/<user*name>/profile").unwrap_err();
        insta::assert_snapshot!(error, @"invalid parameter name `user*name`");
    }

    #[test]
    fn parser_error_duplicate_parameter() {
        let error = Template::new(b"/users/<id>/posts/<id>").unwrap_err();
        insta::assert_snapshot!(error, @"duplicate parameter name `id`");
    }

    #[test]
    fn parser_error_empty_wildcard() {
        let error = Template::new(b"/files/<*>").unwrap_err();
        insta::assert_snapshot!(error, @"empty wildcard name");
    }

    #[test]
    fn parser_error_touching_parameters() {
        let error = Template::new(b"/users/<id><*name>").unwrap_err();
        insta::assert_snapshot!(error, @"parameters must be separated by a static segment");
    }
}
