use std::ops::Range;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ByteView<'a> {
    pub input: &'a [u8],
    pub range: Range<usize>,
}

impl<'a> ByteView<'a> {
    #[must_use]
    pub const fn new(input: &'a [u8], range: Range<usize>) -> Self {
        Self { input, range }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.input[self.range.clone()]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route<'a> {
    pub segments: Vec<Range<usize>>,
    pub parts: Vec<Part<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part<'a> {
    Static {
        // Vec of ranges becomes Vec of ByteViews
        prefix: Vec<ByteView<'a>>,
    },

    Dynamic {
        name: ByteView<'a>,
        constraint: Option<ByteView<'a>>,
    },

    Wildcard {
        name: ByteView<'a>,
        constraint: Option<ByteView<'a>>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parser<'a> {
    pub input: &'a [u8],
    pub routes: Vec<Route<'a>>,
}

impl<'a> Parser<'a> {
    #[must_use]
    pub fn new(input: &'a [u8]) -> Self {
        let mut routes = Self::expand_optional_groups(input, 0..input.len());
        for route in &mut routes {
            route.parts = Self::parse_segments(input, &route.segments);
        }

        Self { input, routes }
    }

    #[must_use]
    pub fn routes(&self) -> Vec<String> {
        self.routes
            .iter()
            .map(|route| {
                let parts: Vec<_> = route
                    .segments
                    .iter()
                    .map(|range| &self.input[range.clone()])
                    .collect();

                String::from_utf8_lossy(&parts.concat()).to_string()
            })
            .collect()
    }

    fn expand_optional_groups(input: &'a [u8], range: Range<usize>) -> Vec<Route<'a>> {
        let mut result = vec![Route {
            segments: vec![],
            parts: vec![],
        }];

        let mut groups: Vec<Range<usize>> = vec![];

        let mut cursor = range.start;
        let mut segment = range.start;

        while cursor < range.end {
            match (input[cursor], input.get(cursor + 1)) {
                (b'\\', Some(_)) => {
                    cursor += 2;
                    continue;
                }

                (b'(', _) => {
                    if groups.is_empty() {
                        if cursor > segment {
                            let segment = segment..cursor;
                            for route in &mut result {
                                route.segments.push(segment.clone());
                            }
                        }

                        segment = cursor + 1;
                    }

                    let segments_len = result.first().map_or(0, |r| r.segments.len());
                    groups.push(cursor..segments_len);
                    cursor += 1;
                }

                (b')', _) => {
                    let Some(group) = groups.pop() else {
                        cursor += 1;
                        continue;
                    };

                    if !groups.is_empty() {
                        cursor += 1;
                        continue;
                    }

                    if cursor > segment {
                        let inner_routes = Self::expand_optional_groups(input, segment..cursor);

                        let mut new_result = vec![];
                        for route in result {
                            let end = group.end.min(route.segments.len());
                            for inner_route in &inner_routes {
                                let mut new_route = Route {
                                    segments: route.segments[..end].to_vec(),
                                    parts: vec![],
                                };

                                new_route.segments.extend(inner_route.segments.clone());
                                new_result.push(new_route);
                            }

                            new_result.push(route);
                        }

                        result = new_result;
                    }

                    segment = cursor + 1;
                    cursor += 1;
                }

                _ => {
                    cursor += 1;
                }
            }
        }

        if segment < range.end {
            let last = segment..range.end;
            for route in &mut result {
                route.segments.push(last.clone());
            }
        }

        for route in &mut result {
            if route.segments.is_empty() {
                if let Some(slash) = range.clone().find(|&i| input[i] == b'/') {
                    #[allow(clippy::range_plus_one)]
                    route.segments.push(slash..(slash + 1));
                }
            }
        }

        result
    }

    fn parse_segments(input: &'a [u8], segments: &[Range<usize>]) -> Vec<Part<'a>> {
        let mut parts = vec![];
        let mut current = vec![];

        for range in segments {
            let mut cursor = range.start;
            let mut end = cursor;

            while cursor < range.end {
                match input[cursor] {
                    b'{' => {
                        if cursor > end {
                            current.push(ByteView::new(input, end..cursor));
                        }

                        if !current.is_empty() {
                            parts.push(Part::Static {
                                prefix: std::mem::take(&mut current),
                            });
                        }

                        let start = cursor;
                        while cursor < range.end && input[cursor] != b'}' {
                            cursor += 1;
                        }

                        cursor += 1;

                        parts.push(Self::parse_parameter(input, start..cursor));
                        end = cursor;
                    }
                    _ => {
                        cursor += 1;
                    }
                }
            }

            if end < range.end {
                current.push(ByteView::new(input, end..range.end));
            }
        }

        if !current.is_empty() {
            parts.push(Part::Static { prefix: current });
        }

        parts.reverse();
        parts
    }

    fn parse_parameter(input: &'a [u8], range: Range<usize>) -> Part<'a> {
        let start = range.start + 1;
        let end = range.end - 1;

        let is_wildcard = input[start] == b'*';
        let name_start = if is_wildcard { start + 1 } else { start };

        let (name_end, constraint_range) = (start..end)
            .find(|&i| input[i] == b':')
            .map_or((end, None), |colon_pos| {
                (colon_pos, Some((colon_pos + 1)..end))
            });

        if is_wildcard {
            Part::Wildcard {
                name: ByteView::new(input, name_start..name_end),
                constraint: constraint_range.map(|r| ByteView::new(input, r)),
            }
        } else {
            Part::Dynamic {
                name: ByteView::new(input, name_start..name_end),
                constraint: constraint_range.map(|r| ByteView::new(input, r)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use similar_asserts::assert_eq;

    #[test]
    fn test_parser_static_route() {
        let parser = Parser::new(b"/abcd");
        assert_eq!(parser.routes(), vec!["/abcd"]);
        assert_eq!(
            parser.routes[0].parts,
            vec![Part::Static {
                prefix: vec![ByteView::new(parser.input, 0..5)] // "/abcd"
            }]
        );
    }

    #[test]
    fn test_parser_dynamic_route() {
        let parser = Parser::new(b"/{name}");
        assert_eq!(parser.routes(), vec!["/{name}"]);
        assert_eq!(
            parser.routes[0].parts,
            vec![
                Part::Dynamic {
                    name: ByteView::new(parser.input, 2..6), // "name"
                    constraint: None
                },
                Part::Static {
                    prefix: vec![ByteView::new(parser.input, 0..1)] // "/"
                }
            ]
        );
    }

    #[test]
    fn test_parser_wildcard_route() {
        let parser = Parser::new(b"/{*route}");
        assert_eq!(parser.routes(), vec!["/{*route}"]);
        assert_eq!(
            parser.routes[0].parts,
            vec![
                Part::Wildcard {
                    name: ByteView::new(parser.input, 3..8), // "route"
                    constraint: None
                },
                Part::Static {
                    prefix: vec![ByteView::new(parser.input, 0..1)] // "/"
                }
            ]
        );
    }

    #[test]
    fn test_parser_complex_route() {
        let parser = Parser::new(b"/{*name:alpha}/{id:numeric}");
        assert_eq!(parser.routes(), vec!["/{*name:alpha}/{id:numeric}"]);
        assert_eq!(
            parser.routes[0].parts,
            vec![
                Part::Dynamic {
                    name: ByteView::new(parser.input, 16..18), // "id"
                    constraint: Some(ByteView::new(parser.input, 19..26))  // "numeric"
                },
                Part::Static {
                    prefix: vec![ByteView::new(parser.input, 14..15)] // "/"
                },
                Part::Wildcard {
                    name: ByteView::new(parser.input, 3..7), // "name"
                    constraint: Some(ByteView::new(parser.input, 8..13))  // "alpha"
                },
                Part::Static {
                    prefix: vec![ByteView::new(parser.input, 0..1)] // "/"
                }
            ]
        );
    }

    #[test]
    fn test_parser_optional_group_simple() {
        let parser = Parser::new(b"/users(/{id})");
        assert_eq!(parser.routes(), vec!["/users/{id}", "/users"]);
        assert_eq!(
            parser.routes[0].parts, // "/users/{id}"
            vec![
                Part::Dynamic {
                    name: ByteView::new(parser.input, 9..11), // "id"
                    constraint: None
                },
                Part::Static {
                    prefix: vec![
                        ByteView::new(parser.input, 0..6), // "/users"
                        ByteView::new(parser.input, 7..8)  // "/"
                    ]
                },
            ]
        );
        assert_eq!(
            parser.routes[1].parts, // "/users"
            vec![Part::Static {
                prefix: vec![ByteView::new(parser.input, 0..6)] // "/users"
            }]
        );
    }

    #[test]
    fn test_parser_optional_groups_nested() {
        let parser = Parser::new(b"/users(/{id}(/profile))");
        assert_eq!(
            parser.routes(),
            vec!["/users/{id}/profile", "/users/{id}", "/users"]
        );
        assert_eq!(
            parser.routes[0].parts, // "/users/{id}/profile"
            vec![
                Part::Static {
                    prefix: vec![ByteView::new(parser.input, 13..21)] // "/profile"
                },
                Part::Dynamic {
                    name: ByteView::new(parser.input, 9..11), // "id"
                    constraint: None
                },
                Part::Static {
                    prefix: vec![
                        ByteView::new(parser.input, 0..6), // "/users"
                        ByteView::new(parser.input, 7..8)  // "/"
                    ]
                }
            ]
        );
        assert_eq!(
            parser.routes[1].parts, // "/users/{id}"
            vec![
                Part::Dynamic {
                    name: ByteView::new(parser.input, 9..11), // "id"
                    constraint: None
                },
                Part::Static {
                    prefix: vec![
                        ByteView::new(parser.input, 0..6), // "/users"
                        ByteView::new(parser.input, 7..8)  // "/"
                    ]
                }
            ]
        );
        assert_eq!(
            parser.routes[2].parts, // "/users"
            vec![Part::Static {
                prefix: vec![ByteView::new(parser.input, 0..6)] // "/users"
            }]
        );
    }

    // #[test]
    // fn test_parser_escaped_characters() {
    //     let parser = Parser::new(b"/path/with\\{braces\\}and\\(parens\\)");
    //     assert_eq!(
    //         parser.routes(),
    //         vec!["/path/with\\{braces\\}and\\(parens\\)"]
    //     );
    // }

    #[test]
    fn test_parser_edge_case_starting_optional_group() {
        let parser = Parser::new(b"(/{lang})/users");
        assert_eq!(parser.routes(), vec!["/{lang}/users", "/users"]);
        assert_eq!(
            parser.routes[0].parts, // "/{lang}/users"
            vec![
                Part::Static {
                    prefix: vec![ByteView::new(parser.input, 9..15)] // "/users"
                },
                Part::Dynamic {
                    name: ByteView::new(parser.input, 3..7), // "lang"
                    constraint: None
                },
                Part::Static {
                    prefix: vec![ByteView::new(parser.input, 1..2)] // "/"
                }
            ]
        );
        assert_eq!(
            parser.routes[1].parts, // "/users"
            vec![Part::Static {
                prefix: vec![ByteView::new(parser.input, 9..15)] // "/users"
            }]
        );
    }

    #[test]
    fn test_parser_edge_case_only_optional_groups() {
        let parser = Parser::new(b"(/{lang})(/{page})");
        assert_eq!(
            parser.routes(),
            vec!["/{lang}/{page}", "/{lang}", "/{page}", "/"]
        );
        assert_eq!(
            parser.routes[0].parts, // "/{lang}/{page}"
            vec![
                Part::Dynamic {
                    name: ByteView::new(parser.input, 12..16), // "page"
                    constraint: None
                },
                Part::Static {
                    prefix: vec![ByteView::new(parser.input, 10..11)] // "/"
                },
                Part::Dynamic {
                    name: ByteView::new(parser.input, 3..7), // "lang"
                    constraint: None
                },
                Part::Static {
                    prefix: vec![ByteView::new(parser.input, 1..2)] // "/"
                }
            ]
        );
        assert_eq!(
            parser.routes[1].parts, // "/{lang}"
            vec![
                Part::Dynamic {
                    name: ByteView::new(parser.input, 3..7), // "lang"
                    constraint: None
                },
                Part::Static {
                    prefix: vec![ByteView::new(parser.input, 1..2)] // "/"
                }
            ]
        );
        assert_eq!(
            parser.routes[2].parts, // "/{page}"
            vec![
                Part::Dynamic {
                    name: ByteView::new(parser.input, 12..16), // "page"
                    constraint: None
                },
                Part::Static {
                    prefix: vec![ByteView::new(parser.input, 10..11)] // "/"
                }
            ]
        );
        assert_eq!(
            parser.routes[3].parts, // "/"
            vec![Part::Static {
                prefix: vec![ByteView::new(parser.input, 1..2)] // "/"
            }]
        );
    }
}
