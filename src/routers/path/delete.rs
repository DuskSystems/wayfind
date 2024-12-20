use super::{
    state::{State, StaticState},
    Node, PathData,
};
use crate::{
    errors::PathDeleteError,
    routers::path::parser::{ParsedRoute, Part},
};

impl<'r, S: State> Node<'r, S> {
    /// Deletes a route from the node tree.
    ///
    /// This method recursively traverses the tree to find and remove the specified route.
    /// Logic should match that used by the insert method.
    ///
    /// If the route is found and deleted, we re-optimize the tree structure.
    ///
    /// For expanded routes, we ensure that routes cannot be deleted individually, only as a group.
    pub fn delete(
        &mut self,
        route: &mut ParsedRoute,
        is_expanded: bool,
    ) -> Result<PathData<'r>, PathDeleteError> {
        if let Some(part) = route.parts.pop() {
            match part {
                Part::Static { prefix } => self.delete_static(route, is_expanded, &prefix),
                Part::Dynamic {
                    name, constraint, ..
                } => self.delete_dynamic(route, is_expanded, &name, constraint.as_ref()),
                Part::Wildcard {
                    name, constraint, ..
                } if route.parts.is_empty() => {
                    self.delete_end_wildcard(route, &name, constraint.as_ref())
                }
                Part::Wildcard {
                    name, constraint, ..
                } => self.delete_wildcard(route, is_expanded, &name, constraint.as_ref()),
            }
        } else {
            let Some(data) = &self.data else {
                return Err(PathDeleteError::NotFound {
                    route: String::from_utf8_lossy(&route.input).to_string(),
                });
            };

            if is_expanded != data.expanded.is_some() {
                return Err(PathDeleteError::RouteMismatch {
                    route: String::from_utf8_lossy(&route.input).to_string(),
                    inserted: data.route.to_owned(),
                });
            }

            let data = self.data.take().unwrap();
            self.needs_optimization = true;

            Ok(data)
        }
    }

    fn delete_static(
        &mut self,
        route: &mut ParsedRoute,
        is_expanded: bool,
        prefix: &[u8],
    ) -> Result<PathData<'r>, PathDeleteError> {
        let index = self
            .static_children
            .iter()
            .position(|child| {
                prefix.len() >= child.state.prefix.len()
                    && child.state.prefix.iter().zip(prefix).all(|(a, b)| a == b)
            })
            .ok_or_else(|| PathDeleteError::NotFound {
                route: String::from_utf8_lossy(&route.input).to_string(),
            })?;

        let child = &mut self.static_children[index];
        child.needs_optimization = true;

        let remaining_prefix = &prefix[child.state.prefix.len()..];
        let result = if remaining_prefix.is_empty() {
            child.delete(route, is_expanded)
        } else {
            child.delete_static(route, is_expanded, remaining_prefix)
        };

        if child.is_empty() {
            // Delete empty nodes.
            self.static_children.remove(index);
            self.needs_optimization = true;
        } else if child.is_compressible() {
            // Compress redundant nodes.
            let merge = child.static_children.remove(0);

            let mut prefix = std::mem::take(&mut child.state.prefix);
            prefix.extend(&merge.state.prefix);

            *child = Node {
                state: StaticState::new(prefix),
                needs_optimization: true,
                ..merge
            };
        }

        result
    }

    fn delete_dynamic(
        &mut self,
        route: &mut ParsedRoute,
        is_expanded: bool,
        name: &str,
        constraint: Option<&String>,
    ) -> Result<PathData<'r>, PathDeleteError> {
        let index = self
            .dynamic_children
            .iter()
            .position(|child| {
                child.state.name == name && child.state.constraint.as_ref() == constraint
            })
            .ok_or_else(|| PathDeleteError::NotFound {
                route: String::from_utf8_lossy(&route.input).to_string(),
            })?;

        let child = &mut self.dynamic_children[index];
        let result = child.delete(route, is_expanded);

        if child.is_empty() {
            self.dynamic_children.remove(index);
            self.needs_optimization = true;
        }

        result
    }

    fn delete_wildcard(
        &mut self,
        route: &mut ParsedRoute,
        is_expanded: bool,
        name: &str,
        constraint: Option<&String>,
    ) -> Result<PathData<'r>, PathDeleteError> {
        let index = self
            .wildcard_children
            .iter()
            .position(|child| {
                child.state.name == name && child.state.constraint.as_ref() == constraint
            })
            .ok_or_else(|| PathDeleteError::NotFound {
                route: String::from_utf8_lossy(&route.input).to_string(),
            })?;

        let child = &mut self.wildcard_children[index];
        let result = child.delete(route, is_expanded);

        if child.is_empty() {
            self.wildcard_children.remove(index);
            self.needs_optimization = true;
        }

        result
    }

    fn delete_end_wildcard(
        &mut self,
        route: &ParsedRoute,
        name: &str,
        constraint: Option<&String>,
    ) -> Result<PathData<'r>, PathDeleteError> {
        let index = self
            .end_wildcard_children
            .iter()
            .position(|child| {
                child.state.name == name && child.state.constraint.as_ref() == constraint
            })
            .ok_or_else(|| PathDeleteError::NotFound {
                route: String::from_utf8_lossy(&route.input).to_string(),
            })?;

        // FIXME: Should we check before remove?
        let mut child = self.end_wildcard_children.remove(index);
        if child.data.is_none() {
            return Err(PathDeleteError::NotFound {
                route: String::from_utf8_lossy(&route.input).to_string(),
            });
        };

        let data = child.data.take().unwrap();
        self.needs_optimization = true;

        Ok(data)
    }

    fn is_empty(&self) -> bool {
        self.data.is_none()
            && self.static_children.is_empty()
            && self.dynamic_children.is_empty()
            && self.wildcard_children.is_empty()
            && self.end_wildcard_children.is_empty()
    }

    fn is_compressible(&self) -> bool {
        self.data.is_none()
            && self.static_children.len() == 1
            && self.dynamic_children.is_empty()
            && self.wildcard_children.is_empty()
            && self.end_wildcard_children.is_empty()
    }
}
