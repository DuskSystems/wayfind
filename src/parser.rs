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

        if !input.is_empty() && input[0] != b'/' {
            return Err(TemplateError::MissingLeadingSlash {
                template: String::from_utf8_lossy(input).to_string(),
            });
        }

        let mut parts = vec![];
        let mut cursor = 0;

        let mut seen_parameters: SmallVec<[(String, usize, usize); 4]> = smallvec![];
        let mut wildcard_parameters: SmallVec<[(usize, usize); 4]> = smallvec![];
        let mut segment_parameters: SmallVec<[(usize, usize); 4]> = smallvec![];

        while cursor < input.len() {
            match input[cursor] {
                b'<' => {
                    let (part, next_cursor) = Self::parse_parameter_part(input, cursor)?;

                    // Check for touching parameters.
                    if let Some((_, start, length)) = seen_parameters.last() {
                        if cursor == start + length {
                            return Err(TemplateError::TouchingParameters {
                                template: String::from_utf8_lossy(input).to_string(),
                                start: *start,
                                length: next_cursor - start,
                            });
                        }
                    }

                    // Check for duplicate names.
                    if let Part::Dynamic { name } | Part::Wildcard { name } = &part {
                        if let Some((_, start, length)) = seen_parameters
                            .iter()
                            .find(|(existing, _, _)| existing == name)
                        {
                            return Err(TemplateError::DuplicateParameter {
                                template: String::from_utf8_lossy(input).to_string(),
                                name: name.clone(),
                                first: *start,
                                first_length: *length,
                                second: cursor,
                                second_length: next_cursor - cursor,
                            });
                        }

                        seen_parameters.push((name.clone(), cursor, next_cursor - cursor));
                    }

                    segment_parameters.push((cursor, next_cursor - cursor));

                    if matches!(part, Part::Wildcard { .. }) {
                        wildcard_parameters.push((cursor, next_cursor - cursor));
                        if wildcard_parameters.len() > 2 {
                            return Err(TemplateError::TooManyWildcards {
                                template: String::from_utf8_lossy(input).to_string(),
                                parameters: wildcard_parameters.to_vec(),
                            });
                        }
                    }

                    parts.push(part);
                    cursor = next_cursor;
                }
                b'>' => {
                    return Err(TemplateError::UnbalancedAngle {
                        template: String::from_utf8_lossy(input).to_string(),
                        position: cursor,
                    });
                }
                _ => {
                    let (part, next_cursor) = Self::parse_static_part(input, cursor);

                    if let Part::Static { prefix } = &part {
                        if prefix.contains(&b'/') {
                            if segment_parameters.len() > 2 {
                                return Err(TemplateError::TooManyInline {
                                    template: String::from_utf8_lossy(input).to_string(),
                                    parameters: segment_parameters.to_vec(),
                                });
                            }

                            segment_parameters.clear();
                        }
                    }

                    parts.push(part);
                    cursor = next_cursor;
                }
            }
        }

        if segment_parameters.len() > 2 {
            return Err(TemplateError::TooManyInline {
                template: String::from_utf8_lossy(input).to_string(),
                parameters: segment_parameters.to_vec(),
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
                position: cursor,
            });
        }

        let content = &input[start..end];
        if content.is_empty() {
            return Err(TemplateError::EmptyParameter {
                template: String::from_utf8_lossy(input).to_string(),
                start: cursor,
                length: end - cursor + 1,
            });
        }

        let is_wildcard = content.starts_with(b"*");
        let name = if is_wildcard { &content[1..] } else { content };

        if is_wildcard && name.is_empty() {
            return Err(TemplateError::EmptyWildcard {
                template: String::from_utf8_lossy(input).to_string(),
                start: cursor,
                length: end - cursor + 1,
            });
        }

        if name.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
            return Err(TemplateError::InvalidParameter {
                template: String::from_utf8_lossy(input).to_string(),
                name: String::from_utf8_lossy(name).to_string(),
                start: cursor,
                length: end - cursor + 1,
            });
        }

        let name =
            String::from_utf8(name.to_vec()).map_err(|_name| TemplateError::InvalidParameter {
                template: String::from_utf8_lossy(input).to_string(),
                name: String::from_utf8_lossy(name).to_string(),
                start: cursor,
                length: end - cursor + 1,
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
        assert_eq!(error, TemplateError::Empty);

        insta::assert_snapshot!(error, @"empty template");
    }

    #[test]
    fn parser_error_empty_parameter() {
        let error = Template::new(b"/users/<>").unwrap_err();
        assert_eq!(
            error,
            TemplateError::EmptyParameter {
                template: "/users/<>".to_owned(),
                start: 7,
                length: 2,
            }
        );

        insta::assert_snapshot!(error, @r"
        empty parameter name

            Template: /users/<>
                             ^^
        ");
    }

    #[test]
    fn parser_error_missing_leading_slash() {
        let error = Template::new(b"abc").unwrap_err();
        assert_eq!(
            error,
            TemplateError::MissingLeadingSlash {
                template: "abc".to_owned(),
            }
        );

        insta::assert_snapshot!(error, @r"
        missing leading slash

            Template: abc

        help: Templates must begin with '/'
        ");
    }

    #[test]
    fn parser_error_unbalanced_angle_opening() {
        let error = Template::new(b"/users/<id/profile").unwrap_err();
        assert_eq!(
            error,
            TemplateError::UnbalancedAngle {
                template: "/users/<id/profile".to_owned(),
                position: 7,
            }
        );

        insta::assert_snapshot!(error, @r"
        unbalanced angle

            Template: /users/<id/profile
                             ^

        help: Each '<' must have a matching '>'

        try:
            - Add the missing closing angle
        ");
    }

    #[test]
    fn parser_error_unbalanced_angle_closing() {
        let error = Template::new(b"/users/id>/profile").unwrap_err();
        assert_eq!(
            error,
            TemplateError::UnbalancedAngle {
                template: "/users/id>/profile".to_owned(),
                position: 9,
            }
        );

        insta::assert_snapshot!(error, @r"
        unbalanced angle

            Template: /users/id>/profile
                               ^

        help: Each '<' must have a matching '>'

        try:
            - Add the missing closing angle
        ");
    }

    #[test]
    fn parser_error_invalid_parameter() {
        let error = Template::new(b"/users/<user*name>/profile").unwrap_err();
        assert_eq!(
            error,
            TemplateError::InvalidParameter {
                template: "/users/<user*name>/profile".to_owned(),
                name: "user*name".to_owned(),
                start: 7,
                length: 11,
            }
        );

        insta::assert_snapshot!(error, @r"
        invalid parameter name: 'user*name'

            Template: /users/<user*name>/profile
                             ^^^^^^^^^^^

        help: Parameter names must not contain the characters: '*', '<', '>', '/'
        ");
    }

    #[test]
    fn parser_error_duplicate_parameter() {
        let error = Template::new(b"/users/<id>/posts/<id>").unwrap_err();
        assert_eq!(
            error,
            TemplateError::DuplicateParameter {
                template: "/users/<id>/posts/<id>".to_owned(),
                name: "id".to_owned(),
                first: 7,
                first_length: 4,
                second: 18,
                second_length: 4,
            }
        );

        insta::assert_snapshot!(error, @r"
        duplicate parameter name: 'id'

            Template: /users/<id>/posts/<id>
                             ^^^^       ^^^^

        help: Parameter names must be unique within a template

        try:
            - Rename one of the parameters to be unique
        ");
    }

    #[test]
    fn parser_error_empty_wildcard() {
        let error = Template::new(b"/files/<*>").unwrap_err();
        assert_eq!(
            error,
            TemplateError::EmptyWildcard {
                template: "/files/<*>".to_owned(),
                start: 7,
                length: 3,
            }
        );

        insta::assert_snapshot!(error, @r"
        empty wildcard name

            Template: /files/<*>
                             ^^^
        ");
    }

    #[test]
    fn parser_error_touching_parameters() {
        let error = Template::new(b"/users/<id><*name>").unwrap_err();
        assert_eq!(
            error,
            TemplateError::TouchingParameters {
                template: "/users/<id><*name>".to_owned(),
                start: 7,
                length: 11,
            }
        );

        insta::assert_snapshot!(error, @r"
        touching parameters

            Template: /users/<id><*name>
                             ^^^^^^^^^^^

        help: Parameters must be separated by at least one part

        try:
            - Add a part between the parameters
            - Combine the parameters if they represent a single value
        ");
    }

    #[test]
    fn parser_error_too_many_parameters() {
        let error = Template::new(b"/<year>-<month>-<day>").unwrap_err();
        assert_eq!(
            error,
            TemplateError::TooManyInline {
                template: "/<year>-<month>-<day>".to_owned(),
                parameters: vec![(1, 6), (8, 7), (16, 5)],
            }
        );

        insta::assert_snapshot!(error, @"
        too many parameters in segment

            Template: /<year>-<month>-<day>
                       ^^^^^^ ^^^^^^^ ^^^^^

        help: At most 2 parameters are allowed per segment
        ");
    }

    #[test]
    fn parser_error_too_many_wildcards() {
        let error = Template::new(b"/<*a>/x/<*b>/y/<*c>").unwrap_err();
        assert_eq!(
            error,
            TemplateError::TooManyWildcards {
                template: "/<*a>/x/<*b>/y/<*c>".to_owned(),
                parameters: vec![(1, 4), (8, 4), (15, 4)],
            }
        );

        insta::assert_snapshot!(error, @"
        too many wildcards

            Template: /<*a>/x/<*b>/y/<*c>
                       ^^^^   ^^^^   ^^^^

        help: At most 2 wildcards are allowed per template
        ");
    }
}
