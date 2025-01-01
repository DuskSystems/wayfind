use crate::errors::AuthorityTemplateError;
use smallvec::{smallvec, SmallVec};
use std::sync::Arc;
use wayfind_tree::parser::{Part, Template};

/// Characters that are not allowed in parameter names or constraints.
const INVALID_PARAM_CHARS: [u8; 5] = [b':', b'*', b'{', b'}', b'.'];

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedAuthority {
    pub input: Arc<str>,
    pub template: Template,
}

impl ParsedAuthority {
    pub fn new(input: &str) -> Result<Self, AuthorityTemplateError> {
        if input.is_empty() {
            return Err(AuthorityTemplateError::Empty);
        }

        Ok(Self {
            input: input.into(),
            template: Self::parse_authority(input)?,
        })
    }

    fn parse_authority(input: &str) -> Result<Template, AuthorityTemplateError> {
        let bytes = input.as_bytes();

        let mut parts = vec![];
        let mut cursor = 0;

        // Parameters in order (name, start, length)
        let mut seen_parameters: SmallVec<[(String, usize, usize); 4]> = smallvec![];

        while cursor < bytes.len() {
            match bytes[cursor] {
                b'{' => {
                    let (part, next_cursor) = Self::parse_parameter_part(input, cursor)?;

                    // Check for touching parameters.
                    if let Some((_, start, length)) = seen_parameters.last() {
                        if cursor == start + length {
                            return Err(AuthorityTemplateError::TouchingParameters {
                                authority: input.to_owned(),
                                start: *start,
                                length: next_cursor - start,
                            });
                        }
                    }

                    // Check for duplicate names.
                    if let Part::Dynamic { name, .. } | Part::Wildcard { name, .. } = &part {
                        if let Some((_, start, length)) = seen_parameters
                            .iter()
                            .find(|(existing, _, _)| existing == name)
                        {
                            return Err(AuthorityTemplateError::DuplicateParameter {
                                authority: input.to_owned(),
                                name: name.to_string(),
                                first: *start,
                                first_length: *length,
                                second: cursor,
                                second_length: next_cursor - cursor,
                            });
                        }

                        seen_parameters.push((name.clone(), cursor, next_cursor - cursor));
                    }

                    parts.push(part);
                    cursor = next_cursor;
                }
                b'}' => {
                    return Err(AuthorityTemplateError::UnbalancedBrace {
                        authority: input.to_owned(),
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

        Ok(Template {
            input: input.into(),
            raw: input.into(),
            parts,
        })
    }

    fn parse_static_part(input: &str, cursor: usize) -> (Part, usize) {
        let bytes = input.as_bytes();
        let mut prefix = vec![];

        let mut end = cursor;
        while end < bytes.len() {
            match (bytes[end], bytes.get(end + 1)) {
                (b'\\', Some(&next_char)) => {
                    prefix.push(next_char);
                    end += 2;
                }
                (b'\\', None) => {
                    prefix.push(b'\\');
                    end += 1;
                }
                (b'{' | b'}', _) => break,
                (char, _) => {
                    prefix.push(char);
                    end += 1;
                }
            }
        }

        (Part::Static { prefix }, end)
    }

    fn parse_parameter_part(
        input: &str,
        cursor: usize,
    ) -> Result<(Part, usize), AuthorityTemplateError> {
        let bytes = input.as_bytes();

        let start = cursor + 1;
        let mut end = start;

        let mut brace_count = 1;
        while end < bytes.len() {
            match bytes[end] {
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
            return Err(AuthorityTemplateError::UnbalancedBrace {
                authority: input.to_owned(),
                position: cursor,
            });
        }

        let content = &bytes[start..end];
        if content.is_empty() {
            return Err(AuthorityTemplateError::EmptyBraces {
                authority: input.to_owned(),
                position: cursor,
            });
        }

        let (name, constraint) = content
            .iter()
            .position(|&c| c == b':')
            .map_or((content, None), |colon_pos| {
                (&content[..colon_pos], Some(&content[colon_pos + 1..]))
            });

        if name.is_empty() {
            return Err(AuthorityTemplateError::EmptyParameter {
                authority: input.to_owned(),
                start: cursor,
                length: end - cursor + 1,
            });
        }

        let is_wildcard = name.starts_with(b"*");
        let name = if is_wildcard { &name[1..] } else { name };

        if is_wildcard && name.is_empty() {
            return Err(AuthorityTemplateError::EmptyWildcard {
                authority: input.to_owned(),
                start: cursor,
                length: end - cursor + 1,
            });
        }

        if name.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
            return Err(AuthorityTemplateError::InvalidParameter {
                authority: input.to_owned(),
                name: String::from_utf8_lossy(name).to_string(),
                start: cursor,
                length: end - cursor + 1,
            });
        }

        if let Some(constraint) = constraint {
            if constraint.is_empty() {
                return Err(AuthorityTemplateError::EmptyConstraint {
                    authority: input.to_owned(),
                    start: cursor,
                    length: end - cursor + 1,
                });
            }

            if constraint.iter().any(|&c| INVALID_PARAM_CHARS.contains(&c)) {
                return Err(AuthorityTemplateError::InvalidConstraint {
                    authority: input.to_owned(),
                    name: String::from_utf8_lossy(name).to_string(),
                    start: cursor,
                    length: end - cursor + 1,
                });
            }
        }

        let name = std::str::from_utf8(name)?.to_owned();
        let constraint = if let Some(constraint) = constraint {
            Some(std::str::from_utf8(constraint)?.to_owned())
        } else {
            None
        };

        let part = if is_wildcard {
            Part::Wildcard { name, constraint }
        } else {
            Part::Dynamic { name, constraint }
        };

        Ok((part, end + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use similar_asserts::assert_eq;

    #[test]
    fn test_parser_static_authority() {
        assert_eq!(
            ParsedAuthority::new("example.com"),
            Ok(ParsedAuthority {
                input: "example.com".into(),
                template: Template {
                    input: "example.com".into(),
                    raw: "example.com".into(),
                    parts: vec![Part::Static {
                        prefix: b"example.com".to_vec()
                    }],
                },
            }),
        );
    }

    #[test]
    fn test_parser_dynamic_authority() {
        assert_eq!(
            ParsedAuthority::new("{subdomain}.example.com"),
            Ok(ParsedAuthority {
                input: "{subdomain}.example.com".into(),
                template: Template {
                    input: "{subdomain}.example.com".into(),
                    raw: "{subdomain}.example.com".into(),
                    parts: vec![
                        Part::Static {
                            prefix: b".example.com".to_vec()
                        },
                        Part::Dynamic {
                            name: "subdomain".to_owned(),
                            constraint: None
                        },
                    ],
                },
            }),
        );
    }

    #[test]
    fn test_parser_wildcard_authority() {
        assert_eq!(
            ParsedAuthority::new("{*subdomains}.example.com"),
            Ok(ParsedAuthority {
                input: "{*subdomains}.example.com".into(),
                template: Template {
                    input: "{*subdomains}.example.com".into(),
                    raw: "{*subdomains}.example.com".into(),
                    parts: vec![
                        Part::Static {
                            prefix: b".example.com".to_vec()
                        },
                        Part::Wildcard {
                            name: "subdomains".to_owned(),
                            constraint: None
                        },
                    ],
                },
            }),
        );
    }

    #[test]
    fn test_parser_complex_authority() {
        assert_eq!(
            ParsedAuthority::new("{*wildcard}.{param:alpha}.example.com"),
            Ok(ParsedAuthority {
                input: "{*wildcard}.{param:alpha}.example.com".into(),
                template: Template {
                    input: "{*wildcard}.{param:alpha}.example.com".into(),
                    raw: "{*wildcard}.{param:alpha}.example.com".into(),
                    parts: vec![
                        Part::Static {
                            prefix: b".example.com".to_vec()
                        },
                        Part::Dynamic {
                            name: "param".to_owned(),
                            constraint: Some("alpha".to_owned())
                        },
                        Part::Static {
                            prefix: b".".to_vec()
                        },
                        Part::Wildcard {
                            name: "wildcard".to_owned(),
                            constraint: None
                        },
                    ],
                },
            }),
        );
    }

    #[test]
    fn test_parser_error_empty() {
        let error = ParsedAuthority::new("").unwrap_err();
        assert_eq!(error, AuthorityTemplateError::Empty);

        insta::assert_snapshot!(error, @"empty authority");
    }

    #[test]
    fn test_parser_error_empty_braces() {
        let error = ParsedAuthority::new("test.{}.com").unwrap_err();
        assert_eq!(
            error,
            AuthorityTemplateError::EmptyBraces {
                authority: "test.{}.com".to_owned(),
                position: 5,
            }
        );

        insta::assert_snapshot!(error, @r"
        empty braces

            Authority: test.{}.com
                            ^^
        ");
    }

    #[test]
    fn test_parser_error_unbalanced_brace_opening() {
        let error = ParsedAuthority::new("test.{param.com").unwrap_err();
        assert_eq!(
            error,
            AuthorityTemplateError::UnbalancedBrace {
                authority: "test.{param.com".to_owned(),
                position: 5,
            }
        );

        insta::assert_snapshot!(error, @r"
        unbalanced brace

            Authority: test.{param.com
                            ^

        tip: Use '\{' and '\}' to represent literal '{' and '}' characters in the authority
        ");
    }

    #[test]
    fn test_parser_error_empty_parameter() {
        let error = ParsedAuthority::new("test.{:constraint}.com").unwrap_err();
        assert_eq!(
            error,
            AuthorityTemplateError::EmptyParameter {
                authority: "test.{:constraint}.com".to_owned(),
                start: 5,
                length: 13,
            }
        );

        insta::assert_snapshot!(error, @r"
        empty parameter name

            Authority: test.{:constraint}.com
                            ^^^^^^^^^^^^^
        ");
    }

    #[test]
    fn test_parser_error_duplicate_parameter() {
        let error = ParsedAuthority::new("{param}.test.{param:alpha}.com").unwrap_err();
        assert_eq!(
            error,
            AuthorityTemplateError::DuplicateParameter {
                authority: "{param}.test.{param:alpha}.com".to_owned(),
                name: "param".to_owned(),
                first: 0,
                first_length: 7,
                second: 13,
                second_length: 13,
            }
        );

        insta::assert_snapshot!(error, @r"
        duplicate parameter name: 'param'

            Authority: {param}.test.{param:alpha}.com
                       ^^^^^^^      ^^^^^^^^^^^^^    

        tip: Parameter names must be unique within an authority
        ");
    }

    #[test]
    fn test_parser_error_touching_parameters() {
        let error = ParsedAuthority::new("{param1}{param2}.com").unwrap_err();
        assert_eq!(
            error,
            AuthorityTemplateError::TouchingParameters {
                authority: "{param1}{param2}.com".to_owned(),
                start: 0,
                length: 16,
            }
        );

        insta::assert_snapshot!(error, @r"
        touching parameters

            Authority: {param1}{param2}.com
                       ^^^^^^^^^^^^^^^^

        tip: Touching parameters are not supported
        ");
    }
}
