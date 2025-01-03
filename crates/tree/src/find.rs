use super::{
    node::Node,
    parser::{Part, Template},
    state::State,
};
use crate::node::Config;

impl<C: Config, S: State> Node<C, S> {
    pub fn find(&self, key: Option<usize>, route: &Template) -> Option<&C::Data> {
        self.find_at_position(key, route, route.parts.len())
    }

    fn find_at_position(
        &self,
        key: Option<usize>,
        route: &Template,
        position: usize,
    ) -> Option<&C::Data> {
        if position == 0 {
            return self.data.get(&key);
        }

        let part = &route.parts[position - 1];
        match part {
            Part::Static { prefix } => self.find_static(key, route, position - 1, prefix),
            Part::Dynamic { name, constraint } => {
                self.find_dynamic(key, route, position - 1, name, constraint.as_deref())
            }
            Part::Wildcard { name, constraint } if position == 1 => {
                self.find_end_wildcard(key, route, position - 1, name, constraint.as_deref())
            }
            Part::Wildcard { name, constraint } => {
                self.find_wildcard(key, route, position - 1, name, constraint.as_deref())
            }
        }
    }

    fn find_static(
        &self,
        key: Option<usize>,
        route: &Template,
        position: usize,
        prefix: &[u8],
    ) -> Option<&C::Data> {
        for child in self.static_children.iter() {
            if !child.state.prefix.is_empty() && child.state.prefix[0] == prefix[0] {
                let common_prefix = prefix
                    .iter()
                    .zip(&child.state.prefix)
                    .take_while(|&(x, y)| x == y)
                    .count();

                if common_prefix >= child.state.prefix.len() {
                    if common_prefix >= prefix.len() {
                        return child.find_at_position(key, route, position);
                    }

                    let remaining = prefix[common_prefix..].to_vec();
                    if !remaining.is_empty() {
                        let mut new_parts = route.parts[..position].to_vec();
                        new_parts.push(Part::Static { prefix: remaining });

                        let new_route = Template {
                            parts: new_parts,
                            ..route.clone()
                        };

                        return child.find_at_position(key, &new_route, position + 1);
                    }
                }
            }
        }

        None
    }

    fn find_dynamic(
        &self,
        key: Option<usize>,
        route: &Template,
        position: usize,
        name: &str,
        constraint: Option<&str>,
    ) -> Option<&C::Data> {
        for child in self.dynamic_children.iter() {
            if child.state.name == name && child.state.constraint.as_deref() == constraint {
                return child.find_at_position(key, route, position);
            }
        }

        None
    }

    fn find_end_wildcard(
        &self,
        key: Option<usize>,
        route: &Template,
        position: usize,
        name: &str,
        constraint: Option<&str>,
    ) -> Option<&C::Data> {
        for child in self.end_wildcard_children.iter() {
            if child.state.name == name && child.state.constraint.as_deref() == constraint {
                return child.find_at_position(key, route, position);
            }
        }

        None
    }

    fn find_wildcard(
        &self,
        key: Option<usize>,
        route: &Template,
        position: usize,
        name: &str,
        constraint: Option<&str>,
    ) -> Option<&C::Data> {
        for child in self.wildcard_children.iter() {
            if child.state.name == name && child.state.constraint.as_deref() == constraint {
                return child.find_at_position(key, route, position);
            }
        }

        None
    }
}
