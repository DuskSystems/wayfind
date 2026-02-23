use alloc::string::{String, ToString as _};
use alloc::vec;
use alloc::vec::Vec;

use smallvec::{SmallVec, smallvec};

use crate::errors::TemplateError;

/// Characters that are not allowed in parameter names.
const INVALID_PARAM_CHARS: [u8; 4] = [b'*', b'<', b'>', b'/'];

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Part {
    Static { prefix: Vec<u8> },
    Dynamic { name: String },
    Wildcard { name: String },
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Template {
    pub parts: Vec<Part>,
}

impl Template {
    pub fn new(input: &[u8]) -> Result<Self, TemplateError> {
        if input.is_empty() {
            return Err(TemplateError::Empty);
        }

        if input[0] != b'/' {
            return Err(TemplateError::MissingLeadingSlash);
        }

        let mut parts = vec![];
        let mut cursor = 0;

        let mut seen_parameters: SmallVec<[(String, usize); 4]> = smallvec![];

        while cursor < input.len() {
            match input[cursor] {
                b'<' => {
                    let (part, next) = Self::parse_parameter_part(input, cursor)?;

                    // Check for touching parameters.
                    if let Some((_, last)) = seen_parameters.last()
                        && cursor == *last
                    {
                        return Err(TemplateError::TouchingParameters);
                    }

                    // Check for duplicate names.
                    if let Part::Dynamic { name } | Part::Wildcard { name } = &part {
                        if seen_parameters.iter().any(|(existing, _)| existing == name) {
                            return Err(TemplateError::DuplicateParameter { name: name.clone() });
                        }

                        seen_parameters.push((name.clone(), next));
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

    fn parse_static_part(input: &[u8], cursor: usize) -> (Part, usize) {
        let mut prefix = vec![];

        let mut end = cursor;
        while end < input.len() {
            match input[end] {
                b'<' | b'>' => break,
                byte => {
                    prefix.push(byte);
                    end += 1;
                }
            }
        }

        (Part::Static { prefix }, end)
    }

    fn parse_parameter_part(input: &[u8], cursor: usize) -> Result<(Part, usize), TemplateError> {
        let start = cursor + 1;
        let end = memchr::memchr(b'>', &input[start..])
            .map(|position| start + position)
            .ok_or(TemplateError::UnbalancedAngle)?;

        let content = &input[start..end];
        if content.is_empty() {
            return Err(TemplateError::EmptyParameter);
        }

        let is_wildcard = content.starts_with(b"*");
        let name = if is_wildcard { &content[1..] } else { content };

        if is_wildcard && name.is_empty() {
            return Err(TemplateError::EmptyWildcard);
        }

        if name.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
            return Err(TemplateError::InvalidParameter {
                name: String::from_utf8_lossy(name).to_string(),
            });
        }

        let name =
            String::from_utf8(name.to_vec()).map_err(|_err| TemplateError::InvalidParameter {
                name: String::from_utf8_lossy(name).to_string(),
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
    use alloc::borrow::ToOwned as _;

    use similar_asserts::assert_eq;

    use super::*;

    #[test]
    fn parser_static_route() {
        assert_eq!(
            Template::new(b"/abcd"),
            Ok(Template {
                parts: vec![Part::Static {
                    prefix: b"/abcd".to_vec()
                }],
            }),
        );
    }

    #[test]
    fn parser_dynamic_route() {
        assert_eq!(
            Template::new(b"/<name>"),
            Ok(Template {
                parts: vec![
                    Part::Dynamic {
                        name: "name".to_owned(),
                    },
                    Part::Static {
                        prefix: b"/".to_vec()
                    },
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
                    Part::Wildcard {
                        name: "wildcard".to_owned(),
                    },
                    Part::Static {
                        prefix: b"/".to_vec()
                    },
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
                    Part::Wildcard {
                        name: "path".to_owned(),
                    },
                    Part::Static {
                        prefix: b"/files/".to_vec()
                    },
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
