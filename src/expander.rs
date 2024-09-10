use crate::parser::{ParsedRoute, RoutePart, RouteParts};
use std::collections::VecDeque;

/// Represents a collection of expanded, simplified routes, derived from an original route.
#[derive(Debug, PartialEq, Eq)]
pub struct ExpandedRoutes {
    pub raw: ParsedRoute,
    pub routes: Vec<ParsedRoute>,
}

impl ExpandedRoutes {
    pub fn new(route: ParsedRoute) -> Self {
        let mut routes = VecDeque::new();
        Self::recursive_expand_optionals(route.parts.0.clone(), VecDeque::new(), &mut routes);

        let mut final_routes = Vec::new();
        for parts in routes {
            // Handle special case, where optional is at the start of a route.
            // Replace with a single "/" part.
            let parts = if parts
                .0
                .iter()
                .all(|part| matches!(part, RoutePart::Static { prefix } if prefix.is_empty()))
            {
                RouteParts(VecDeque::from(vec![RoutePart::Static {
                    prefix: b"/".to_vec(),
                }]))
            } else {
                parts
            };

            let base_route = ParsedRoute::from(parts);
            final_routes.push(base_route.clone());

            // Create an additional route with a hard-coded "/" at the end, if trailing slash is enabled.
            if route.trailing_slash {
                let mut with_slash = base_route;
                with_slash.raw.push(b'/');

                if let Some(end) = with_slash.parts.0.back() {
                    if end.ends_with_slash() {
                        continue;
                    }
                }

                if let Some(RoutePart::Static { prefix }) = with_slash.parts.0.back_mut() {
                    prefix.push(b'/');
                } else {
                    with_slash
                        .parts
                        .0
                        .push_back(RoutePart::Static { prefix: vec![b'/'] });
                }

                final_routes.push(with_slash);
            }
        }

        Self {
            raw: route,
            routes: final_routes,
        }
    }

