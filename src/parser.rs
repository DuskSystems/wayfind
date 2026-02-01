use alloc::string::{String, ToString as _};
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;

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

        if !input.is_empty() && input[0] != b'/' {
            return Err(TemplateError::MissingLeadingSlash {
                template: String::from_utf8_lossy(input).to_string(),
            });
        }

        let mut parts = vec![];
        let mut cursor = 0;

        let mut seen_parameters: SmallVec<[(String, Range<usize>); 4]> = smallvec![];
        let mut segment_parameters: SmallVec<[Range<usize>; 4]> = smallvec![];

        while cursor < input.len() {
            match input[cursor] {
                b'<' => {
                    let (part, next_cursor) = Self::parse_parameter_part(input, cursor)?;

                    // Check for touching parameters.
                    if let Some((first, last)) = seen_parameters.last()
                        && cursor == last.end
                        && let Part::Dynamic { name: second } | Part::Wildcard { name: second } =
                            &part
                    {
                        return Err(TemplateError::TouchingParameters {
                            template: String::from_utf8_lossy(input).to_string(),
                            first: first.clone(),
                            second: second.clone(),
                            position: last.start..next_cursor,
                        });
                    }

                    // Check for duplicate names.
                    if let Part::Dynamic { name } | Part::Wildcard { name } = &part {
                        if let Some((_, original)) = seen_parameters
                            .iter()
                            .find(|(existing, _)| existing == name)
                        {
                            return Err(TemplateError::DuplicateParameter {
                                template: String::from_utf8_lossy(input).to_string(),
                                name: name.clone(),
                                original: original.clone(),
                                duplicate: cursor..next_cursor,
                            });
                        }

                        seen_parameters.push((name.clone(), cursor..next_cursor));
                    }

                    segment_parameters.push(cursor..next_cursor);

                    parts.push(part);
                    cursor = next_cursor;
                }
                b'>' => {
                    return Err(TemplateError::UnbalancedAngle {
                        template: String::from_utf8_lossy(input).to_string(),
                        position: cursor..cursor + 1,
                    });
                }
                _ => {
                    let (part, next_cursor) = Self::parse_static_part(input, cursor);

                    if let Part::Static { prefix } = &part
                        && prefix.contains(&b'/')
                    {
                        if segment_parameters.len() > 1 {
                            return Err(TemplateError::TooManyParameters {
                                template: String::from_utf8_lossy(input).to_string(),
                                positions: segment_parameters.to_vec(),
                            });
                        }

                        segment_parameters.clear();
                    }

                    parts.push(part);
                    cursor = next_cursor;
                }
            }
        }

        if segment_parameters.len() > 1 {
            return Err(TemplateError::TooManyParameters {
                template: String::from_utf8_lossy(input).to_string(),
                positions: segment_parameters.to_vec(),
            });
        }

        parts.reverse();

        Ok(Self { parts })
    }

    fn parse_static_part(input: &[u8], cursor: usize) -> (Part, usize) {
        let mut prefix = vec![];

        let mut end = cursor;
        while end < input.len() {
            match (input[end], input.get(end + 1)) {
                (b'<' | b'>', _) => break,
                (char, _) => {
                    prefix.push(char);
                    end += 1;
                }
            }
        }

        (Part::Static { prefix }, end)
    }

    fn parse_parameter_part(input: &[u8], cursor: usize) -> Result<(Part, usize), TemplateError> {
        let start = cursor + 1;
        let mut end = start;

        let mut angle_count = 1;
        while end < input.len() {
            match input[end] {
                b'<' => angle_count += 1,
                b'>' => {
                    angle_count -= 1;
                    if angle_count == 0 {
                        break;
                    }
                }
                _ => {}
            }

            end += 1;
        }

        if angle_count != 0 {
            return Err(TemplateError::UnbalancedAngle {
                template: String::from_utf8_lossy(input).to_string(),
                position: cursor..cursor + 1,
            });
        }

        let content = &input[start..end];
        if content.is_empty() {
            return Err(TemplateError::EmptyParameter {
                template: String::from_utf8_lossy(input).to_string(),
                position: cursor..end + 1,
            });
        }

        let is_wildcard = content.starts_with(b"*");
        let name = if is_wildcard { &content[1..] } else { content };

        if is_wildcard && name.is_empty() {
            return Err(TemplateError::EmptyWildcard {
                template: String::from_utf8_lossy(input).to_string(),
                position: cursor..end + 1,
            });
        }

        if name.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
            return Err(TemplateError::InvalidParameter {
                template: String::from_utf8_lossy(input).to_string(),
                name: String::from_utf8_lossy(name).to_string(),
                position: cursor..end + 1,
            });
        }

        let name =
            String::from_utf8(name.to_vec()).map_err(|_name| TemplateError::InvalidParameter {
                template: String::from_utf8_lossy(input).to_string(),
                name: String::from_utf8_lossy(name).to_string(),
                position: cursor..end + 1,
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
        insta::assert_debug_snapshot!(error, @r"
        error: empty template

        help: templates must not be empty
        ");
    }

    #[test]
    fn parser_error_empty_parameter() {
        let error = Template::new(b"/users/<>").unwrap_err();
        insta::assert_snapshot!(error, @"empty parameter name in `/users/<>`");
        insta::assert_debug_snapshot!(error, @r"
        error: empty parameter name

            /users/<>
                   ━━

        help: provide a name between `<` and `>`
        ");
    }

    #[test]
    fn parser_error_missing_leading_slash() {
        let error = Template::new(b"abc").unwrap_err();
        insta::assert_snapshot!(error, @"missing leading slash in `abc`");
        insta::assert_debug_snapshot!(error, @r"
        error: missing leading slash

            abc
            ━━━

        help: templates must begin with `/`
        ");
    }

    #[test]
    fn parser_error_unbalanced_angle_opening() {
        let error = Template::new(b"/users/<id/profile").unwrap_err();
        insta::assert_snapshot!(error, @"unbalanced angle bracket in `/users/<id/profile`");
        insta::assert_debug_snapshot!(error, @r"
        error: unbalanced angle bracket

            /users/<id/profile
                   ━

        help: each `<` must have a matching `>`
        ");
    }

    #[test]
    fn parser_error_unbalanced_angle_closing() {
        let error = Template::new(b"/users/id>/profile").unwrap_err();
        insta::assert_snapshot!(error, @"unbalanced angle bracket in `/users/id>/profile`");
        insta::assert_debug_snapshot!(error, @r"
        error: unbalanced angle bracket

            /users/id>/profile
                     ━

        help: each `<` must have a matching `>`
        ");
    }

    #[test]
    fn parser_error_invalid_parameter() {
        let error = Template::new(b"/users/<user*name>/profile").unwrap_err();
        insta::assert_snapshot!(error, @"invalid parameter name `user*name` in `/users/<user*name>/profile`");
        insta::assert_debug_snapshot!(error, @r"
        error: invalid parameter name: `user*name`

            /users/<user*name>/profile
                   ━━━━━━━━━━━

        help: parameter names must not contain `*`, `<`, `>`, or `/`
        ");
    }

    #[test]
    fn parser_error_duplicate_parameter() {
        let error = Template::new(b"/users/<id>/posts/<id>").unwrap_err();
        insta::assert_snapshot!(error, @"duplicate parameter name `id` in `/users/<id>/posts/<id>`");
        insta::assert_debug_snapshot!(error, @r"
        error: duplicate parameter name: `id`

            /users/<id>/posts/<id>
                   ━━━━       ━━━━

        help: rename one of the parameters
        ");
    }

    #[test]
    fn parser_error_empty_wildcard() {
        let error = Template::new(b"/files/<*>").unwrap_err();
        insta::assert_snapshot!(error, @"empty wildcard name in `/files/<*>`");
        insta::assert_debug_snapshot!(error, @r"
        error: empty wildcard name

            /files/<*>
                   ━━━

        help: provide a name after `*`
        ");
    }

    #[test]
    fn parser_error_touching_parameters() {
        let error = Template::new(b"/users/<id><*name>").unwrap_err();
        insta::assert_snapshot!(error, @"touching parameters in `/users/<id><*name>`");
        insta::assert_debug_snapshot!(error, @r"
        error: touching parameters `id` and `name`

            /users/<id><*name>
                   ━━━━━━━━━━━

        help: parameters must be separated by at least one static segment
        ");
    }

    #[test]
    fn parser_error_too_many_parameters() {
        let error = Template::new(b"/<name>.<ext>").unwrap_err();
        insta::assert_snapshot!(error, @"too many parameters in segment in `/<name>.<ext>`");
        insta::assert_debug_snapshot!(error, @r"
        error: too many parameters in segment

            /<name>.<ext>
             ━━━━━━ ━━━━━

        help: only one parameter is allowed per segment
        ");
    }
}
