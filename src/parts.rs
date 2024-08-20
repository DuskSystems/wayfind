use crate::errors::route::RouteError;
use std::fmt::Debug;

// NOTE: '?' is reserved for potential future use.
const INVALID_PARAM_CHARS: [u8; 6] = [b':', b'*', b'?', b'{', b'}', b'/'];

#[derive(Debug, PartialEq, Eq)]
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
pub struct Parts(pub Vec<Part>);

impl Parts {
    pub fn new(path: &[u8]) -> Result<Self, RouteError> {
        if path.is_empty() {
            return Err(RouteError::EmptyPath);
        }

        let mut parts = vec![];
        let mut cursor = 0;
        let mut current_static = vec![];

        while cursor < path.len() {
            match (path[cursor], path.get(cursor + 1)) {
                (b'{', Some(b'{')) => {
                    current_static.push(b'{');
                    cursor += 2;
                }
                (b'}', Some(b'}')) => {
                    current_static.push(b'}');
                    cursor += 2;
                }
                (b'{', _) => {
                    if !current_static.is_empty() {
                        parts.push(Part::Static {
                            prefix: std::mem::take(&mut current_static),
                        });
                    }

                    cursor = Self::parse_parameter(path, cursor, &mut parts)?;
                }
                (b'}', _) => {
                    return Err(RouteError::UnescapedBrace {
                        path: String::from_utf8_lossy(path).to_string(),
                        position: cursor,
                    })
                }
                (c, _) => {
                    current_static.push(c);
                    cursor += 1;
                }
            }
        }

        if !current_static.is_empty() {
            parts.push(Part::Static {
                prefix: std::mem::take(&mut current_static),
            });
        }

        parts.reverse();
        Ok(Self(parts))
    }

    fn parse_parameter(
        path: &[u8],
        cursor: usize,
        parts: &mut Vec<Part>,
    ) -> Result<usize, RouteError> {
        let start = cursor + 1;
        let end = path[start..]
            .iter()
            .position(|&c| c == b'}')
            .map(|pos| start + pos)
            .ok_or(RouteError::UnescapedBrace {
                path: String::from_utf8_lossy(path).to_string(),
                position: cursor,
            })?;

        if start == end {
            return Err(RouteError::EmptyBraces {
                path: String::from_utf8_lossy(path).to_string(),
                position: cursor,
            });
        }

        let colon = path[start..end].iter().position(|&c| c == b':');
        let (name, constraint) = colon.map_or_else(
            || (&path[start..end], None),
            |pos| (&path[start..start + pos], Some(&path[start + pos + 1..end])),
        );

        let (is_wildcard, name) = if name.starts_with(b"*") {
            (true, &name[1..])
        } else {
            (false, name)
        };

        if name.is_empty() {
            if is_wildcard {
                return Err(RouteError::EmptyWildcard {
                    path: String::from_utf8_lossy(path).to_string(),
                    start: cursor,
                    length: end - cursor + 1,
                });
            }

            return Err(RouteError::EmptyParameter {
                path: String::from_utf8_lossy(path).to_string(),
                start: cursor,
                length: end - start + 2,
            });
        }

        if name.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
            return Err(RouteError::InvalidParameter {
                path: String::from_utf8_lossy(path).to_string(),
                start: start - 1,
                length: end - start + 2,
            });
        }

