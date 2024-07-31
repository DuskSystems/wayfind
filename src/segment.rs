use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Segment<'a> {
    Static { prefix: &'a [u8] },
    Dynamic { name: &'a [u8] },
    Wildcard { name: &'a [u8] },
    Regex { name: &'a [u8], pattern: &'a [u8] },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Segments<'a>(Vec<Segment<'a>>);

impl<'a> Segments<'a> {
    #[must_use]
    pub fn new(path: &'a [u8]) -> Self {
        let mut segments = vec![];
        let mut index = 0;

        while index < path.len() {
            if path[index] == b'{' {
                let (name, value) = Self::parse_parameter(path, &mut index);
                if let Some(value) = value {
                    if value.starts_with(b"*") {
                        segments.push(Segment::Wildcard { name });
                    } else {
                        segments.push(Segment::Regex { name, pattern: value });
                    }
                } else {
                    segments.push(Segment::Dynamic { name });
                }
            } else {
                let prefix = Self::parse_static(path, &mut index);
                segments.push(Segment::Static { prefix });
            }
        }

        Self(segments)
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

        if let Some(colon) = colon {
            let name = &path[start..colon];
            let value = &path[colon + 1..end];
            (name, Some(value))
        } else {
            let name = &path[start..end];
            (name, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segments_static() {
        assert_eq!(
            Segments::new(b"/abcd"),
            Segments(vec![Segment::Static { prefix: b"/abcd" }])
        );
    }

    #[test]
    fn test_segments_dynamic() {
        assert_eq!(
            Segments::new(b"/{name}"),
            Segments(vec![
                Segment::Static { prefix: b"/" },
                Segment::Dynamic { name: b"name" }
            ])
        );
    }

    #[test]
    fn test_segments_wildcard() {
        assert_eq!(
            Segments::new(b"/{path:*}"),
            Segments(vec![
                Segment::Static { prefix: b"/" },
                Segment::Wildcard { name: b"path" }
            ])
        );
    }

    #[test]
    fn test_segments_regex() {
        assert_eq!(
            Segments::new(b"/{id:[0-9]+}"),
            Segments(vec![
                Segment::Static { prefix: b"/" },
                Segment::Regex {
                    name: b"id",
                    pattern: b"[0-9]+"
                }
            ])
        );
    }

    #[test]
    fn test_segments_mixed() {
        assert_eq!(
            Segments::new(b"/users/{id:[0-9]+}/posts/{file}.{extension}"),
            Segments(vec![
                Segment::Static { prefix: b"/users/" },
                Segment::Regex {
                    name: b"id",
                    pattern: b"[0-9]+"
                },
                Segment::Static { prefix: b"/posts/" },
                Segment::Dynamic { name: b"file" },
                Segment::Static { prefix: b"." },
                Segment::Dynamic { name: b"extension" }
            ])
        );
    }
}
