use crate::{
    errors::DeleteError,
    node::Node,
    parser::{ParsedRoute, RoutePart},
};

impl<T> Node<T> {
    /// Deletes a route from the node tree.
    ///
    /// This method recursively traverses the tree to find and remove the specified route.
    /// Logic should match that used by the insert method.
    ///
    /// If the route is found and deleted, we re-optimize the tree structure.
    pub fn delete(&mut self, parts: &mut ParsedRoute<'_>) -> Result<(), DeleteError> {
        if let Some(segment) = parts.pop() {
            let result = match segment {
                RoutePart::Static { prefix } => self.delete_static(parts, &prefix),
                RoutePart::Dynamic { name, constraint } => {
                    self.delete_dynamic(parts, &name, &constraint)
                }
                RoutePart::Wildcard { name, constraint } if parts.is_empty() => {
                    self.delete_end_wildcard(parts, &name, &constraint)
                }
                RoutePart::Wildcard { name, constraint } => {
                    self.delete_wildcard(parts, &name, &constraint)
                }
            };

            if result.is_ok() {
                self.optimize();
            }

            result
        } else {
            if self.data.take().is_some() {
                self.optimize();
                return Ok(());
            }

            Err(DeleteError::NotFound {
                route: String::from_utf8_lossy(parts.route).to_string(),
            })
        }
    }

    fn delete_static(
        &mut self,
        parts: &mut ParsedRoute<'_>,
        prefix: &[u8],
    ) -> Result<(), DeleteError> {
        let index = self
            .static_children
            .iter()
            .position(|child| {
                prefix.len() >= child.prefix.len()
                    && child.prefix.iter().zip(prefix).all(|(a, b)| a == b)
            })
            .ok_or(DeleteError::NotFound {
                route: String::from_utf8_lossy(parts.route).to_string(),
            })?;

        let child = &mut self.static_children[index];
        let remaining_prefix = &prefix[child.prefix.len()..];

        let result = if remaining_prefix.is_empty() {
            child.delete(parts)
        } else {
            child.delete_static(parts, remaining_prefix)
        };

        if result.is_ok() {
            child.optimize();

            if child.is_empty() {
                self.static_children.remove(index);
            }
        }

        result
    }

    fn delete_dynamic(
        &mut self,
        parts: &mut ParsedRoute<'_>,
        name: &[u8],
        constraint: &Option<Vec<u8>>,
    ) -> Result<(), DeleteError> {
        let index = self
            .dynamic_children
            .iter()
            .position(|child| child.prefix == name && child.constraint == *constraint)
            .ok_or(DeleteError::NotFound {
                route: String::from_utf8_lossy(parts.route).to_string(),
            })?;

        let child = &mut self.dynamic_children[index];
        let result = child.delete(parts);

        if result.is_ok() {
            child.optimize();

            if child.is_empty() {
                self.dynamic_children.remove(index);
            }
        }

        result
    }

    fn delete_wildcard(
        &mut self,
        parts: &mut ParsedRoute<'_>,
        name: &[u8],
        constraint: &Option<Vec<u8>>,
    ) -> Result<(), DeleteError> {
        let index = self
            .wildcard_children
            .iter()
            .position(|child| child.prefix == name && child.constraint == *constraint)
            .ok_or(DeleteError::NotFound {
                route: String::from_utf8_lossy(parts.route).to_string(),
            })?;

        let child = &mut self.wildcard_children[index];
        let result = child.delete(parts);

        if result.is_ok() {
            child.optimize();

            if child.is_empty() {
                self.wildcard_children.remove(index);
            }
        }

        result
    }

    fn delete_end_wildcard(
        &mut self,
        parts: &ParsedRoute<'_>,
        name: &[u8],
        constraint: &Option<Vec<u8>>,
    ) -> Result<(), DeleteError> {
        let index = self
            .end_wildcard_children
            .iter()
            .position(|child| child.prefix == name && child.constraint == *constraint)
            .ok_or(DeleteError::NotFound {
                route: String::from_utf8_lossy(parts.route).to_string(),
            })?;

        self.end_wildcard_children.remove(index);
        Ok(())
    }

    /// Re-optimize the tree after a deletion.
    ///
    /// This method removes empty children, then updates quick search flags.
    fn optimize(&mut self) {
        self.static_children.retain_mut(|child| {
            child.optimize();
            !child.is_empty()
        });

        self.dynamic_children.retain_mut(|child| {
            child.optimize();
            !child.is_empty()
        });

        self.wildcard_children.retain_mut(|child| {
            child.optimize();
            !child.is_empty()
        });

        self.end_wildcard_children.retain_mut(|child| {
            child.optimize();
            !child.is_empty()
        });

        self.update_quicks();
    }

    pub(super) fn is_empty(&self) -> bool {
        self.data.is_none()
            && self.static_children.is_empty()
            && self.dynamic_children.is_empty()
            && self.wildcard_children.is_empty()
            && self.end_wildcard_children.is_empty()
    }
}
