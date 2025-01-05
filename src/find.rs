use crate::{
    node::{Node, NodeData},
    parser::{Part, Template},
    state::NodeState,
};

impl<'r, T, S: NodeState> Node<'r, T, S> {
    pub(crate) fn find(&'r self, template: &mut Template) -> Option<&'r NodeData<'r, T>> {
        if template.parts.is_empty() {
            return self.data.as_ref();
        }

        if let Some(part) = template.parts.pop() {
            return match part {
                Part::Static { prefix } => self.find_static(template, &prefix),
                Part::DynamicConstrained { name, constraint } => {
                    self.find_dynamic_constrained(template, &name, &constraint)
                }
                Part::Dynamic { name } => self.find_dynamic(template, &name),
                Part::WildcardConstrained { name, constraint } if template.parts.is_empty() => {
                    self.find_end_wildcard_constrained(template, &name, &constraint)
                }
                Part::Wildcard { name } if template.parts.is_empty() => {
                    self.find_end_wildcard(template, &name)
                }
                Part::WildcardConstrained { name, constraint } => {
                    self.find_wildcard_constrained(template, &name, &constraint)
                }
                Part::Wildcard { name } => self.find_wildcard(template, &name),
            };
        }

        None
    }

    fn find_static(
        &'r self,
        template: &mut Template,
        prefix: &[u8],
    ) -> Option<&'r NodeData<'r, T>> {
        for child in self.static_children.iter() {
            if !child.state.prefix.is_empty() && child.state.prefix[0] == prefix[0] {
                let common_prefix = prefix
                    .iter()
                    .zip(&child.state.prefix)
                    .take_while(|&(x, y)| x == y)
                    .count();

                if common_prefix >= child.state.prefix.len() {
                    if common_prefix >= prefix.len() {
                        return child.find(template);
                    }

                    let remaining = prefix[common_prefix..].to_vec();
                    if !remaining.is_empty() {
                        let mut new_template = Template {
                            parts: template.parts.clone(),
                            ..template.clone()
                        };

                        new_template.parts.push(Part::Static { prefix: remaining });
                        return child.find(&mut new_template);
                    }
                }
            }
        }

        None
    }

    fn find_dynamic_constrained(
        &'r self,
        template: &mut Template,
        name: &str,
        constraint: &str,
    ) -> Option<&'r NodeData<'r, T>> {
        for child in self.dynamic_constrained_children.iter() {
            if child.state.name == name && child.state.constraint == constraint {
                return child.find(template);
            }
        }

        None
    }

    fn find_dynamic(&'r self, template: &mut Template, name: &str) -> Option<&'r NodeData<'r, T>> {
        for child in self.dynamic_children.iter() {
            if child.state.name == name {
                return child.find(template);
            }
        }

        None
    }

    fn find_end_wildcard_constrained(
        &'r self,
        template: &mut Template,
        name: &str,
        constraint: &str,
    ) -> Option<&'r NodeData<'r, T>> {
        for child in self.end_wildcard_constrained_children.iter() {
            if child.state.name == name && child.state.constraint == constraint {
                return child.find(template);
            }
        }

        None
    }

    fn find_end_wildcard(
        &'r self,
        template: &mut Template,
        name: &str,
    ) -> Option<&'r NodeData<'r, T>> {
        for child in self.end_wildcard_children.iter() {
            if child.state.name == name {
                return child.find(template);
            }
        }

        None
    }

    fn find_wildcard_constrained(
        &'r self,
        template: &mut Template,
        name: &str,
        constraint: &str,
    ) -> Option<&'r NodeData<'r, T>> {
        for child in self.wildcard_constrained_children.iter() {
            if child.state.name == name && child.state.constraint == constraint {
                return child.find(template);
            }
        }

        None
    }

    fn find_wildcard(&'r self, template: &mut Template, name: &str) -> Option<&'r NodeData<'r, T>> {
        for child in self.wildcard_children.iter() {
            if child.state.name == name {
                return child.find(template);
            }
        }

        None
    }
}
