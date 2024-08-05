use crate::errors::insert::InsertError;
use std::fmt::Debug;

#[cfg(regex)]
use regex::bytes::Regex;

#[derive(Debug)]
pub enum Part {
    Static {
        prefix: Vec<u8>,
    },

    Dynamic {
        name: Vec<u8>,
    },

    Wildcard {
        name: Vec<u8>,
    },

    #[cfg(regex)]
    Regex {
        name: Vec<u8>,
        pattern: Regex,
    },
}

impl PartialEq for Part {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Static { prefix: a }, Self::Static { prefix: b })
            | (Self::Dynamic { name: a }, Self::Dynamic { name: b })
            | (Self::Wildcard { name: a }, Self::Wildcard { name: b }) => a == b,
            #[cfg(regex)]
            (Self::Regex { name: a, pattern: p1 }, Self::Regex { name: b, pattern: p2 }) => {
                a == b && p1.as_str() == p2.as_str()
            }
            _ => false,
        }
    }
}

impl Eq for Part {}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts(Vec<Part>);

impl Parts {
    pub fn new(path: &[u8]) -> Result<Self, InsertError> {
        let mut parts = vec![];
        let mut index = 0;

        while index < path.len() {
            if path[index] == b'<' {
                let (name, value) = Self::parse_parameter(path, &mut index);
                if let Some(value) = value {
                    if value == b"*" {
                        parts.push(Part::Wildcard { name: name.to_vec() });
                    } else {
                        #[cfg(regex)]
                        {
                            let Ok(value_str) = std::str::from_utf8(value) else {
                                return Err(InsertError::InvalidRegex);
                            };

                            let Ok(pattern) = Regex::new(value_str) else {
                                return Err(InsertError::InvalidRegex);
                            };

                            parts.push(Part::Regex { name, pattern });
                        }

                        #[cfg(not(regex))]
                        {
                            return Err(InsertError::RegexNotEnabled);
                        }
                    }
                } else {
                    parts.push(Part::Dynamic { name: name.to_vec() });
                }
            } else {
                let prefix = Self::parse_static(path, &mut index);
                parts.push(Part::Static {
                    prefix: prefix.to_vec(),
                });
            }
        }

        // Reverse here to allow for easy 'popping'
        parts.reverse();
        Ok(Self(parts))
    }

    pub fn pop(&mut self) -> Option<Part> {
        self.0.pop()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn parse_static<'a>(path: &'a [u8], index: &mut usize) -> &'a [u8] {
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

    fn parse_parameter<'a>(path: &'a [u8], index: &mut usize) -> (&'a [u8], Option<&'a [u8]>) {
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

    #[cfg(regex)]
    use std::error::Error;

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
            Parts::new(b"/<name>"),
            Ok(Parts(vec![
                Part::Dynamic { name: b"name".to_vec() },
                Part::Static { prefix: b"/".to_vec() },
            ]))
        );
    }

    #[test]
    fn test_parts_wildcard() {
        assert_eq!(
            Parts::new(b"/<path:*>"),
            Ok(Parts(vec![
                Part::Wildcard { name: b"path".to_vec() },
                Part::Static { prefix: b"/".to_vec() },
            ]))
        );
    }

    #[test]
    #[cfg(regex)]
    fn test_parts_regex() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            Parts::new(b"/<id:[0-9]+>"),
            Ok(Parts(vec![
                Part::Regex {
                    name: b"id".to_vec(),
                    pattern: Regex::new("[0-9]+")?,
                },
                Part::Static { prefix: b"/".to_vec() },
            ]))
        );

        Ok(())
    }

    #[test]
    #[cfg(regex)]
    fn test_parts_mixed() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            Parts::new(b"/users/<id:[0-9]+>/posts/<file>.<extension>"),
            Ok(Parts(vec![
                Part::Dynamic {
                    name: b"extension".to_vec()
                },
                Part::Static { prefix: b".".to_vec() },
                Part::Dynamic { name: b"file".to_vec() },
                Part::Static {
                    prefix: b"/posts/".to_vec()
                },
                Part::Regex {
                    name: b"id".to_vec(),
                    pattern: Regex::new("[0-9]+")?,
                },
                Part::Static {
                    prefix: b"/users/".to_vec()
                },
            ]))
        );

        Ok(())
    }
}
