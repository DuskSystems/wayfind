use crate::{
    errors::DeleteError,
    node::Node,
    parser::{Part, Route},
};

impl<'router, T> Node<'router, T> {
    /// Deletes a route from the node tree.
    ///
    /// This method recursively traverses the tree to find and remove the specified route.
    /// Logic should match that used by the insert method.
    ///
    /// If the route is found and deleted, we re-optimize the tree structure.
    ///
    /// For expanded routes, we ensure that routes cannot be deleted individually, only as a group.
    pub fn delete(&mut self, route: &mut Route<'router>) -> Result<(), DeleteError> {
        if let Some(part) = route.parts.pop() {
            match part {
                Part::Static { prefix } => self.delete_static(route, prefix),
                Part::Dynamic { name, .. } => self.delete_dynamic(route, name),
                Part::Wildcard { name, .. } if route.parts.is_empty() => {
                    self.delete_end_wildcard(route, name)
                }
                Part::Wildcard { name, .. } => self.delete_wildcard(route, name),
            }
        } else {
            if self.data.is_none() {
                return Err(DeleteError::NotFound {
                    route: String::from_utf8_lossy(route.raw).to_string(),
                });
            }

            self.data = None;
            self.needs_optimization = true;

            Ok(())
        }
    }

    fn delete_static(
        &mut self,
        route: &mut Route<'router>,
        prefix: &[u8],
    ) -> Result<(), DeleteError> {
        let index = self
            .static_children
            .iter()
            .position(|child| {
                prefix.len() >= child.prefix.len()
                    && child.prefix.iter().zip(prefix).all(|(a, b)| a == b)
            })
            .ok_or_else(|| DeleteError::NotFound {
                route: String::from_utf8_lossy(route.raw).to_string(),
            })?;

        let child = &mut self.static_children[index];
        child.needs_optimization = true;

        let remaining_prefix = &prefix[child.prefix.len()..];

        let result = if remaining_prefix.is_empty() {
            child.delete(route)
        } else {
            child.delete_static(route, remaining_prefix)
        };

        if child.is_empty() {
            self.static_children.remove(index);
            self.needs_optimization = true;
        }

        result
    }

    fn delete_dynamic(
        &mut self,
        route: &mut Route<'router>,
        name: &[u8],
    ) -> Result<(), DeleteError> {
        let index = self
            .dynamic_children
            .iter()
            .position(|child| child.prefix == name)
            .ok_or_else(|| DeleteError::NotFound {
                route: String::from_utf8_lossy(route.raw).to_string(),
            })?;

        let child = &mut self.dynamic_children[index];
        let result = child.delete(route);

        if child.is_empty() {
            self.dynamic_children.remove(index);
            self.needs_optimization = true;
        }

        result
    }

    fn delete_wildcard(
        &mut self,
        route: &mut Route<'router>,
        name: &[u8],
    ) -> Result<(), DeleteError> {
        let index = self
            .wildcard_children
            .iter()
            .position(|child| child.prefix == name)
            .ok_or_else(|| DeleteError::NotFound {
                route: String::from_utf8_lossy(route.raw).to_string(),
            })?;

        let child = &mut self.wildcard_children[index];
        let result = child.delete(route);

        if child.is_empty() {
            self.wildcard_children.remove(index);
            self.needs_optimization = true;
        }

        result
    }

    fn delete_end_wildcard(
        &mut self,
        route: &Route<'router>,
        name: &[u8],
    ) -> Result<(), DeleteError> {
        let index = self
            .end_wildcard_children
            .iter()
            .position(|child| child.prefix == name)
            .ok_or_else(|| DeleteError::NotFound {
                route: String::from_utf8_lossy(route.raw).to_string(),
            })?;

        self.end_wildcard_children.remove(index);
        self.needs_optimization = true;

        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.data.is_none()
            && self.static_children.is_empty()
            && self.dynamic_children.is_empty()
            && self.wildcard_children.is_empty()
            && self.end_wildcard_children.is_empty()
    }
}
