use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use smallvec::{SmallVec, smallvec};

use crate::errors::TemplateError;

/// Characters that are not allowed in parameter names.
const INVALID_PARAM_CHARS: [u8; 4] = [b'*', b'<', b'>', b'/'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part {
    Static { prefix: Vec<u8> },
    Dynamic { name: String },
    Wildcard { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

        // Parameters in order (name, start, length)
        let mut seen_parameters: SmallVec<[(String, usize, usize); 4]> = smallvec![];

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
                                name: name.to_string(),
                                first: *start,
                                first_length: *length,
                                second: cursor,
                                second_length: next_cursor - cursor,
                            });
                        }

                        seen_parameters.push((name.clone(), cursor, next_cursor - cursor));
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
            return Err(TemplateError::EmptyAngles {
                template: String::from_utf8_lossy(input).to_string(),
                position: cursor,
            });
        }

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
            String::from_utf8(name.to_vec()).map_err(|_| TemplateError::InvalidParameter {
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

    /// Calculate a specificity of this template.
    /// This is not a perfect solution, but appears 'good enough' for now.
    pub fn specificity(&self) -> usize {
        let mut static_length = 0;
        let mut dynamic_count = 0;
        let mut wildcard_count = 0;

        for part in &self.parts {
            match part {
                Part::Static { prefix } => {
                    static_length += prefix.len();
                }
                Part::Dynamic { .. } => {
                    dynamic_count += 1;
                }
                Part::Wildcard { .. } => {
                    wildcard_count += 1;
                }
            }
        }

        let mut specificity = static_length.saturating_mul(1000);
        specificity = specificity.saturating_sub(dynamic_count * 10);
        specificity = specificity.saturating_sub(wildcard_count * 100);
        specificity
    }
}

#[cfg(test)]
mod tests {
    use alloc::borrow::ToOwned;

    use similar_asserts::assert_eq;

    use super::*;

    #[test]
    fn test_parser_static_route() {
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
    fn test_parser_dynamic_route() {
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
    fn test_parser_wildcard_route() {
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
    fn test_parser_route_with_wildcard_at_end() {
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
    fn test_parser_error_empty() {
        let error = Template::new(b"").unwrap_err();
        assert_eq!(error, TemplateError::Empty);

        insta::assert_snapshot!(error, @"empty template");
    }

    #[test]
    fn test_parser_error_empty_angles() {
        let error = Template::new(b"/users/<>").unwrap_err();
        assert_eq!(
            error,
            TemplateError::EmptyAngles {
                template: "/users/<>".to_owned(),
                position: 7,
            }
        );

        insta::assert_snapshot!(error, @r"
        empty angles

            Template: /users/<>
                             ^^
        ");
    }

    #[test]
    fn test_parser_error_missing_leading_slash() {
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
    fn test_parser_error_unbalanced_angle_opening() {
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
    fn test_parser_error_unbalanced_angle_closing() {
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
    fn test_parser_error_invalid_parameter() {
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
    fn test_parser_error_duplicate_parameter() {
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
    fn test_parser_error_empty_wildcard() {
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
    fn test_parser_error_touching_parameters() {
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
}
