use crate::errors::RouteError;
use std::{
    collections::VecDeque,
    fmt::{self, Debug},
};

/// Characters that are not allowed in parameter names or constraints.
const INVALID_PARAM_CHARS: [u8; 6] = [b':', b'*', b'?', b'{', b'}', b'/'];

/// A parsed section of a route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutePart {
    Static {
        prefix: Vec<u8>,
    },

    Dynamic {
        name: Vec<u8>,
        optional: bool,
        constraint: Option<Vec<u8>>,
    },

    Wildcard {
        name: Vec<u8>,
        optional: bool,
        constraint: Option<Vec<u8>>,
    },
}

impl RoutePart {
    pub fn disable_optional(&mut self) {
        match self {
            Self::Dynamic { optional, .. } | Self::Wildcard { optional, .. } => {
                *optional = false;
            }
            Self::Static { .. } => {}
        }
    }

    pub fn ends_with_slash(&self) -> bool {
        match self {
            Self::Static { prefix } => prefix.last() == Some(&b'/'),
            _ => false,
        }
    }

    pub fn starts_with_slash(&self) -> bool {
        match self {
            Self::Static { prefix } => prefix.first() == Some(&b'/'),
            _ => false,
        }
    }
}

/// The parsed parts of the route, in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteParts(pub VecDeque<RoutePart>);

impl RouteParts {
    pub fn pop_front(&mut self) -> Option<RoutePart> {
        self.0.pop_front()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, RoutePart> {
        self.into_iter()
    }
}

impl fmt::Display for RouteParts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for part in &self.0 {
            match part {
                RoutePart::Static { prefix } => {
                    write!(f, "{}", String::from_utf8_lossy(prefix))?;
                }
                RoutePart::Dynamic {
                    name,
                    optional,
                    constraint,
                    ..
                } => {
                    write!(f, "{{{}", String::from_utf8_lossy(name))?;
                    if *optional {
                        write!(f, "?")?;
                    }

                    if let Some(constraint) = constraint {
                        write!(f, ":{}", String::from_utf8_lossy(constraint))?;
                    }

                    write!(f, "}}")?;
                }
                RoutePart::Wildcard {
                    name,
                    optional,
                    constraint,
                    ..
                } => {
                    write!(f, "{{*{}", String::from_utf8_lossy(name))?;

                    if *optional {
                        write!(f, "?")?;
                    }

                    if let Some(constraint) = constraint {
                        write!(f, ":{}", String::from_utf8_lossy(constraint))?;
                    }

                    write!(f, "}}")?;
                }
            }
        }

        Ok(())
    }
}