        if let Some(constraint) = constraint {
            if constraint.is_empty() {
                return Err(RouteError::EmptyConstraint {
                    path: String::from_utf8_lossy(path).to_string(),
                    start: start + name.len() + 1,
                    length: 1,
                });
            }

            if constraint.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
                return Err(RouteError::InvalidConstraint {
                    path: String::from_utf8_lossy(path).to_string(),
                    start: start + name.len() + 1,
                    length: constraint.len(),
                });
            }
        }

        let part = if is_wildcard {
            Part::Wildcard {
                name: name.to_vec(),
                constraint: constraint.map(<[u8]>::to_vec),
            }
        } else {
            Part::Dynamic {
                name: name.to_vec(),
                constraint: constraint.map(<[u8]>::to_vec),
            }
        };

        parts.push(part);
        Ok(end + 1)
    }

    pub fn pop(&mut self) -> Option<Part> {
        self.0.pop()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts_static() {
        assert_eq!(
            Parts::new(b"/abcd"),
            Ok(Parts(vec![Part::Static {
                prefix: b"/abcd".to_vec()
            }])),
        );
    }

    #[test]
    fn test_parts_dynamic() {
        assert_eq!(
            Parts::new(b"/{name}"),
            Ok(Parts(vec![
                Part::Dynamic {
                    name: b"name".to_vec(),
                    constraint: None
                },
                Part::Static {
                    prefix: b"/".to_vec()
                },
            ]))
        );
    }

    #[test]
    fn test_parts_wildcard() {
        assert_eq!(
            Parts::new(b"/{*path}"),
            Ok(Parts(vec![
                Part::Wildcard {
                    name: b"path".to_vec(),
                    constraint: None
                },
                Part::Static {
                    prefix: b"/".to_vec()
                },
            ]))
        );
    }

    #[test]
    fn test_parts_constraint() {
        assert_eq!(
            Parts::new(b"/{name:alpha}/{id:numeric}"),
            Ok(Parts(vec![
                Part::Dynamic {
                    name: b"id".to_vec(),
                    constraint: Some("numeric".into())
                },
                Part::Static {
                    prefix: b"/".to_vec()
                },
                Part::Dynamic {
                    name: b"name".to_vec(),
                    constraint: Some("alpha".into())
                },
                Part::Static {
                    prefix: b"/".to_vec()
                },
            ]))
        );
    }

    #[test]
    fn test_parts_empty() {
        let error = Parts::new(b"").err().unwrap();
        assert_eq!(error, RouteError::EmptyPath);
        insta::assert_snapshot!(error, @"error: empty path");
    }

    #[test]
    fn test_parts_unclosed_braces() {
        let error = Parts::new(b"/{").err().unwrap();
        assert_eq!(
            error,
            RouteError::UnescapedBrace {
                path: "/{".to_string(),
                position: 1,
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: unescaped brace

           Path: /{
                  ^

        tip: Use '{{' and '}}' to represent literal '{' and '}' characters in the path
        "###);

        let error = Parts::new(b"/{name").err().unwrap();
        assert_eq!(
            error,
            RouteError::UnescapedBrace {
                path: "/{name".to_string(),
                position: 1,
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: unescaped brace

           Path: /{name
                  ^

        tip: Use '{{' and '}}' to represent literal '{' and '}' characters in the path
        "###);

        let error = Parts::new(b"/name}").err().unwrap();
        assert_eq!(
            error,
            RouteError::UnescapedBrace {
                path: "/name}".to_string(),
                position: 5,
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: unescaped brace

           Path: /name}
                      ^

        tip: Use '{{' and '}}' to represent literal '{' and '}' characters in the path
        "###);
    }

    #[test]
    fn test_parts_empty_braces() {
        let error = Parts::new(b"/{}").err().unwrap();
        assert_eq!(
            error,
            RouteError::EmptyBraces {
                path: "/{}".to_string(),
                position: 1,
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: empty braces

           Path: /{}
                  ^^
        "###);
    }

    #[test]
    fn test_parts_empty_name() {
        let error = Parts::new(b"/{:}").err().unwrap();
        assert_eq!(
            error,
            RouteError::EmptyParameter {
                path: "/{:}".to_string(),
                start: 1,
                length: 3
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: empty parameter name

           Path: /{:}
                  ^^^
        "###);

        let error = Parts::new(b"/{:constraint}").err().unwrap();
        assert_eq!(
            error,
            RouteError::EmptyParameter {
                path: "/{:constraint}".to_string(),
                start: 1,
                length: 13
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: empty parameter name

           Path: /{:constraint}
                  ^^^^^^^^^^^^^
        "###);
    }

    #[test]
    fn test_parts_empty_wildcard() {
        let error = Parts::new(b"/{*}").err().unwrap();
        assert_eq!(
            error,
            RouteError::EmptyWildcard {
                path: "/{*}".to_string(),
                start: 1,
                length: 3
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: empty wildcard name

           Path: /{*}
                  ^^^
        "###);

        let error = Parts::new(b"/{*:constraint}").err().unwrap();
        assert_eq!(
            error,
            RouteError::EmptyWildcard {
                path: "/{*:constraint}".to_string(),
                start: 1,
                length: 14
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: empty wildcard name

           Path: /{*:constraint}
                  ^^^^^^^^^^^^^^
        "###);
    }

    #[test]
    fn test_parts_empty_constraint() {
        let error = Parts::new(b"/{name:}").err().unwrap();
        assert_eq!(
            error,
            RouteError::EmptyConstraint {
                path: "/{name:}".to_string(),
                start: 7,
                length: 1
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: empty constraint name

           Path: /{name:}
                        ^
        "###);
    }

    #[test]
    fn test_parts_invalid_characters() {
        let error = Parts::new(b"/{name/with/slash}").err().unwrap();
        assert_eq!(
            error,
            RouteError::InvalidParameter {
                path: "/{name/with/slash}".to_string(),
                start: 1,
                length: 17
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: invalid parameter name

           Path: /{name/with/slash}
                  ^^^^^^^^^^^^^^^^^

        tip: Parameter names must not contain the characters: ':', '*', '?', '{', '}', '/'
        "###);

        let error = Parts::new(b"/{name{with{brace}").err().unwrap();
        assert_eq!(
            error,
            RouteError::InvalidParameter {
                path: "/{name{with{brace}".to_string(),
                start: 1,
                length: 17
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: invalid parameter name

           Path: /{name{with{brace}
                  ^^^^^^^^^^^^^^^^^

        tip: Parameter names must not contain the characters: ':', '*', '?', '{', '}', '/'
        "###);

        let error = Parts::new(b"/{name{with}brace}").err().unwrap();
        assert_eq!(
            error,
            RouteError::InvalidParameter {
                path: "/{name{with}brace}".to_string(),
                start: 1,
                length: 11
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: invalid parameter name

           Path: /{name{with}brace}
                  ^^^^^^^^^^^

        tip: Parameter names must not contain the characters: ':', '*', '?', '{', '}', '/'
        "###);

        let error = Parts::new(b"/{name:with:colon}").err().unwrap();
        assert_eq!(
            error,
            RouteError::InvalidConstraint {
                path: "/{name:with:colon}".to_string(),
                start: 7,
                length: 10
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: invalid constraint name

           Path: /{name:with:colon}
                        ^^^^^^^^^^

        tip: Constraint names must not contain the characters: ':', '*', '?', '{', '}', '/'
        "###);
    }

    #[test]
    fn test_parts_escaped() {
        assert_eq!(
            Parts::new(b"/{{name}}"),
            Ok(Parts(vec![Part::Static {
                prefix: b"/{name}".to_vec()
            }]))
        );

        assert_eq!(
            Parts::new(b"/name}}"),
            Ok(Parts(vec![Part::Static {
                prefix: b"/name}".to_vec()
            }]))
        );

        assert_eq!(
            Parts::new(b"/name{{"),
            Ok(Parts(vec![Part::Static {
                prefix: b"/name{".to_vec()
            }]))
        );

        assert_eq!(
            Parts::new(b"/{{{name}}}"),
            Ok(Parts(vec![
                Part::Static {
                    prefix: b"}".to_vec()
                },
                Part::Dynamic {
                    name: b"name".to_vec(),
                    constraint: None
                },
                Part::Static {
                    prefix: b"/{".to_vec()
                },
            ]))
        );

        assert_eq!(
            Parts::new(b"/{{{{name}}}}"),
            Ok(Parts(vec![Part::Static {
                prefix: b"/{{name}}".to_vec()
            }]))
        );

        assert_eq!(
            Parts::new(b"{{}}"),
            Ok(Parts(vec![Part::Static {
                prefix: b"{}".to_vec()
            }]))
        );

        assert_eq!(
            Parts::new(b"{{:}}"),
            Ok(Parts(vec![Part::Static {
                prefix: b"{:}".to_vec()
            }]))
        );
    }

    #[test]
    fn test_parts_invalid_escaped() {
        let error = Parts::new(b"{name}}").err().unwrap();
        assert_eq!(
            error,
            RouteError::UnescapedBrace {
                path: "{name}}".to_string(),
                position: 6,
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: unescaped brace

           Path: {name}}
                       ^

        tip: Use '{{' and '}}' to represent literal '{' and '}' characters in the path
        "###);

        let error = Parts::new(b"{{name}").err().unwrap();
        assert_eq!(
            error,
            RouteError::UnescapedBrace {
                path: "{{name}".to_string(),
                position: 6,
            }
        );

        insta::assert_snapshot!(error, @r###"
        error: unescaped brace

           Path: {{name}
                       ^

        tip: Use '{{' and '}}' to represent literal '{' and '}' characters in the path
        "###);
    }
}
