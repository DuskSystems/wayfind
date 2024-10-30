use super::Node;
use crate::parser::{Part, Route};

impl Node {
    /// Finds an exact node matching the route.
    /// Follows the same traversal logic as insert.
    pub fn find<'router>(&'router self, route: &mut Route) -> Option<&'router Self> {
        if let Some(part) = route.parts.pop() {
            match part {
                Part::Static { prefix } => self.find_static(route, &prefix),
                Part::Dynamic {
                    name, constraint, ..
                } => self.find_dynamic(route, &name, &constraint),
                Part::Wildcard {
                    name, constraint, ..
                } if route.parts.is_empty() => self.find_end_wildcard(&name, &constraint),
                Part::Wildcard {
                    name, constraint, ..
                } => self.find_wildcard(route, &name, &constraint),
            }
        } else if self.data.is_some() {
            Some(self)
        } else {
            None
        }
    }

    fn find_static<'router>(
        &'router self,
        route: &mut Route,
        prefix: &[u8],
    ) -> Option<&'router Self> {
        let child = self
            .static_children
            .iter()
            .find(|child| child.prefix[0] == prefix[0])?;

        let common_prefix = prefix
            .iter()
            .zip(&child.prefix)
            .take_while(|&(x, y)| x == y)
            .count();

        if common_prefix >= child.prefix.len() {
            if common_prefix >= prefix.len() {
                child.find(route)
            } else {
                child.find_static(route, &prefix[common_prefix..])
            }
        } else {
            None
        }
    }

    fn find_dynamic<'router>(
        &'router self,
        route: &mut Route,
        name: &[u8],
        constraint: &Option<Vec<u8>>,
    ) -> Option<&'router Self> {
        self.dynamic_children
            .iter()
            .find(|child| child.prefix == name && child.constraint == *constraint)
            .and_then(|child| child.find(route))
    }

    fn find_wildcard<'router>(
        &'router self,
        route: &mut Route,
        name: &[u8],
        constraint: &Option<Vec<u8>>,
    ) -> Option<&'router Self> {
        self.wildcard_children
            .iter()
            .find(|child| child.prefix == name && child.constraint == *constraint)
            .and_then(|child| child.find(route))
    }

    fn find_end_wildcard<'router>(
        &'router self,
        name: &[u8],
        constraint: &Option<Vec<u8>>,
    ) -> Option<&'router Self> {
        self.end_wildcard_children
            .iter()
            .find(|child| child.prefix == name && child.constraint == *constraint)
            .and_then(|child| {
                if child.data.is_some() {
                    Some(child)
                } else {
                    None
                }
            })
    }
}