impl IntoIterator for RouteParts {
    type Item = RoutePart;
    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a RouteParts {
    type Item = &'a RoutePart;
    type IntoIter = std::collections::vec_deque::Iter<'a, RoutePart>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// A parsed route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedRoute {
    pub raw: Vec<u8>,
    pub parts: RouteParts,
    pub trailing_slash: bool,
}

impl ParsedRoute {
    pub fn new(route: &[u8]) -> Result<Self, RouteError> {
        if route.is_empty() {
            return Err(RouteError::EmptyRoute);
        }

        let mut parts = VecDeque::new();
        let mut cursor = 0;
        let mut current_static = vec![];

        while cursor < route.len() {
            match (route[cursor], route.get(cursor + 1)) {
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
                        parts.push_back(RoutePart::Static {
                            prefix: std::mem::take(&mut current_static),
                        });
                    }

                    cursor = Self::parse_parameter(route, cursor, &mut parts)?;
                }
                (b'}', _) => {
                    return Err(RouteError::UnescapedBrace {
                        route: String::from_utf8_lossy(route).to_string(),
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
            parts.push_back(RoutePart::Static {
                prefix: std::mem::take(&mut current_static),
            });
        }

        // Check for trailing slash parameter.
        let mut trailing_slash = false;
        if let Some(RoutePart::Dynamic { name, .. }) = parts.back() {
            if name == b"/" {
                // Remove the trailing slash
                parts.pop_back();

                // Check if the second last (now last) part ends in "/"
                if parts.back().map_or(false, RoutePart::ends_with_slash) {
                    return Err(RouteError::ConflictingTrailingSlash {
                        route: String::from_utf8_lossy(route).to_string() + "/",
                        position: route.len(),
                    });
                }

                trailing_slash = true;
            }
        }

        Ok(Self {
            raw: route.to_vec(),
            parts: RouteParts(parts),
            trailing_slash,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn parse_parameter(
        route: &[u8],
        cursor: usize,
        parts: &mut VecDeque<RoutePart>,
    ) -> Result<usize, RouteError> {
        let start = cursor + 1;
        let end = route[start..]
            .iter()
            .position(|&c| c == b'}')
            .map(|pos| start + pos)
            .ok_or(RouteError::UnescapedBrace {
                route: String::from_utf8_lossy(route).to_string(),
                position: cursor,
            })?;

        if start == end {
            return Err(RouteError::EmptyBraces {
                route: String::from_utf8_lossy(route).to_string(),
                position: cursor,
            });
        }

        let colon = route[start..end].iter().position(|&c| c == b':');
        let (name, constraint) = colon.map_or_else(
            || (&route[start..end], None),
            |pos| {
                (
                    &route[start..start + pos],
                    Some(&route[start + pos + 1..end]),
                )
            },
        );

        let (wildcard, name) = if name.starts_with(b"*") {
            (true, &name[1..])
        } else {
            (false, name)
        };

        let (optional, name) = if name.ends_with(b"?") {
            (true, &name[..name.len() - 1])
        } else {
            (false, name)
        };

        // Special case for trailing slash.
        if name == b"/" && constraint.is_none() {
            // Ensure it's at the end of the route.
            if end + 1 != route.len() {
                return Err(RouteError::InvalidTrailingSlash {
                    route: String::from_utf8_lossy(route).to_string(),
                    position: cursor,
                });
            }

            parts.push_back(RoutePart::Dynamic {
                name: name.to_vec(),
                optional: false,
                constraint: None,
            });

            return Ok(end + 1);
        }

        if name.is_empty() {
            if wildcard {
                return Err(RouteError::EmptyWildcard {
                    route: String::from_utf8_lossy(route).to_string(),
                    start: cursor,
                    length: end - start + 2,
                });
            }

            return Err(RouteError::EmptyParameter {
                route: String::from_utf8_lossy(route).to_string(),
                start: cursor,
                length: end - start + 2,
            });
        }

        if name.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
            return Err(RouteError::InvalidParameter {
                route: String::from_utf8_lossy(route).to_string(),
                name: String::from_utf8_lossy(name).to_string(),
                start: start - 1,
                length: end - start + 2,
            });
        }

        if let Some(constraint) = constraint {
            if constraint.is_empty() {
                return Err(RouteError::EmptyConstraint {
                    route: String::from_utf8_lossy(route).to_string(),
                    start: start - 1,
                    length: end - start + 2,
                });
            }

            if constraint.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
                return Err(RouteError::InvalidConstraint {
                    route: String::from_utf8_lossy(route).to_string(),
                    name: String::from_utf8_lossy(constraint).to_string(),
                    start: start - 1,
                    length: end - start + 2,
                });
            }
        }

        let part = if wildcard {
            RoutePart::Wildcard {
                name: name.to_vec(),
                optional,
                constraint: constraint.map(<[u8]>::to_vec),
            }
        } else {
            RoutePart::Dynamic {
                name: name.to_vec(),
                optional,
                constraint: constraint.map(<[u8]>::to_vec),
            }
        };

        parts.push_back(part);
        Ok(end + 1)
    }
}

impl From<RouteParts> for ParsedRoute {
    fn from(value: RouteParts) -> Self {
        let mut parts = VecDeque::new();
        let mut current_static = Vec::new();

        for part in value.0 {
            if let RoutePart::Static { prefix } = part {
                current_static.extend_from_slice(&prefix);
            } else {
                if !current_static.is_empty() {
                    parts.push_back(RoutePart::Static {
                        prefix: std::mem::take(&mut current_static),
                    });
                }
                parts.push_back(part);
            }
        }

        if !current_static.is_empty() {
            parts.push_back(RoutePart::Static {
                prefix: current_static,
            });
        }

        let parts = RouteParts(parts);
        let raw = parts.to_string().into_bytes();

        Self {
            raw,
            parts,
            trailing_slash: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use similar_asserts::assert_eq;

    #[test]
    fn test_parts_static() {
        assert_eq!(
            ParsedRoute::new(b"/abcd"),
            Ok(ParsedRoute {
                raw: b"/abcd".to_vec(),
                parts: RouteParts(VecDeque::from(vec![RoutePart::Static {
                    prefix: b"/abcd".to_vec()
                }])),
                trailing_slash: false,
            })
        );
    }

    #[test]
    fn test_parts_dynamic() {
        assert_eq!(
            ParsedRoute::new(b"/{name}"),
            Ok(ParsedRoute {
                raw: b"/{name}".to_vec(),
                parts: RouteParts(VecDeque::from(vec![
                    RoutePart::Static {
                        prefix: b"/".to_vec()
                    },
                    RoutePart::Dynamic {
                        name: b"name".to_vec(),
                        optional: false,
                        constraint: None
                    },
                ])),
                trailing_slash: false,
            })
        );
    }

    #[test]
    fn test_parts_wildcard() {
        assert_eq!(
            ParsedRoute::new(b"/{*route}"),
            Ok(ParsedRoute {
                raw: b"/{*route}".to_vec(),
                parts: RouteParts(VecDeque::from(vec![
                    RoutePart::Static {
                        prefix: b"/".to_vec()
                    },
                    RoutePart::Wildcard {
                        name: b"route".to_vec(),
                        optional: false,
                        constraint: None
                    },
                ])),
                trailing_slash: false,
            })
        );
    }

    #[test]
    fn test_parts_optional() {
        assert_eq!(
            ParsedRoute::new(b"/release/v{major}.{minor?}.{patch?}"),
            Ok(ParsedRoute {
                raw: b"/release/v{major}.{minor?}.{patch?}".to_vec(),
                parts: RouteParts(VecDeque::from(vec![
                    RoutePart::Static {
                        prefix: b"/release/v".to_vec()
                    },
                    RoutePart::Dynamic {
                        name: b"major".to_vec(),
                        optional: false,
                        constraint: None
                    },
                    RoutePart::Static {
                        prefix: b".".to_vec()
                    },
                    RoutePart::Dynamic {
                        name: b"minor".to_vec(),
                        optional: true,
                        constraint: None
                    },
                    RoutePart::Static {
                        prefix: b".".to_vec()
                    },
                    RoutePart::Dynamic {
                        name: b"patch".to_vec(),
                        optional: true,
                        constraint: None
                    },
                ])),
                trailing_slash: false,
            })
        );
    }

    #[test]
    fn test_parts_constraint() {
        assert_eq!(
            ParsedRoute::new(b"/{name:alpha}/{id:numeric}"),
            Ok(ParsedRoute {
                raw: b"/{name:alpha}/{id:numeric}".to_vec(),
                parts: RouteParts(VecDeque::from(vec![
                    RoutePart::Static {
                        prefix: b"/".to_vec()
                    },
                    RoutePart::Dynamic {
                        name: b"name".to_vec(),
                        optional: false,
                        constraint: Some(b"alpha".to_vec())
                    },
                    RoutePart::Static {
                        prefix: b"/".to_vec()
                    },
                    RoutePart::Dynamic {
                        name: b"id".to_vec(),
                        optional: false,
                        constraint: Some(b"numeric".to_vec())
                    },
                ])),
                trailing_slash: false,
            })
        );
    }

    #[test]
    fn test_parts_trailing_slash() {
        assert_eq!(
            ParsedRoute::new(b"/users/{id}/posts{/}"),
            Ok(ParsedRoute {
                raw: b"/users/{id}/posts{/}".to_vec(),
                parts: RouteParts(VecDeque::from(vec![
                    RoutePart::Static {
                        prefix: b"/users/".to_vec(),
                    },
                    RoutePart::Dynamic {
                        name: b"id".to_vec(),
                        optional: false,
                        constraint: None,
                    },
                    RoutePart::Static {
                        prefix: b"/posts".to_vec(),
                    },
                ])),
                trailing_slash: true,
            })
        );
    }

    #[test]
    fn test_parts_empty() {
        let error = ParsedRoute::new(b"").unwrap_err();
        insta::assert_snapshot!(error, @"empty route");
    }

    #[test]
    fn test_parts_unclosed_braces() {
        let error = ParsedRoute::new(b"/{").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        unescaped brace

           Route: /{
                   ^

        tip: Use '{{' and '}}' to represent literal '{' and '}' characters in the route
        "#);

        let error = ParsedRoute::new(b"/{name").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        unescaped brace

           Route: /{name
                   ^

        tip: Use '{{' and '}}' to represent literal '{' and '}' characters in the route
        "#);

        let error = ParsedRoute::new(b"/name}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        unescaped brace

           Route: /name}
                       ^

        tip: Use '{{' and '}}' to represent literal '{' and '}' characters in the route
        "#);
    }

    #[test]
    fn test_parts_empty_braces() {
        let error = ParsedRoute::new(b"/{}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        empty braces

           Route: /{}
                   ^^
        "#);
    }

    #[test]
    fn test_parts_empty_name() {
        let error = ParsedRoute::new(b"/{:}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        empty parameter name

           Route: /{:}
                   ^^^
        "#);

        let error = ParsedRoute::new(b"/{:constraint}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        empty parameter name

           Route: /{:constraint}
                   ^^^^^^^^^^^^^
        "#);
    }

    #[test]
    fn test_parts_empty_wildcard() {
        let error = ParsedRoute::new(b"/{*}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        empty wildcard name

           Route: /{*}
                   ^^^
        "#);

        let error = ParsedRoute::new(b"/{*:constraint}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        empty wildcard name

           Route: /{*:constraint}
                   ^^^^^^^^^^^^^^
        "#);
    }

    #[test]
    fn test_parts_empty_constraint() {
        let error = ParsedRoute::new(b"/{name:}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        empty constraint name

           Route: /{name:}
                   ^^^^^^^
        "#);
    }

    #[test]
    fn test_parts_invalid_characters() {
        let error = ParsedRoute::new(b"/{name/with/slash}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        invalid parameter name

           Route: /{name/with/slash}
                   ^^^^^^^^^^^^^^^^^

        tip: Parameter names must not contain the characters: ':', '*', '?', '{', '}', '/'
        "#);

        let error = ParsedRoute::new(b"/{name{with{brace}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        invalid parameter name

           Route: /{name{with{brace}
                   ^^^^^^^^^^^^^^^^^

        tip: Parameter names must not contain the characters: ':', '*', '?', '{', '}', '/'
        "#);

        let error = ParsedRoute::new(b"/{name{with}brace}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        invalid parameter name

           Route: /{name{with}brace}
                   ^^^^^^^^^^^

        tip: Parameter names must not contain the characters: ':', '*', '?', '{', '}', '/'
        "#);

        let error = ParsedRoute::new(b"/{name:with:colon}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        invalid constraint name

           Route: /{name:with:colon}
                   ^^^^^^^^^^^^^^^^^

        tip: Constraint names must not contain the characters: ':', '*', '?', '{', '}', '/'
        "#);
    }

    #[test]
    fn test_parts_escaped() {
        assert_eq!(
            ParsedRoute::new(b"/{{name}}"),
            Ok(ParsedRoute {
                raw: b"/{{name}}".to_vec(),
                parts: RouteParts(VecDeque::from(vec![RoutePart::Static {
                    prefix: b"/{name}".to_vec()
                }])),
                trailing_slash: false,
            })
        );

        assert_eq!(
            ParsedRoute::new(b"/name}}"),
            Ok(ParsedRoute {
                raw: b"/name}}".to_vec(),
                parts: RouteParts(VecDeque::from(vec![RoutePart::Static {
                    prefix: b"/name}".to_vec()
                }])),
                trailing_slash: false,
            })
        );

        assert_eq!(
            ParsedRoute::new(b"/name{{"),
            Ok(ParsedRoute {
                raw: b"/name{{".to_vec(),
                parts: RouteParts(VecDeque::from(vec![RoutePart::Static {
                    prefix: b"/name{".to_vec()
                }])),
                trailing_slash: false,
            })
        );

        assert_eq!(
            ParsedRoute::new(b"/{{{name}}}"),
            Ok(ParsedRoute {
                raw: b"/{{{name}}}".to_vec(),
                parts: RouteParts(VecDeque::from(vec![
                    RoutePart::Static {
                        prefix: b"/{".to_vec()
                    },
                    RoutePart::Dynamic {
                        name: b"name".to_vec(),
                        optional: false,
                        constraint: None
                    },
                    RoutePart::Static {
                        prefix: b"}".to_vec()
                    },
                ])),
                trailing_slash: false,
            })
        );

        assert_eq!(
            ParsedRoute::new(b"/{{{{name}}}}"),
            Ok(ParsedRoute {
                raw: b"/{{{{name}}}}".to_vec(),
                parts: RouteParts(VecDeque::from(vec![RoutePart::Static {
                    prefix: b"/{{name}}".to_vec()
                }])),
                trailing_slash: false,
            })
        );

        assert_eq!(
            ParsedRoute::new(b"{{}}"),
            Ok(ParsedRoute {
                raw: b"{{}}".to_vec(),
                parts: RouteParts(VecDeque::from(vec![RoutePart::Static {
                    prefix: b"{}".to_vec()
                }])),
                trailing_slash: false,
            })
        );

        assert_eq!(
            ParsedRoute::new(b"{{:}}"),
            Ok(ParsedRoute {
                raw: b"{{:}}".to_vec(),
                parts: RouteParts(VecDeque::from(vec![RoutePart::Static {
                    prefix: b"{:}".to_vec()
                }])),
                trailing_slash: false,
            })
        );
    }

    #[test]
    fn test_parts_invalid_escaped() {
        let error = ParsedRoute::new(b"{name}}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        unescaped brace

           Route: {name}}
                        ^

        tip: Use '{{' and '}}' to represent literal '{' and '}' characters in the route
        "#);

        let error = ParsedRoute::new(b"{{name}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        unescaped brace

           Route: {{name}
                        ^

        tip: Use '{{' and '}}' to represent literal '{' and '}' characters in the route
        "#);
    }

    #[test]
    fn test_invalid_trailing_slash() {
        let error = ParsedRoute::new(b"/users/{/}/posts").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        invalid trailing slash

           Route: /users/{/}/posts
                         ^^^

        tip: Trailing slash parameters must occur at the end of a route
        "#);
    }

    #[test]
    fn test_conflicting_trailing_slash() {
        let error = ParsedRoute::new(b"/users/{id}/{/}").unwrap_err();
        insta::assert_snapshot!(error, @r#"
        conflicting trailing slash

           Route: /users/{id}/{/}/
                                 ^

        tip: Remove the existing trailing slash to allow optional occurrence
        "#);
    }
}
