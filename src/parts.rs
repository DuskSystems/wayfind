use crate::errors::route::RouteError;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Part<'a> {
    Static { prefix: &'a [u8] },
    Dynamic { name: &'a [u8] },
    Wildcard { name: &'a [u8] },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a>(Vec<Part<'a>>);

impl<'a> Parts<'a> {
    pub fn new(path: &'a [u8]) -> Result<Self, RouteError> {
        let mut parts = vec![];
        let mut index = 0;

        while index < path.len() {
            if path[index] == b'<' {
                let (name, value) = Self::parse_parameter(path, &mut index);
                if let Some(value) = value {
                    if value == b"*" {
                        parts.push(Part::Wildcard { name });
                    } else {
                        return Err(RouteError::InvalidPath);
                    }
                } else {
                    parts.push(Part::Dynamic { name });
                }
            } else {
                let prefix = Self::parse_static(path, &mut index);
                parts.push(Part::Static { prefix });
            }
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

        // Consume up until the next '<'
        while *index < path.len() {
            if path[*index] == b'<' {
                break;
            }

            *index += 1;
        }

        &path[start..*index]
    }

    fn parse_parameter(path: &'a [u8], index: &mut usize) -> (&'a [u8], Option<&'a [u8]>) {
        // Consume opening '<'
        *index += 1;
        let start = *index;

        // Consume until we see a '>'
        let mut colon = None;
        while *index < path.len() {
            if path[*index] == b'>' {
                break;
            }

            if path[*index] == b':' && colon.is_none() {
                colon = Some(*index);
            }

            *index += 1;
        }

        // Consume closing '>'
        let end = *index;
        *index += 1;

        colon.map_or_else(
            || {
                let name = &path[start..end];
                (name, None)
            },
            |colon| {
                let name = &path[start..colon];
                let value = &path[colon + 1..end];
                (name, Some(value))
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts_static() {
        assert_eq!(Parts::new(b"/abcd"), Ok(Parts(vec![Part::Static { prefix: b"/abcd" }])),);
    }

    #[test]
    fn test_parts_dynamic() {
        assert_eq!(
            Parts::new(b"/<name>"),
            Ok(Parts(vec![
                Part::Dynamic { name: b"name" },
                Part::Static { prefix: b"/" },
            ]))
        );
    }

    #[test]
    fn test_parts_wildcard() {
        assert_eq!(
            Parts::new(b"/<path:*>"),
            Ok(Parts(vec![
                Part::Wildcard { name: b"path" },
                Part::Static { prefix: b"/" },
            ]))
        );
    }
}
