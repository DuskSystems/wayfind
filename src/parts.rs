use crate::errors::route::RouteError;
use std::fmt::Debug;

const INVALID_PARAM_CHARS: [u8; 4] = [b':', b'/', b'{', b'}'];

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
                (b'}', _) => return Err(RouteError::InvalidPath),
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

        if parts.is_empty() {
            return Err(RouteError::InvalidPath);
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
            .ok_or(RouteError::InvalidPath)?;

        let colon = path[start..end].iter().position(|&c| c == b':');
        let (name, constraint) = colon.map_or_else(
            || (&path[start..end], None),
            |pos| (&path[start..start + pos], Some(&path[start + pos + 1..end])),
        );

        if name.is_empty() || name.iter().any(|c| INVALID_PARAM_CHARS.contains(c)) {
            return Err(RouteError::InvalidPath);
        }

        if let Some(constraint) = constraint {
            if constraint.is_empty() || constraint.iter().any(|c| INVALID_PARAM_CHARS.contains(c)) {
                return Err(RouteError::InvalidPath);
            }
        }

        let part = if name.starts_with(b"*") {
            Part::Wildcard {
                name: name[1..].to_vec(),
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
        assert_eq!(Parts::new(b""), Err(RouteError::InvalidPath));
    }

    #[test]
    fn test_parts_unclosed_braces() {
        assert_eq!(Parts::new(b"/{"), Err(RouteError::InvalidPath));
        assert_eq!(Parts::new(b"/{name"), Err(RouteError::InvalidPath));
        assert_eq!(
            Parts::new(b"/{name:constraint"),
            Err(RouteError::InvalidPath)
        );
    }

    #[test]
    fn test_parts_empty_braces() {
        assert_eq!(Parts::new(b"/{}"), Err(RouteError::InvalidPath));
        assert_eq!(Parts::new(b"/{:}"), Err(RouteError::InvalidPath));
    }

    #[test]
    fn test_parts_nested_braces() {
        assert_eq!(Parts::new(b"/{outer{inner}}"), Err(RouteError::InvalidPath));
    }

    #[test]
    fn test_parts_empty_name() {
        assert_eq!(Parts::new(b"/{:constraint}"), Err(RouteError::InvalidPath));
        assert_eq!(Parts::new(b"/{:constraint"), Err(RouteError::InvalidPath));
    }

    #[test]
    fn test_parts_empty_constraint() {
        assert_eq!(Parts::new(b"/{name:}"), Err(RouteError::InvalidPath));
    }

    #[test]
    fn test_parts_invalid_characters() {
        assert_eq!(
            Parts::new(b"/{name:with:colon}"),
            Err(RouteError::InvalidPath)
        );

        assert_eq!(
            Parts::new(b"/{name/with/slash}"),
            Err(RouteError::InvalidPath)
        );

        assert_eq!(
            Parts::new(b"/{name{with{brace}"),
            Err(RouteError::InvalidPath)
        );

        assert_eq!(
            Parts::new(b"/{name{with}brace}"),
            Err(RouteError::InvalidPath)
        );
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
        assert_eq!(Parts::new(b"{name}}"), Err(RouteError::InvalidPath));
        assert_eq!(Parts::new(b"{{name}"), Err(RouteError::InvalidPath));
    }
}
