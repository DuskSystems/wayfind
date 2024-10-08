use crate::errors::RouteError;
use std::collections::HashMap;

/// Characters that are not allowed in parameter names or constraints.
const INVALID_PARAM_CHARS: [u8; 7] = [b':', b'*', b'{', b'}', b'(', b')', b'/'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
    pub raw: Vec<u8>,
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part {
    Static {
        prefix: Vec<u8>,
    },

    Dynamic {
        name: Vec<u8>,
        constraint: Option<Vec<u8>>,
    },

    Wildcard {
        name: Vec<u8>,
        constraint: Option<Vec<u8>>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parser {
    pub raw: Vec<u8>,
    pub routes: Vec<Route>,
}

impl Parser {
    pub fn new(input: &[u8]) -> Result<Self, RouteError> {
        if input.is_empty() {
            return Err(RouteError::Empty);
        }

        let routes = Self::expand_optional_groups(input, 0, input.len())?;
        let routes = routes
            .into_iter()
            .map(|raw| Self::parse_route(&raw))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            raw: input.to_vec(),
            routes,
        })
    }

    // Recursively expands optional groups in the route, generating all possible combinations
    fn expand_optional_groups(
        input: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Vec<Vec<u8>>, RouteError> {
        let mut result = vec![vec![]];

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
                        result
                            .iter_mut()
                            .for_each(|route| route.extend_from_slice(&input[group..cursor]));

                        group = cursor + 1;
                    }

                    depth += 1;
                    cursor += 1;
                }
                (b')', _) => {
                    depth -= 1;

                    if depth < 0 {
                        return Err(RouteError::UnbalancedParenthesis {
                            route: String::from_utf8_lossy(input).to_string(),
                            position: cursor,
                        });
                    }

                    if depth == 0 {
                        if cursor == group {
                            return Err(RouteError::EmptyParentheses {
                                route: String::from_utf8_lossy(input).to_string(),
                                position: cursor - 1,
                            });
                        }

                        let optional_groups = Self::expand_optional_groups(input, group, cursor)?;

                        let mut new_result = vec![];
                        for route in result {
                            for optional_group in &optional_groups {
                                let mut new_route = route.clone();
                                new_route.extend_from_slice(optional_group);
                                new_result.push(new_route);
                            }

                            new_result.push(route);
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
            return Err(RouteError::UnbalancedParenthesis {
                route: String::from_utf8_lossy(input).to_string(),
                position: start + group - 1,
            });
        }

        if group < end {
            result
                .iter_mut()
                .for_each(|route| route.extend_from_slice(&input[group..end]));
        }

        Ok(result)
    }

    fn parse_route(input: &[u8]) -> Result<Route, RouteError> {
        let mut parts = vec![];
        let mut cursor = 0;

        let mut seen_parameters: HashMap<Vec<u8>, (usize, usize)> = HashMap::new();

        while cursor < input.len() {
            match input[cursor] {
                b'{' => {
                    let (part, next_cursor) = Self::parse_parameter_part(input, cursor)?;

                    if let Part::Dynamic { name, .. } | Part::Wildcard { name, .. } = &part {
                        if let Some(&(first, first_length)) = seen_parameters.get(name) {
                            return Err(RouteError::DuplicateParameter {
                                route: String::from_utf8_lossy(input).to_string(),
                                name: String::from_utf8_lossy(name).to_string(),
                                first,
                                first_length,
                                second: cursor,
                                second_length: next_cursor - cursor,
                            });
                        }

                        seen_parameters.insert(name.clone(), (cursor, next_cursor - cursor));
                    }

                    parts.push(part);
                    cursor = next_cursor;
                }
                b'}' => {
                    return Err(RouteError::UnbalancedBrace {
                        route: String::from_utf8_lossy(input).to_string(),
                        position: cursor,
                    })
                }
                _ => {
                    let (part, next_cursor) = Self::parse_static_part(input, cursor);
                    parts.push(part);
                    cursor = next_cursor;
                }
            }
        }

        parts.reverse();

        Ok(Route {
            raw: input.to_vec(),
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

    fn parse_parameter_part(input: &[u8], cursor: usize) -> Result<(Part, usize), RouteError> {
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
            return Err(RouteError::UnbalancedBrace {
                route: String::from_utf8_lossy(input).to_string(),
                position: cursor,
            });
        }

        let content = &input[start..end];
        if content.is_empty() {
            return Err(RouteError::EmptyBraces {
                route: String::from_utf8_lossy(input).to_string(),
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
            return Err(RouteError::EmptyParameter {
                route: String::from_utf8_lossy(input).to_string(),
                start: cursor,
                length: end - cursor + 1,
            });
        }

        let is_wildcard = name.starts_with(b"*");
        let name = if is_wildcard { &name[1..] } else { name };

        if is_wildcard && name.is_empty() {
            return Err(RouteError::EmptyWildcard {
                route: String::from_utf8_lossy(input).to_string(),
                start: cursor,
                length: end - cursor + 1,
            });
        }

        if name.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
            return Err(RouteError::InvalidParameter {
                route: String::from_utf8_lossy(input).to_string(),
                name: String::from_utf8_lossy(name).to_string(),
                start: cursor,
                length: end - cursor + 1,
            });
        }

        if let Some(constraint) = constraint {
            if constraint.is_empty() {
                return Err(RouteError::EmptyConstraint {
                    route: String::from_utf8_lossy(input).to_string(),
                    start: cursor,
                    length: end - cursor + 1,
                });
            }

            if constraint.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
                return Err(RouteError::InvalidConstraint {
                    route: String::from_utf8_lossy(input).to_string(),
                    name: String::from_utf8_lossy(name).to_string(),
                    start: cursor,
                    length: end - cursor + 1,
                });
            }
        }

        let name = name.to_vec();
        let constraint = constraint.map(<[u8]>::to_vec);

        let part = if is_wildcard {
            Part::Wildcard { name, constraint }
        } else {
            Part::Dynamic { name, constraint }
        };

        Ok((part, end + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use similar_asserts::assert_eq;

    #[test]
    fn test_parser_static_route() {
        assert_eq!(
            Parser::new(b"/abcd"),
            Ok(Parser {
                raw: b"/abcd".to_vec(),
                routes: vec![Route {
                    raw: b"/abcd".to_vec(),
                    parts: vec![Part::Static {
                        prefix: b"/abcd".to_vec()
                    }],
                }],
            }),
        );
    }

    #[test]
    fn test_parser_dynamic_route() {
        assert_eq!(
            Parser::new(b"/{name}"),
            Ok(Parser {
                raw: b"/{name}".to_vec(),
                routes: vec![Route {
                    raw: b"/{name}".to_vec(),
                    parts: vec![
                        Part::Dynamic {
                            name: b"name".to_vec(),
                            constraint: None
                        },
                        Part::Static {
                            prefix: b"/".to_vec()
                        },
                    ],
                }],
            }),
        );
    }

    #[test]
    fn test_parser_wildcard_route() {
        assert_eq!(
            Parser::new(b"/{*route}"),
            Ok(Parser {
                raw: b"/{*route}".to_vec(),
                routes: vec![Route {
                    raw: b"/{*route}".to_vec(),
                    parts: vec![
                        Part::Wildcard {
                            name: b"route".to_vec(),
                            constraint: None
                        },
                        Part::Static {
                            prefix: b"/".to_vec()
                        },
                    ],
                }],
            }),
        );
    }

    #[test]
    fn test_parser_complex_route() {
        assert_eq!(
            Parser::new(b"/{*name:alpha}/{id:numeric}"),
            Ok(Parser {
                raw: b"/{*name:alpha}/{id:numeric}".to_vec(),
                routes: vec![Route {
                    raw: b"/{*name:alpha}/{id:numeric}".to_vec(),
                    parts: vec![
                        Part::Dynamic {
                            name: b"id".to_vec(),
                            constraint: Some(b"numeric".to_vec())
                        },
                        Part::Static {
                            prefix: b"/".to_vec()
                        },
                        Part::Wildcard {
                            name: b"name".to_vec(),
                            constraint: Some(b"alpha".to_vec())
                        },
                        Part::Static {
                            prefix: b"/".to_vec()
                        },
                    ],
                }],
            }),
        );
    }

    #[test]
    fn test_parser_optional_group_simple() {
        assert_eq!(
            Parser::new(b"/users(/{id})"),
            Ok(Parser {
                raw: b"/users(/{id})".to_vec(),
                routes: vec![
                    Route {
                        raw: b"/users/{id}".to_vec(),
                        parts: vec![
                            Part::Dynamic {
                                name: b"id".to_vec(),
                                constraint: None
                            },
                            Part::Static {
                                prefix: b"/users/".to_vec()
                            },
                        ],
                    },
                    Route {
                        raw: b"/users".to_vec(),
                        parts: vec![Part::Static {
                            prefix: b"/users".to_vec()
                        }],
                    },
                ],
            }),
        );
    }

    #[test]
    fn test_parser_optional_groups_nested() {
        assert_eq!(
            Parser::new(b"/users(/{id}(/profile))"),
            Ok(Parser {
                raw: b"/users(/{id}(/profile))".to_vec(),
                routes: vec![
                    Route {
                        raw: b"/users/{id}/profile".to_vec(),
                        parts: vec![
                            Part::Static {
                                prefix: b"/profile".to_vec()
                            },
                            Part::Dynamic {
                                name: b"id".to_vec(),
                                constraint: None
                            },
                            Part::Static {
                                prefix: b"/users/".to_vec()
                            },
                        ],
                    },
                    Route {
                        raw: b"/users/{id}".to_vec(),
                        parts: vec![
                            Part::Dynamic {
                                name: b"id".to_vec(),
                                constraint: None
                            },
                            Part::Static {
                                prefix: b"/users/".to_vec()
                            },
                        ],
                    },
                    Route {
                        raw: b"/users".to_vec(),
                        parts: vec![Part::Static {
                            prefix: b"/users".to_vec()
                        }],
                    },
                ],
            }),
        );
    }

    #[test]
    fn test_parser_escaped_characters() {
        assert_eq!(
            Parser::new(b"/path/with\\{braces\\}and\\(parens\\)"),
            Ok(Parser {
                raw: b"/path/with\\{braces\\}and\\(parens\\)".to_vec(),
                routes: vec![Route {
                    raw: b"/path/with\\{braces\\}and\\(parens\\)".to_vec(),
                    parts: vec![Part::Static {
                        prefix: b"/path/with{braces}and(parens)".to_vec()
                    }],
                }],
            }),
        );
    }

    #[test]
    fn test_parser_edge_case_starting_optional_group() {
        assert_eq!(
            Parser::new(b"(/{lang})/users"),
            Ok(Parser {
                raw: b"(/{lang})/users".to_vec(),
                routes: vec![
                    Route {
                        raw: b"/{lang}/users".to_vec(),
                        parts: vec![
                            Part::Static {
                                prefix: b"/users".to_vec()
                            },
                            Part::Dynamic {
                                name: b"lang".to_vec(),
                                constraint: None
                            },
                            Part::Static {
                                prefix: b"/".to_vec()
                            },
                        ],
                    },
                    Route {
                        raw: b"/users".to_vec(),
                        parts: vec![Part::Static {
                            prefix: b"/users".to_vec()
                        }],
                    },
                ],
            }),
        );
    }

    #[test]
    fn test_parser_edge_case_only_optional_groups() {
        assert_eq!(
            Parser::new(b"(/{lang})(/{page})"),
            Ok(Parser {
                raw: b"(/{lang})(/{page})".to_vec(),
                routes: vec![
                    Route {
                        raw: b"/{lang}/{page}".to_vec(),
                        parts: vec![
                            Part::Dynamic {
                                name: b"page".to_vec(),
                                constraint: None
                            },
                            Part::Static {
                                prefix: b"/".to_vec()
                            },
                            Part::Dynamic {
                                name: b"lang".to_vec(),
                                constraint: None
                            },
                            Part::Static {
                                prefix: b"/".to_vec()
                            },
                        ],
                    },
                    Route {
                        raw: b"/{lang}".to_vec(),
                        parts: vec![
                            Part::Dynamic {
                                name: b"lang".to_vec(),
                                constraint: None
                            },
                            Part::Static {
                                prefix: b"/".to_vec()
                            },
                        ],
                    },
                    Route {
                        raw: b"/{page}".to_vec(),
                        parts: vec![
                            Part::Dynamic {
                                name: b"page".to_vec(),
                                constraint: None
                            },
                            Part::Static {
                                prefix: b"/".to_vec()
                            },
                        ],
                    },
                    Route {
                        raw: b"".to_vec(),
                        parts: vec![],
                    },
                ],
            }),
        );
    }

    #[test]
    fn test_parser_error_empty() {
        let error = Parser::new(b"").unwrap_err();
        assert_eq!(error, RouteError::Empty);

        insta::assert_snapshot!(error, @"empty route");
    }

    #[test]
    fn test_parser_error_empty_braces() {
        let error = Parser::new(b"/users/{}").unwrap_err();
        assert_eq!(
            error,
            RouteError::EmptyBraces {
                route: "/users/{}".to_string(),
                position: 7,
            }
        );

        insta::assert_snapshot!(error, @r#"
        empty braces

            Route: /users/{}
                          ^^
        "#);
    }

    #[test]
    fn test_parser_error_unbalanced_brace_opening() {
        let error = Parser::new(b"/users/{id/profile").unwrap_err();
        assert_eq!(
            error,
            RouteError::UnbalancedBrace {
                route: "/users/{id/profile".to_string(),
                position: 7,
            }
        );

        insta::assert_snapshot!(error, @r#"
        unbalanced brace

            Route: /users/{id/profile
                          ^

        tip: Use '\{' and '\}' to represent literal '{' and '}' characters in the route
        "#);
    }

    #[test]
    fn test_parser_error_unbalanced_brace_closing() {
        let error = Parser::new(b"/users/id}/profile").unwrap_err();
        assert_eq!(
            error,
            RouteError::UnbalancedBrace {
                route: "/users/id}/profile".to_string(),
                position: 9,
            }
        );

        insta::assert_snapshot!(error, @r#"
        unbalanced brace

            Route: /users/id}/profile
                            ^

        tip: Use '\{' and '\}' to represent literal '{' and '}' characters in the route
        "#);
    }

    #[test]
    fn test_parser_error_empty_parenthesis() {
        let error = Parser::new(b"/products()/category").unwrap_err();
        assert_eq!(
            error,
            RouteError::EmptyParentheses {
                route: "/products()/category".to_string(),
                position: 9,
            }
        );

        insta::assert_snapshot!(error, @r#"
        empty parentheses

            Route: /products()/category
                            ^^
        "#);
    }

    #[test]
    fn test_parser_error_unbalanced_parenthesis_opening() {
        let error = Parser::new(b"/products(/category").unwrap_err();
        assert_eq!(
            error,
            RouteError::UnbalancedParenthesis {
                route: "/products(/category".to_string(),
                position: 9,
            }
        );

        insta::assert_snapshot!(error, @r#"
        unbalanced parenthesis

            Route: /products(/category
                            ^

        tip: Use '\(' and '\)' to represent literal '(' and ')' characters in the route
        "#);
    }

    #[test]
    fn test_parser_error_unbalanced_parenthesis_closing() {
        let error = Parser::new(b"/products)/category").unwrap_err();
        assert_eq!(
            error,
            RouteError::UnbalancedParenthesis {
                route: "/products)/category".to_string(),
                position: 9,
            }
        );

        insta::assert_snapshot!(error, @r#"
        unbalanced parenthesis

            Route: /products)/category
                            ^

        tip: Use '\(' and '\)' to represent literal '(' and ')' characters in the route
        "#);
    }

    #[test]
    fn test_parser_error_empty_parameter() {
        let error = Parser::new(b"/users/{:constraint}/profile").unwrap_err();
        assert_eq!(
            error,
            RouteError::EmptyParameter {
                route: "/users/{:constraint}/profile".to_string(),
                start: 7,
                length: 13,
            }
        );

        insta::assert_snapshot!(error, @r#"
        empty parameter name

            Route: /users/{:constraint}/profile
                          ^^^^^^^^^^^^^
        "#);
    }

    #[test]
    fn test_parser_error_invalid_parameter() {
        let error = Parser::new(b"/users/{user*name}/profile").unwrap_err();
        assert_eq!(
            error,
            RouteError::InvalidParameter {
                route: "/users/{user*name}/profile".to_string(),
                name: "user*name".to_string(),
                start: 7,
                length: 11,
            }
        );

        insta::assert_snapshot!(error, @r#"
        invalid parameter name

            Route: /users/{user*name}/profile
                          ^^^^^^^^^^^

        tip: Parameter names must not contain the characters: ':', '*', '{', '}', '(', ')', '/'
        "#);
    }

    #[test]
    fn test_parser_error_duplicate_parameter() {
        let error = Parser::new(b"/users/{id}/posts/{id:uuid}").unwrap_err();
        assert_eq!(
            error,
            RouteError::DuplicateParameter {
                route: "/users/{id}/posts/{id:uuid}".to_string(),
                name: "id".to_string(),
                first: 7,
                first_length: 4,
                second: 18,
                second_length: 9,
            }
        );

        insta::assert_snapshot!(error, @r#"
        duplicate parameter name: 'id'

            Route: /users/{id}/posts/{id:uuid}
                          ^^^^       ^^^^^^^^^

        tip: Parameter names must be unique within a route
        "#);
    }

    #[test]
    fn test_parser_error_empty_wildcard() {
        let error = Parser::new(b"/files/{*}").unwrap_err();
        assert_eq!(
            error,
            RouteError::EmptyWildcard {
                route: "/files/{*}".to_string(),
                start: 7,
                length: 3,
            }
        );

        insta::assert_snapshot!(error, @r#"
        empty wildcard name

            Route: /files/{*}
                          ^^^
        "#);
    }

    #[test]
    fn test_parser_error_empty_constraint() {
        let error = Parser::new(b"/users/{id:}/profile").unwrap_err();
        assert_eq!(
            error,
            RouteError::EmptyConstraint {
                route: "/users/{id:}/profile".to_string(),
                start: 7,
                length: 5,
            }
        );

        insta::assert_snapshot!(error, @r#"
        empty constraint name

            Route: /users/{id:}/profile
                          ^^^^^
        "#);
    }

    #[test]
    fn test_parser_error_invalid_constraint() {
        let error = Parser::new(b"/users/{id:*}/profile").unwrap_err();
        assert_eq!(
            error,
            RouteError::InvalidConstraint {
                route: "/users/{id:*}/profile".to_string(),
                name: "id".to_string(),
                start: 7,
                length: 6,
            }
        );

        insta::assert_snapshot!(error, @r#"
        invalid constraint name

            Route: /users/{id:*}/profile
                          ^^^^^^

        tip: Constraint names must not contain the characters: ':', '*', '{', '}', '(', ')', '/'
        "#);
    }
}
