use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Part<'a> {
    Static { prefix: &'a [u8] },
    Dynamic { name: &'a [u8] },
    Wildcard { name: &'a [u8] },
    Regex { name: &'a [u8], pattern: &'a [u8] },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parts<'a>(Vec<Part<'a>>);

impl<'a> Parts<'a> {
    #[must_use]
    pub fn new(path: &'a [u8]) -> Self {
        let mut parts = vec![];
        let mut index = 0;

        while index < path.len() {
            if path[index] == b'{' {
                let (name, value) = Self::parse_parameter(path, &mut index);
                if let Some(value) = value {
                    if value.starts_with(b"*") {
                        parts.push(Part::Wildcard { name });
                    } else {
                        parts.push(Part::Regex { name, pattern: value });
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
        Self(parts)
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

    fn parse_parameter(path: &'a [u8], index: &mut usize) -> (&'a [u8], Option<&'a [u8]>) {
        // Consume opening '{'
        *index += 1;
        let start = *index;

        // Consume until we see a '}'
        let mut colon = None;
        while *index < path.len() {
            if path[*index] == b'}' {
                break;
            }

            if path[*index] == b':' && colon.is_none() {
                colon = Some(*index);
            }

            *index += 1;
        }

        // Consume closing '}'
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
        assert_eq!(Parts(vec![Part::Static { prefix: b"/abcd" }]), Parts::new(b"/abcd"),);
    }

    #[test]
    fn test_parts_dynamic() {
        assert_eq!(
            Parts::new(b"/{name}"),
            Parts(vec![Part::Dynamic { name: b"name" }, Part::Static { prefix: b"/" },])
        );
    }

    #[test]
    fn test_parts_wildcard() {
        assert_eq!(
            Parts::new(b"/{path:*}"),
            Parts(vec![Part::Wildcard { name: b"path" }, Part::Static { prefix: b"/" },])
        );
    }

    #[test]
    fn test_parts_regex() {
        assert_eq!(
            Parts::new(b"/{id:[0-9]+}"),
            Parts(vec![
                Part::Regex {
                    name: b"id",
                    pattern: b"[0-9]+"
                },
                Part::Static { prefix: b"/" },
            ])
        );
    }

    #[test]
    fn test_parts_mixed() {
        assert_eq!(
            Parts::new(b"/users/{id:[0-9]+}/posts/{file}.{extension}"),
            Parts(vec![
                Part::Dynamic { name: b"extension" },
                Part::Static { prefix: b"." },
                Part::Dynamic { name: b"file" },
                Part::Static { prefix: b"/posts/" },
                Part::Regex {
                    name: b"id",
                    pattern: b"[0-9]+"
                },
                Part::Static { prefix: b"/users/" },
            ])
        );
    }
}
