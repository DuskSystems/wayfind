use crate::errors::RouteError;
use rustc_hash::FxHashMap;

/// Characters that are not allowed in parameter names.
const INVALID_PARAM_CHARS: [u8; 7] = [b':', b'*', b'{', b'}', b'(', b')', b'/'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route<'a> {
    pub raw: &'a [u8],
    pub parts: Vec<Part<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part<'a> {
    Static { prefix: &'a [u8] },
    Dynamic { name: &'a [u8] },
    Wildcard { name: &'a [u8] },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parser<'a> {
    pub raw: &'a [u8],
    pub route: Route<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [u8]) -> Result<Self, RouteError> {
        if input.is_empty() {
            return Err(RouteError::Empty);
        }

        Ok(Self {
            raw: input,
            route: Self::parse_route(input)?,
        })
    }

    fn parse_route(input: &'a [u8]) -> Result<Route<'a>, RouteError> {
        if !input.is_empty() && input[0] != b'/' {
            return Err(RouteError::MissingLeadingSlash {
                route: String::from_utf8_lossy(input).to_string(),
            });
        }

        let mut parts = vec![];
        let mut cursor = 0;

        let mut seen_parameters: FxHashMap<&[u8], (usize, usize)> = FxHashMap::default();

        while cursor < input.len() {
            match input[cursor] {
                b'{' => {
                    let (part, next_cursor) = Self::parse_parameter_part(input, cursor)?;

                    if let Part::Dynamic { name } | Part::Wildcard { name } = &part {
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

                        seen_parameters.insert(name, (cursor, next_cursor - cursor));
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

        Ok(Route { raw: input, parts })
    }

    fn parse_static_part(input: &'a [u8], cursor: usize) -> (Part<'a>, usize) {
        let mut end = cursor;
        while end < input.len() {
            match input[end] {
                b'{' | b'}' => break,
                _ => end += 1,
            }
        }

        (
            Part::Static {
                prefix: &input[cursor..end],
            },
            end,
        )
    }

    fn parse_parameter_part(
        input: &'a [u8],
        cursor: usize,
    ) -> Result<(Part<'a>, usize), RouteError> {
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

        let name = &input[start..end];
        if name.is_empty() {
            return Err(RouteError::EmptyBraces {
                route: String::from_utf8_lossy(input).to_string(),
                position: cursor,
            });
        }

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
    use super::*;
    use similar_asserts::assert_eq;

    #[test]
    fn test_parser_static_route() {
        assert_eq!(
            Parser::new(b"/abcd"),
            Ok(Parser {
                raw: b"/abcd",
                route: Route {
                    raw: b"/abcd",
                    parts: vec![Part::Static { prefix: b"/abcd" }],
                },
            }),
        );
    }

    #[test]
    fn test_parser_dynamic_route() {
        assert_eq!(
            Parser::new(b"/{name}"),
            Ok(Parser {
                raw: b"/{name}",
                route: Route {
                    raw: b"/{name}",
                    parts: vec![
                        Part::Dynamic { name: b"name" },
                        Part::Static { prefix: b"/" },
                    ],
                },
            }),
        );
    }

    #[test]
    fn test_parser_wildcard_route() {
        assert_eq!(
            Parser::new(b"/{*route}"),
            Ok(Parser {
                raw: b"/{*route}",
                route: Route {
                    raw: b"/{*route}",
                    parts: vec![
                        Part::Wildcard { name: b"route" },
                        Part::Static { prefix: b"/" },
                    ],
                },
            }),
        );
    }

    #[test]
    fn test_parser_complex_route() {
        assert_eq!(
            Parser::new(b"/{*name}/{id}"),
            Ok(Parser {
                raw: b"/{*name}/{id}",
                route: Route {
                    raw: b"/{*name}/{id}",
                    parts: vec![
                        Part::Dynamic { name: b"id" },
                        Part::Static { prefix: b"/" },
                        Part::Wildcard { name: b"name" },
                        Part::Static { prefix: b"/" },
                    ],
                },
            }),
        );
    }

    #[test]
    fn test_parser_escaped_characters() {
        assert_eq!(
            Parser::new(b"/path/with\\{braces\\}and\\(parens\\)"),
            Ok(Parser {
                raw: b"/path/with\\{braces\\}and\\(parens\\)",
                route: Route {
                    raw: b"/path/with\\{braces\\}and\\(parens\\)",
                    parts: vec![Part::Static {
                        prefix: b"/path/with{braces}and(parens)"
                    }],
                },
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
    fn test_parser_error_missing_leading_slash() {
        let error = Parser::new(b"abc").unwrap_err();
        assert_eq!(
            error,
            RouteError::MissingLeadingSlash {
                route: "abc".to_string(),
            }
        );

        insta::assert_snapshot!(error, @r#"
        missing leading slash

            Route: abc

        tip: Routes must begin with '/'
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
        let error = Parser::new(b"/users/{id}/posts/{id}").unwrap_err();
        assert_eq!(
            error,
            RouteError::DuplicateParameter {
                route: "/users/{id}/posts/{id}".to_string(),
                name: "id".to_string(),
                first: 7,
                first_length: 4,
                second: 18,
                second_length: 4,
            }
        );

        insta::assert_snapshot!(error, @r#"
        duplicate parameter name: 'id'

            Route: /users/{id}/posts/{id}
                          ^^^^       ^^^^

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
}