    fn recursive_expand_optionals(
        mut remaining: VecDeque<RoutePart>,
        mut current: VecDeque<RoutePart>,
        expanded: &mut VecDeque<RouteParts>,
    ) {
        let Some(part) = remaining.pop_front() else {
            expanded.push_back(RouteParts(current));
            return;
        };

        match part {
            RoutePart::Static { .. }
            | RoutePart::Dynamic {
                optional: false, ..
            }
            | RoutePart::Wildcard {
                optional: false, ..
            } => {
                current.push_back(part);
                Self::recursive_expand_optionals(remaining, current, expanded);
            }
            RoutePart::Dynamic { .. } | RoutePart::Wildcard { .. } => {
                // Handle optional present case
                let mut new_part = part.clone();
                new_part.disable_optional();
                let mut new_route = current.clone();
                new_route.push_back(new_part);
                Self::recursive_expand_optionals(remaining.clone(), new_route, expanded);

                // Handle optional absent case
                // TODO: Consider calculating this at insert time.
                // Then we could handle this at the match level?
                let is_segment = matches!(&part, RoutePart::Static { .. })
                    || current.back().map_or(true, RoutePart::ends_with_slash)
                    || remaining.front().map_or(true, RoutePart::starts_with_slash);

                if is_segment {
                    // Trim the prior `/`, if exists.
                    if let Some(RoutePart::Static { prefix }) = current.back_mut() {
                        prefix.pop();
                    }

                    Self::recursive_expand_optionals(remaining, current, expanded);
                } else {
                    // Trim any prior static characters.
                    let mut current = current.clone();
                    while let Some(last) = current.back() {
                        if matches!(last, RoutePart::Static { prefix } if !prefix.is_empty() && !last.ends_with_slash())
                        {
                            current.pop_back();
                        } else {
                            break;
                        }
                    }

                    expanded.push_back(RouteParts(current));
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use similar_asserts::assert_eq;
    use std::error::Error;

    #[test]
    fn test_expander_optional_segment_simple() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/users/{id?}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/users/{id}")?,
                    ParsedRoute::new(b"/users")?
                ],
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_optional_segment_simple_suffix() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/users/{id?}/info")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/users/{id}/info")?,
                    ParsedRoute::new(b"/users/info")?
                ],
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_optional_segment_multiple() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/users/{id?}/{name?}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/users/{id}/{name}")?,
                    ParsedRoute::new(b"/users/{id}")?,
                    ParsedRoute::new(b"/users/{name}")?,
                    ParsedRoute::new(b"/users")?,
                ]
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_optional_segment_constraint() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/users/{id?:int}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/users/{id:int}")?,
                    ParsedRoute::new(b"/users")?,
                ]
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_optional_segment_all_optional() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/{category?}/{subcategory?}/{id?}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/{category}/{subcategory}/{id}")?,
                    ParsedRoute::new(b"/{category}/{subcategory}")?,
                    ParsedRoute::new(b"/{category}/{id}")?,
                    ParsedRoute::new(b"/{category}")?,
                    ParsedRoute::new(b"/{subcategory}/{id}")?,
                    ParsedRoute::new(b"/{subcategory}")?,
                    ParsedRoute::new(b"/{id}")?,
                    ParsedRoute::new(b"/")?,
                ]
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_optional_segment_mixed() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/api/{version}/{resource}/{id?}/details")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/api/{version}/{resource}/{id}/details")?,
                    ParsedRoute::new(b"/api/{version}/{resource}/details")?,
                ]
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_optional_inline_simple() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/files/{name}.{extension?}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/files/{name}.{extension}")?,
                    ParsedRoute::new(b"/files/{name}")?,
                ],
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_optional_inline_multiple() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/release/v{major}.{minor?}.{patch?}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/release/v{major}.{minor}.{patch}")?,
                    ParsedRoute::new(b"/release/v{major}.{minor}")?,
                    ParsedRoute::new(b"/release/v{major}")?,
                ],
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_optional_start() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"{id?}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![ParsedRoute::new(b"{id}")?, ParsedRoute::new(b"/")?,],
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_optional_start_slash() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/{id?}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![ParsedRoute::new(b"/{id}")?, ParsedRoute::new(b"/")?,],
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_slash() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/users{/}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![ParsedRoute::new(b"/users")?, ParsedRoute::new(b"/users/")?,],
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_slash_parameter() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/users/{id}{/}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/users/{id}")?,
                    ParsedRoute::new(b"/users/{id}/")?,
                ],
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_slash_optional() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/users/{id?}{/}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/users/{id}")?,
                    ParsedRoute::new(b"/users/{id}/")?,
                    ParsedRoute::new(b"/users")?,
                    ParsedRoute::new(b"/users/")?,
                ],
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_slash_optionals() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/api/{version}/{resource?}/{id?}{/}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/api/{version}/{resource}/{id}")?,
                    ParsedRoute::new(b"/api/{version}/{resource}/{id}/")?,
                    ParsedRoute::new(b"/api/{version}/{resource}")?,
                    ParsedRoute::new(b"/api/{version}/{resource}/")?,
                    ParsedRoute::new(b"/api/{version}/{id}")?,
                    ParsedRoute::new(b"/api/{version}/{id}/")?,
                    ParsedRoute::new(b"/api/{version}")?,
                    ParsedRoute::new(b"/api/{version}/")?,
                ],
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_slash_inline_optional() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/files/{name}.{extension?}{/}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/files/{name}.{extension}")?,
                    ParsedRoute::new(b"/files/{name}.{extension}/")?,
                    ParsedRoute::new(b"/files/{name}")?,
                    ParsedRoute::new(b"/files/{name}/")?,
                ],
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_slash_optional_start() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"{id?}{/}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"{id}")?,
                    ParsedRoute::new(b"{id}/")?,
                    ParsedRoute::new(b"/")?,
                ],
            }
        );

        Ok(())
    }

    #[test]
    fn test_expander_slash_optional_start_slash() -> Result<(), Box<dyn Error>> {
        let route = ParsedRoute::new(b"/{id?}{/}")?;
        let expanded = ExpandedRoutes::new(route.clone());

        assert_eq!(
            expanded,
            ExpandedRoutes {
                raw: route,
                routes: vec![
                    ParsedRoute::new(b"/{id}")?,
                    ParsedRoute::new(b"/{id}/")?,
                    ParsedRoute::new(b"/")?,
                ],
            }
        );

        Ok(())
    }
}
