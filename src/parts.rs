use crate::errors::route::RouteError;
use std::fmt::Debug;

const INVALID_STATIC_CHARS: [u8; 2] = [b'{', b'}'];
const INVALID_PARAM_CHARS: [u8; 4] = [b':', b'/', b'{', b'}'];

#[derive(Debug, PartialEq, Eq)]
pub enum Part<'a> {
    Static {
        prefix: &'a [u8],
    },

    Dynamic {
        name: &'a [u8],
        constraint: Option<Vec<u8>>,
    },

    Wildcard {
        name: &'a [u8],
        constraint: Option<Vec<u8>>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a>(pub Vec<Part<'a>>);

impl<'a> Parts<'a> {
    pub fn new(path: &'a [u8]) -> Result<Self, RouteError> {
        let mut parts = vec![];
        let mut index = 0;

        while index < path.len() {
            if path[index] == b'{' {
                let (name, constraint) = Self::parse_parameter(path, &mut index)?;

                if name.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
                    return Err(RouteError::InvalidPath);
                }

                if let Some(constraint) = &constraint {
                    if constraint.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
                        return Err(RouteError::InvalidPath);
                    }
                }

                if name.starts_with(b"*") {
                    parts.push(Part::Wildcard {
                        name: &name[1..],
                        constraint,
                    });
                } else {
                    parts.push(Part::Dynamic { name, constraint });
                }
            } else {
                let prefix = Self::parse_static(path, &mut index);
                if prefix.iter().any(|&c| INVALID_STATIC_CHARS.contains(&c)) {
                    return Err(RouteError::InvalidPath);
                }

                parts.push(Part::Static { prefix });
            }
        }

        if parts.is_empty() {
            return Err(RouteError::InvalidPath);
        }

        // Reverse here to allow for easy 'popping'
        parts.reverse();
        Ok(Self(parts))
    }

    pub fn pop(&mut self) -> Option<Part<'a>> {
        self.0.pop()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn parse_static(path: &'a [u8], index: &mut usize) -> &'a [u8] {
        let start = *index;

        // Consume up until the next '{'
        while *index < path.len() {
            if path[*index] == b'{' {
                break;
            }

            *index += 1;
        }

        &path[start..*index]
    }

    fn parse_parameter(
        path: &'a [u8],
        index: &mut usize,
    ) -> Result<(&'a [u8], Option<Vec<u8>>), RouteError> {
        // Consume opening '{'
        *index += 1;
        let start = *index;

        // Consume until we see a '}' or ':'
        while *index < path.len() && path[*index] != b'}' && path[*index] != b':' {
            *index += 1;
        }

        // No '}' or ':' found
        if *index == path.len() {
            return Err(RouteError::InvalidPath);
        }

        let name_end = *index;

        // Empty name
        if name_end == start {
            return Err(RouteError::InvalidPath);
        }

        let constraint = if path[*index] == b':' {
            // Consume ':'
            *index += 1;
            let constraint_start = *index;

            // Consume until we see a '}'
            while *index < path.len() && path[*index] != b'}' {
                *index += 1;
            }

            // No '}' found
            if *index == path.len() {
                return Err(RouteError::InvalidPath);
            }

            // Empty constraint
            if *index == constraint_start {
                return Err(RouteError::InvalidPath);
            }

            Some(path[constraint_start..*index].to_vec())
        } else {
            None
        };

        // Consume closing '}'
        *index += 1;

        Ok((&path[start..name_end], constraint))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts_static() {
        assert_eq!(
            Parts::new(b"/abcd"),
            Ok(Parts(vec![Part::Static { prefix: b"/abcd" }])),
        );
    }

    #[test]
    fn test_parts_dynamic() {
        assert_eq!(
            Parts::new(b"/{name}"),
            Ok(Parts(vec![
                Part::Dynamic {
                    name: b"name",
                    constraint: None
                },
                Part::Static { prefix: b"/" },
            ]))
        );
    }

    #[test]
    fn test_parts_wildcard() {
        assert_eq!(
            Parts::new(b"/{*path}"),
            Ok(Parts(vec![
                Part::Wildcard {
                    name: b"path",
                    constraint: None
                },
                Part::Static { prefix: b"/" },
            ]))
        );
    }

    #[test]
    fn test_parts_constraint() {
        assert_eq!(
            Parts::new(b"/{name:alpha}/{id:numeric}"),
            Ok(Parts(vec![
                Part::Dynamic {
                    name: b"id",
                    constraint: Some("numeric".into())
                },
                Part::Static { prefix: b"/" },
                Part::Dynamic {
                    name: b"name",
                    constraint: Some("alpha".into())
                },
                Part::Static { prefix: b"/" },
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
    fn test_parts_unescaped_braces() {
        assert_eq!(
            Parts::new(b"/{name}with}brace}"),
            Err(RouteError::InvalidPath)
        );

        assert_eq!(
            Parts::new(b"/{name}with{brace{"),
            Err(RouteError::InvalidPath)
        );
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
}
