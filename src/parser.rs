use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use smallvec::{SmallVec, smallvec};

use crate::errors::TemplateError;

/// Characters that are not allowed in parameter names or constraints.
const INVALID_PARAM_CHARS: [u8; 7] = [b':', b'*', b'{', b'}', b'(', b')', b'/'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub input: Vec<u8>,
    pub raw: Vec<u8>,
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part {
    Static { prefix: Vec<u8> },
    DynamicConstrained { name: String, constraint: String },
    Dynamic { name: String },
    WildcardConstrained { name: String, constraint: String },
    Wildcard { name: String },
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedTemplate {
    pub input: Vec<u8>,
    pub templates: Vec<Template>,
    pub expanded: bool,
}

impl ParsedTemplate {
    pub fn new(input: &[u8]) -> Result<Self, TemplateError> {
        if input.is_empty() {
            return Err(TemplateError::Empty);
        }

        let templates = Self::expand_optional_groups(input, 0, input.len())?;
        let templates = templates
            .into_iter()
            .map(|raw| Self::parse_template(input, &raw))
            .collect::<Result<Vec<_>, _>>()?;

        let expanded = templates.len() > 1;

        Ok(Self {
            input: input.to_vec(),
            templates,
            expanded,
        })
    }

    // Recursively expands optional groups in the template, generating all possible combinations
    fn expand_optional_groups(
        input: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Vec<Vec<u8>>, TemplateError> {
        let mut result = Vec::from([vec![]]);

        let mut cursor = start;
        let mut group = start;
        let mut depth = 0;

        while cursor < end {
            match (input[cursor], input.get(cursor + 1)) {
                (b'\\', Some(_)) => {
                    // Skip the backslash and the escaped character
                    cursor += 2;
                }
                (b'(', _) => {
                    if depth == 0 {
                        for template in &mut result {
                            template.extend_from_slice(&input[group..cursor]);
                        }

                        group = cursor + 1;
                    }

                    depth += 1;
                    cursor += 1;
                }
                (b')', _) => {
                    depth -= 1;

                    if depth < 0 {
                        return Err(TemplateError::UnbalancedParenthesis {
                            template: String::from_utf8_lossy(input).to_string(),
                            position: cursor,
                        });
                    }

                    if depth == 0 {
                        if cursor == group {
                            return Err(TemplateError::EmptyParentheses {
                                template: String::from_utf8_lossy(input).to_string(),
                                position: cursor - 1,
                            });
                        }

                        let optional_groups = Self::expand_optional_groups(input, group, cursor)?;

                        let mut new_result = vec![];
                        for template in result {
                            for optional_group in &optional_groups {
                                let mut new_template = template.clone();
                                new_template.extend_from_slice(optional_group);
                                new_result.push(new_template);
                            }

                            new_result.push(template);
                        }

                        result = new_result;
                        group = cursor + 1;
                    }

                    cursor += 1;
                }
                (_, _) => {
                    cursor += 1;
                }
            }
        }

        if depth != 0 {
            return Err(TemplateError::UnbalancedParenthesis {
                template: String::from_utf8_lossy(input).to_string(),
                position: start + group - 1,
            });
        }

        if group < end {
            for template in &mut result {
                template.extend_from_slice(&input[group..end]);
            }
        }

        for template in &mut result {
            if template.is_empty() {
                template.push(b'/');
            }
        }

        Ok(result)
    }

    fn parse_template(input: &[u8], raw: &[u8]) -> Result<Template, TemplateError> {
        if !raw.is_empty() && raw[0] != b'/' {
            return Err(TemplateError::MissingLeadingSlash {
                template: String::from_utf8_lossy(raw).to_string(),
            });
        }

        let mut parts = vec![];
        let mut cursor = 0;

        // Parameters in order (name, start, length)
        let mut seen_parameters: SmallVec<[(String, usize, usize); 4]> = smallvec![];

        while cursor < raw.len() {
            match raw[cursor] {
                b'{' => {
                    let (part, next_cursor) = Self::parse_parameter_part(raw, cursor)?;

                    // Check for touching parameters.
                    if let Some((_, start, length)) = seen_parameters.last() {
                        if cursor == start + length {
                            return Err(TemplateError::TouchingParameters {
                                template: String::from_utf8_lossy(raw).to_string(),
                                start: *start,
                                length: next_cursor - start,
                            });
                        }
                    }

                    // Check for duplicate names.
                    if let Part::Dynamic { name }
                    | Part::DynamicConstrained { name, .. }
                    | Part::Wildcard { name }
                    | Part::WildcardConstrained { name, .. } = &part
                    {
                        if let Some((_, start, length)) = seen_parameters
                            .iter()
                            .find(|(existing, _, _)| existing == name)
                        {
                            return Err(TemplateError::DuplicateParameter {
                                template: String::from_utf8_lossy(raw).to_string(),
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
                b'}' => {
                    return Err(TemplateError::UnbalancedBrace {
                        template: String::from_utf8_lossy(raw).to_string(),
                        position: cursor,
                    });
                }
                _ => {
                    let (part, next_cursor) = Self::parse_static_part(raw, cursor);
                    parts.push(part);
                    cursor = next_cursor;
                }
            }
        }

        parts.reverse();

        Ok(Template {
            input: input.to_vec(),
            raw: raw.to_vec(),
            parts,
        })
    }

    fn parse_static_part(input: &[u8], cursor: usize) -> (Part, usize) {
        let mut prefix = vec![];

        let mut end = cursor;
        while end < input.len() {
            match (input[end], input.get(end + 1)) {
                (b'\\', Some(&next_char)) => {
                    prefix.push(next_char);
                    end += 2;
                }
                (b'\\', None) => {
                    prefix.push(b'\\');
                    end += 1;
                }
                (b'{' | b'}', _) => break,
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

        let mut brace_count = 1;
        while end < input.len() {
            match input[end] {
                b'{' => brace_count += 1,
                b'}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        break;
                    }
                }
                _ => {}
            }

            end += 1;
        }

        if brace_count != 0 {
            return Err(TemplateError::UnbalancedBrace {
                template: String::from_utf8_lossy(input).to_string(),
                position: cursor,
            });
        }

        let content = &input[start..end];
        if content.is_empty() {
            return Err(TemplateError::EmptyBraces {
                template: String::from_utf8_lossy(input).to_string(),
                position: cursor,
            });
        }

        let (name, constraint) = content
            .iter()
            .position(|&c| c == b':')
            .map_or((content, None), |colon_pos| {
                (&content[..colon_pos], Some(&content[colon_pos + 1..]))
            });

        if name.is_empty() {
            return Err(TemplateError::EmptyParameter {
                template: String::from_utf8_lossy(input).to_string(),
                start: cursor,
                length: end - cursor + 1,
            });
        }

        let is_wildcard = name.starts_with(b"*");
        let name = if is_wildcard { &name[1..] } else { name };

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

        if let Some(constraint) = constraint {
            if constraint.is_empty() {
                return Err(TemplateError::EmptyConstraint {
                    template: String::from_utf8_lossy(input).to_string(),
                    start: cursor,
                    length: end - cursor + 1,
                });
            }

            if constraint.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
                return Err(TemplateError::InvalidConstraint {
                    template: String::from_utf8_lossy(input).to_string(),
                    name: String::from_utf8_lossy(constraint).to_string(),
                    start: cursor,
                    length: end - cursor + 1,
                });
            }
        }

        let name =
            String::from_utf8(name.to_vec()).map_err(|_| TemplateError::InvalidParameter {
                template: String::from_utf8_lossy(input).to_string(),
                name: String::from_utf8_lossy(name).to_string(),
                start: cursor,
                length: end - cursor + 1,
            })?;

        let constraint = if let Some(constraint) = constraint {
            Some(String::from_utf8(constraint.to_vec()).map_err(|_| {
                TemplateError::InvalidConstraint {
                    template: String::from_utf8_lossy(input).to_string(),
                    name: String::from_utf8_lossy(constraint).to_string(),
                    start: cursor,
                    length: end - cursor + 1,
                }
            })?)
        } else {
            None
        };

        let part = match (is_wildcard, constraint) {
            (true, Some(constraint)) => Part::WildcardConstrained { name, constraint },
            (true, None) => Part::Wildcard { name },
            (false, Some(constraint)) => Part::DynamicConstrained { name, constraint },
            (false, None) => Part::Dynamic { name },
        };

        Ok((part, end + 1))
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
            ParsedTemplate::new(b"/abcd"),
            Ok(ParsedTemplate {
                input: b"/abcd".to_vec(),
                templates: vec![Template {
                    input: b"/abcd".to_vec(),
                    raw: b"/abcd".to_vec(),
                    parts: vec![Part::Static {
                        prefix: b"/abcd".to_vec()
                    }],
                }],
                expanded: false,
            }),
        );
    }

    #[test]
    fn test_parser_dynamic_route() {
        assert_eq!(
            ParsedTemplate::new(b"/{name}"),
            Ok(ParsedTemplate {
                input: b"/{name}".to_vec(),
                templates: vec![Template {
                    input: b"/{name}".to_vec(),
                    raw: b"/{name}".to_vec(),
                    parts: vec![
                        Part::Dynamic {
                            name: "name".to_owned(),
                        },
                        Part::Static {
                            prefix: b"/".to_vec()
                        },
                    ],
                }],
                expanded: false,
            }),
        );
    }

    #[test]
    fn test_parser_wildcard_route() {
        assert_eq!(
            ParsedTemplate::new(b"/{*wildcard}"),
            Ok(ParsedTemplate {
                input: b"/{*wildcard}".to_vec(),
                templates: vec![Template {
                    input: b"/{*wildcard}".to_vec(),
                    raw: b"/{*wildcard}".to_vec(),
                    parts: vec![
                        Part::Wildcard {
                            name: "wildcard".to_owned(),
                        },
                        Part::Static {
                            prefix: b"/".to_vec()
                        },
                    ],
                }],
                expanded: false,
            }),
        );
    }

    #[test]
    fn test_parser_complex_route() {
        assert_eq!(
            ParsedTemplate::new(b"/{*name:alpha}/{id:numeric}"),
            Ok(ParsedTemplate {
                input: b"/{*name:alpha}/{id:numeric}".to_vec(),
                templates: vec![Template {
                    input: b"/{*name:alpha}/{id:numeric}".to_vec(),
                    raw: b"/{*name:alpha}/{id:numeric}".to_vec(),
                    parts: vec![
                        Part::DynamicConstrained {
                            name: "id".to_owned(),
                            constraint: "numeric".to_owned()
                        },
                        Part::Static {
                            prefix: b"/".to_vec()
                        },
                        Part::WildcardConstrained {
                            name: "name".to_owned(),
                            constraint: "alpha".to_owned()
                        },
                        Part::Static {
                            prefix: b"/".to_vec()
                        },
                    ],
                }],
                expanded: false,
            }),
        );
    }

    #[test]
    fn test_parser_optional_group_simple() {
        assert_eq!(
            ParsedTemplate::new(b"/users(/{id})"),
            Ok(ParsedTemplate {
                input: b"/users(/{id})".to_vec(),
                templates: vec![
                    Template {
                        input: b"/users(/{id})".to_vec(),
                        raw: b"/users/{id}".to_vec(),
                        parts: vec![
                            Part::Dynamic {
                                name: "id".to_owned(),
                            },
                            Part::Static {
                                prefix: b"/users/".to_vec()
                            },
                        ],
                    },
                    Template {
                        input: b"/users(/{id})".to_vec(),
                        raw: b"/users".to_vec(),
                        parts: vec![Part::Static {
                            prefix: b"/users".to_vec()
                        }],
                    },
                ],
                expanded: true,
            }),
        );
    }

    #[test]
    fn test_parser_optional_groups_nested() {
        assert_eq!(
            ParsedTemplate::new(b"/users(/{id}(/profile))"),
            Ok(ParsedTemplate {
                input: b"/users(/{id}(/profile))".to_vec(),
                templates: vec![
                    Template {
                        input: b"/users(/{id}(/profile))".to_vec(),
                        raw: b"/users/{id}/profile".to_vec(),
                        parts: vec![
                            Part::Static {
                                prefix: b"/profile".to_vec()
                            },
                            Part::Dynamic {
                                name: "id".to_owned(),
                            },
                            Part::Static {
                                prefix: b"/users/".to_vec()
                            },
                        ],
                    },
                    Template {
                        input: b"/users(/{id}(/profile))".to_vec(),
                        raw: b"/users/{id}".to_vec(),
                        parts: vec![
                            Part::Dynamic {
                                name: "id".to_owned(),
                            },
                            Part::Static {
                                prefix: b"/users/".to_vec()
                            },
                        ],
                    },
                    Template {
                        input: b"/users(/{id}(/profile))".to_vec(),
                        raw: b"/users".to_vec(),
                        parts: vec![Part::Static {
                            prefix: b"/users".to_vec()
                        }],
                    },
                ],
                expanded: true,
            }),
        );
    }

    #[test]
    fn test_parser_escaped_characters() {
        assert_eq!(
            ParsedTemplate::new(b"/path/with\\{braces\\}and\\(parens\\)"),
            Ok(ParsedTemplate {
                input: b"/path/with\\{braces\\}and\\(parens\\)".to_vec(),
                templates: vec![Template {
                    input: b"/path/with\\{braces\\}and\\(parens\\)".to_vec(),
                    raw: b"/path/with\\{braces\\}and\\(parens\\)".to_vec(),
                    parts: vec![Part::Static {
                        prefix: b"/path/with{braces}and(parens)".to_vec()
                    }],
                }],
                expanded: false,
            }),
        );
    }

    #[test]
    fn test_parser_edge_case_starting_optional_group() {
        assert_eq!(
            ParsedTemplate::new(b"(/{lang})/users"),
            Ok(ParsedTemplate {
                input: b"(/{lang})/users".to_vec(),
                templates: vec![
                    Template {
                        input: b"(/{lang})/users".to_vec(),
                        raw: b"/{lang}/users".to_vec(),
                        parts: vec![
                            Part::Static {
                                prefix: b"/users".to_vec()
                            },
                            Part::Dynamic {
                                name: "lang".to_owned(),
                            },
                            Part::Static {
                                prefix: b"/".to_vec()
                            },
                        ],
                    },
                    Template {
                        input: b"(/{lang})/users".to_vec(),
                        raw: b"/users".to_vec(),
                        parts: vec![Part::Static {
                            prefix: b"/users".to_vec()
                        }],
                    },
                ],
                expanded: true,
            }),
        );
    }

    #[test]
    fn test_parser_edge_case_only_optional_groups() {
        assert_eq!(
            ParsedTemplate::new(b"(/{lang})(/{page})"),
            Ok(ParsedTemplate {
                input: b"(/{lang})(/{page})".to_vec(),
                templates: vec![
                    Template {
                        input: b"(/{lang})(/{page})".to_vec(),
                        raw: b"/{lang}/{page}".to_vec(),
                        parts: vec![
                            Part::Dynamic {
                                name: "page".to_owned(),
                            },
                            Part::Static {
                                prefix: b"/".to_vec()
                            },
                            Part::Dynamic {
                                name: "lang".to_owned(),
                            },
                            Part::Static {
                                prefix: b"/".to_vec()
                            },
                        ],
                    },
                    Template {
                        input: b"(/{lang})(/{page})".to_vec(),
                        raw: b"/{lang}".to_vec(),
                        parts: vec![
                            Part::Dynamic {
                                name: "lang".to_owned(),
                            },
                            Part::Static {
                                prefix: b"/".to_vec()
                            },
                        ],
                    },
                    Template {
                        input: b"(/{lang})(/{page})".to_vec(),
                        raw: b"/{page}".to_vec(),
                        parts: vec![
                            Part::Dynamic {
                                name: "page".to_owned(),
                            },
                            Part::Static {
                                prefix: b"/".to_vec()
                            },
                        ],
                    },
                    Template {
                        input: b"(/{lang})(/{page})".to_vec(),
                        raw: b"/".to_vec(),
                        parts: vec![Part::Static {
                            prefix: b"/".to_vec()
                        }],
                    },
                ],
                expanded: true,
            }),
        );
    }

    #[test]
    fn test_parser_error_empty() {
        let error = ParsedTemplate::new(b"").unwrap_err();
        assert_eq!(error, TemplateError::Empty);

        insta::assert_snapshot!(error, @"empty template");
    }

    #[test]
    fn test_parser_error_empty_braces() {
        let error = ParsedTemplate::new(b"/users/{}").unwrap_err();
        assert_eq!(
            error,
            TemplateError::EmptyBraces {
                template: "/users/{}".to_owned(),
                position: 7,
            }
        );

        insta::assert_snapshot!(error, @r"
        empty braces

            Template: /users/{}
                             ^^
        ");
    }

    #[test]
    fn test_parser_error_missing_leading_slash() {
        let error = ParsedTemplate::new(b"abc").unwrap_err();
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
    fn test_parser_error_unbalanced_brace_opening() {
        let error = ParsedTemplate::new(b"/users/{id/profile").unwrap_err();
        assert_eq!(
            error,
            TemplateError::UnbalancedBrace {
                template: "/users/{id/profile".to_owned(),
                position: 7,
            }
        );

        insta::assert_snapshot!(error, @r"
        unbalanced brace

            Template: /users/{id/profile
                             ^

        help: Each '{' must have a matching '}'

        try:
            - Add the missing closing brace
            - Use '\{' and '\}' to represent literal braces
        ");
    }

    #[test]
    fn test_parser_error_unbalanced_brace_closing() {
        let error = ParsedTemplate::new(b"/users/id}/profile").unwrap_err();
        assert_eq!(
            error,
            TemplateError::UnbalancedBrace {
                template: "/users/id}/profile".to_owned(),
                position: 9,
            }
        );

        insta::assert_snapshot!(error, @r"
        unbalanced brace

            Template: /users/id}/profile
                               ^

        help: Each '{' must have a matching '}'

        try:
            - Add the missing closing brace
            - Use '\{' and '\}' to represent literal braces
        ");
    }

    #[test]
    fn test_parser_error_empty_parenthesis() {
        let error = ParsedTemplate::new(b"/products()/category").unwrap_err();
        assert_eq!(
            error,
            TemplateError::EmptyParentheses {
                template: "/products()/category".to_owned(),
                position: 9,
            }
        );

        insta::assert_snapshot!(error, @r"
        empty parentheses

            Template: /products()/category
                               ^^
        ");
    }

    #[test]
    fn test_parser_error_unbalanced_parenthesis_opening() {
        let error = ParsedTemplate::new(b"/products(/category").unwrap_err();
        assert_eq!(
            error,
            TemplateError::UnbalancedParenthesis {
                template: "/products(/category".to_owned(),
                position: 9,
            }
        );

        insta::assert_snapshot!(error, @r"
        unbalanced parenthesis

            Template: /products(/category
                               ^

        help: Each '(' must have a matching ')'

        try:
            - Add the missing closing parenthesis
            - Use '\(' and '\)' to represent literal parentheses
        ");
    }

    #[test]
    fn test_parser_error_unbalanced_parenthesis_closing() {
        let error = ParsedTemplate::new(b"/products)/category").unwrap_err();
        assert_eq!(
            error,
            TemplateError::UnbalancedParenthesis {
                template: "/products)/category".to_owned(),
                position: 9,
            }
        );

        insta::assert_snapshot!(error, @r"
        unbalanced parenthesis

            Template: /products)/category
                               ^

        help: Each '(' must have a matching ')'

        try:
            - Add the missing closing parenthesis
            - Use '\(' and '\)' to represent literal parentheses
        ");
    }

    #[test]
    fn test_parser_error_empty_parameter() {
        let error = ParsedTemplate::new(b"/users/{:constraint}/profile").unwrap_err();
        assert_eq!(
            error,
            TemplateError::EmptyParameter {
                template: "/users/{:constraint}/profile".to_owned(),
                start: 7,
                length: 13,
            }
        );

        insta::assert_snapshot!(error, @r"
        empty parameter name

            Template: /users/{:constraint}/profile
                             ^^^^^^^^^^^^^
        ");
    }

    #[test]
    fn test_parser_error_invalid_parameter() {
        let error = ParsedTemplate::new(b"/users/{user*name}/profile").unwrap_err();
        assert_eq!(
            error,
            TemplateError::InvalidParameter {
                template: "/users/{user*name}/profile".to_owned(),
                name: "user*name".to_owned(),
                start: 7,
                length: 11,
            }
        );

        insta::assert_snapshot!(error, @r"
        invalid parameter name: 'user*name'

            Template: /users/{user*name}/profile
                             ^^^^^^^^^^^

        help: Parameter names must not contain the characters: ':', '*', '{', '}', '(', ')', '/'
        ");
    }

    #[test]
    fn test_parser_error_duplicate_parameter() {
        let error = ParsedTemplate::new(b"/users/{id}/posts/{id:uuid}").unwrap_err();
        assert_eq!(
            error,
            TemplateError::DuplicateParameter {
                template: "/users/{id}/posts/{id:uuid}".to_owned(),
                name: "id".to_owned(),
                first: 7,
                first_length: 4,
                second: 18,
                second_length: 9,
            }
        );

        insta::assert_snapshot!(error, @r"
        duplicate parameter name: 'id'

            Template: /users/{id}/posts/{id:uuid}
                             ^^^^       ^^^^^^^^^

        help: Parameter names must be unique within a template

        try:
            - Rename one of the parameters to be unique
        ");
    }

    #[test]
    fn test_parser_error_empty_wildcard() {
        let error = ParsedTemplate::new(b"/files/{*}").unwrap_err();
        assert_eq!(
            error,
            TemplateError::EmptyWildcard {
                template: "/files/{*}".to_owned(),
                start: 7,
                length: 3,
            }
        );

        insta::assert_snapshot!(error, @r"
        empty wildcard name

            Template: /files/{*}
                             ^^^
        ");
    }

    #[test]
    fn test_parser_error_empty_constraint() {
        let error = ParsedTemplate::new(b"/users/{id:}/profile").unwrap_err();
        assert_eq!(
            error,
            TemplateError::EmptyConstraint {
                template: "/users/{id:}/profile".to_owned(),
                start: 7,
                length: 5,
            }
        );

        insta::assert_snapshot!(error, @r"
        empty constraint name

            Template: /users/{id:}/profile
                             ^^^^^
        ");
    }

    #[test]
    fn test_parser_error_invalid_constraint() {
        let error = ParsedTemplate::new(b"/users/{id:*}/profile").unwrap_err();
        assert_eq!(
            error,
            TemplateError::InvalidConstraint {
                template: "/users/{id:*}/profile".to_owned(),
                name: "*".to_owned(),
                start: 7,
                length: 6,
            }
        );

        insta::assert_snapshot!(error, @r"
        invalid constraint name: '*'

            Template: /users/{id:*}/profile
                             ^^^^^^

        help: Constraint names must not contain the characters: ':', '*', '{', '}', '(', ')', '/'
        ");
    }

    #[test]
    fn test_parser_error_touching_parameters() {
        let error = ParsedTemplate::new(b"/users/{id}{*name}").unwrap_err();
        assert_eq!(
            error,
            TemplateError::TouchingParameters {
                template: "/users/{id}{*name}".to_owned(),
                start: 7,
                length: 11,
            }
        );

        insta::assert_snapshot!(error, @r"
        touching parameters

            Template: /users/{id}{*name}
                             ^^^^^^^^^^^

        help: Parameters must be separated by at least one part

        try:
            - Add a part between the parameters
            - Combine the parameters if they represent a single value
        ");
    }
}
