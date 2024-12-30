use super::{
    node::Node,
    parser::{ParsedTemplate, Part},
    state::State,
    AuthorityData,
};

impl<'r, S: State> Node<'r, S> {
    pub(crate) fn find(&'r self, authority: &mut ParsedTemplate) -> Option<&'r AuthorityData<'r>> {
        if authority.parts.is_empty() {
            return self.data.as_ref();
        }

        if let Some(part) = authority.parts.pop() {
            return match part {
                Part::Static { prefix } => self.find_static(authority, &prefix),
                Part::Dynamic { name, constraint } => {
                    self.find_dynamic(authority, &name, constraint.as_deref())
                }
                Part::Wildcard { name, constraint } if authority.parts.is_empty() => {
                    self.find_end_wildcard(authority, &name, constraint.as_deref())
                }
                Part::Wildcard { name, constraint } => {
                    self.find_wildcard(authority, &name, constraint.as_deref())
                }
            };
        }

        None
    }

    fn find_static(
        &'r self,
        authority: &mut ParsedTemplate,
        prefix: &[u8],
    ) -> Option<&'r AuthorityData<'r>> {
        for child in self.static_children.iter() {
            if !child.state.prefix.is_empty() && child.state.prefix[0] == prefix[0] {
                let common_prefix = prefix
                    .iter()
                    .zip(&child.state.prefix)
                    .take_while(|&(x, y)| x == y)
                    .count();

                if common_prefix >= child.state.prefix.len() {
                    if common_prefix >= prefix.len() {
                        return child.find(authority);
                    }

                    let remaining = prefix[common_prefix..].to_vec();
                    if !remaining.is_empty() {
                        let mut new_authority = ParsedTemplate {
                            parts: authority.parts.clone(),
                            ..authority.clone()
                        };

                        new_authority.parts.push(Part::Static { prefix: remaining });
                        return child.find(&mut new_authority);
                    }
                }
            }
        }

        None
    }

    fn find_dynamic(
        &'r self,
        authority: &mut ParsedTemplate,
        name: &str,
        constraint: Option<&str>,
    ) -> Option<&'r AuthorityData<'r>> {
        for child in self.dynamic_children.iter() {
            if child.state.name == name && child.state.constraint.as_deref() == constraint {
                return child.find(authority);
            }
        }

        None
    }

    fn find_end_wildcard(
        &'r self,
        authority: &mut ParsedTemplate,
        name: &str,
        constraint: Option<&str>,
    ) -> Option<&'r AuthorityData<'r>> {
        for child in self.end_wildcard_children.iter() {
            if child.state.name == name && child.state.constraint.as_deref() == constraint {
                return child.find(authority);
            }
        }

        None
    }

    fn find_wildcard(
        &'r self,
        authority: &mut ParsedTemplate,
        name: &str,
        constraint: Option<&str>,
    ) -> Option<&'r AuthorityData<'r>> {
        for child in self.wildcard_children.iter() {
            if child.state.name == name && child.state.constraint.as_deref() == constraint {
                return child.find(authority);
            }
        }

        None
    }
}
