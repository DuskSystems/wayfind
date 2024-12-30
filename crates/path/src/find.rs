use super::{
    node::Node,
    parser::{ParsedTemplate, Part},
    state::State,
    PathData,
};

impl<'r, S: State> Node<'r, S> {
    pub(crate) fn find(&'r self, route: &mut ParsedTemplate) -> Option<&'r PathData<'r>> {
        if route.parts.is_empty() {
            return self.data.as_ref();
        }

        if let Some(part) = route.parts.pop() {
            return match part {
                Part::Static { prefix } => self.find_static(route, &prefix),
                Part::Dynamic { name, constraint } => {
                    self.find_dynamic(route, &name, constraint.as_deref())
                }
                Part::Wildcard { name, constraint } if route.parts.is_empty() => {
                    self.find_end_wildcard(route, &name, constraint.as_deref())
                }
                Part::Wildcard { name, constraint } => {
                    self.find_wildcard(route, &name, constraint.as_deref())
                }
            };
        }

        None
    }

    fn find_static(
        &'r self,
        route: &mut ParsedTemplate,
        prefix: &[u8],
    ) -> Option<&'r PathData<'r>> {
        for child in self.static_children.iter() {
            if !child.state.prefix.is_empty() && child.state.prefix[0] == prefix[0] {
                let common_prefix = prefix
                    .iter()
                    .zip(&child.state.prefix)
                    .take_while(|&(x, y)| x == y)
                    .count();

                if common_prefix >= child.state.prefix.len() {
                    if common_prefix >= prefix.len() {
                        return child.find(route);
                    }

                    let remaining = prefix[common_prefix..].to_vec();
                    if !remaining.is_empty() {
                        let mut new_route = ParsedTemplate {
                            parts: route.parts.clone(),
                            ..route.clone()
                        };

                        new_route.parts.push(Part::Static { prefix: remaining });
                        return child.find(&mut new_route);
                    }
                }
            }
        }

        None
    }

    fn find_dynamic(
        &'r self,
        route: &mut ParsedTemplate,
        name: &str,
        constraint: Option<&str>,
    ) -> Option<&'r PathData<'r>> {
        for child in self.dynamic_children.iter() {
            if child.state.name == name && child.state.constraint.as_deref() == constraint {
                return child.find(route);
            }
        }

        None
    }

    fn find_end_wildcard(
        &'r self,
        route: &mut ParsedTemplate,
        name: &str,
        constraint: Option<&str>,
    ) -> Option<&'r PathData<'r>> {
        for child in self.end_wildcard_children.iter() {
            if child.state.name == name && child.state.constraint.as_deref() == constraint {
                return child.find(route);
            }
        }

        None
    }

    fn find_wildcard(
        &'r self,
        route: &mut ParsedTemplate,
        name: &str,
        constraint: Option<&str>,
    ) -> Option<&'r PathData<'r>> {
        for child in self.wildcard_children.iter() {
            if child.state.name == name && child.state.constraint.as_deref() == constraint {
                return child.find(route);
            }
        }

        None
    }
}
