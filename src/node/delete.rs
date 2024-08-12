use crate::{
    errors::delete::DeleteError,
    node::{Node, NodeConstraint},
    parts::Part,
    route::Route,
};

impl<T> Node<T> {
    pub fn delete(&mut self, route: &mut Route<'_>) -> Result<(), DeleteError> {
        if let Some(segment) = route.parts.pop() {
            let result = match segment {
                Part::Static { prefix } => self.delete_static(route, prefix),
                Part::Dynamic { name } => {
                    let constraint = route.constraints.remove(name);
                    self.delete_dynamic(route, name, constraint)
                }
                Part::Wildcard { name } if route.parts.is_empty() => {
                    let constraint = route.constraints.remove(name);
                    self.delete_end_wildcard(name, constraint)
                }
                Part::Wildcard { name } => {
                    let constraint = route.constraints.remove(name);
                    self.delete_wildcard(route, name, constraint)
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

            Err(DeleteError::NotFound)
        }
    }

    fn delete_static(&mut self, route: &mut Route<'_>, prefix: &[u8]) -> Result<(), DeleteError> {
        let index = self
            .static_children
            .iter()
            .position(|child| {
                prefix.len() >= child.prefix.len()
                    && child
                        .prefix
                        .iter()
                        .zip(prefix)
                        .all(|(a, b)| a == b)
            })
            .ok_or(DeleteError::NotFound)?;

        let child = &mut self.static_children[index];
        let remaining_prefix = &prefix[child.prefix.len()..];

        let result = if remaining_prefix.is_empty() {
            child.delete(route)
        } else {
            child.delete_static(route, remaining_prefix)
        };

        if result.is_ok() {
            child.optimize();

            if child.is_empty() {
                self.static_children.remove(index);
            }
        }

        result
    }

    #[allow(clippy::needless_pass_by_value)]
    fn delete_dynamic(
        &mut self,
        route: &mut Route<'_>,
        name: &[u8],

        constraint: Option<NodeConstraint>,
    ) -> Result<(), DeleteError> {
        let index = self
            .dynamic_children
            .iter()
            .position(|child| child.prefix == name && child.constraint == constraint)
            .ok_or(DeleteError::NotFound)?;

        let child = &mut self.dynamic_children[index];
        let result = child.delete(route);

        if result.is_ok() {
            child.optimize();

            if child.is_empty() {
                self.dynamic_children.remove(index);
            }
        }

        result
    }

    #[allow(clippy::needless_pass_by_value)]
    fn delete_wildcard(
        &mut self,
        route: &mut Route<'_>,
        name: &[u8],
        constraint: Option<NodeConstraint>,
    ) -> Result<(), DeleteError> {
        let index = self
            .wildcard_children
            .iter()
            .position(|child| child.prefix == name && child.constraint == constraint)
            .ok_or(DeleteError::NotFound)?;

        let child = &mut self.wildcard_children[index];
        let result = child.delete(route);

        if result.is_ok() {
            child.optimize();

            if child.is_empty() {
                self.wildcard_children.remove(index);
            }
        }

        result
    }

    #[allow(clippy::needless_pass_by_value)]
    fn delete_end_wildcard(&mut self, name: &[u8], constraint: Option<NodeConstraint>) -> Result<(), DeleteError> {
        let index = self
            .end_wildcard_children
            .iter()
            .position(|child| child.prefix == name && child.constraint == constraint)
            .ok_or(DeleteError::NotFound)?;

        self.end_wildcard_children.remove(index);
        Ok(())
    }

    fn optimize(&mut self) {
        self.static_children
            .retain_mut(|child| {
                child.optimize();
                !child.is_empty()
            });

        self.dynamic_children
            .retain_mut(|child| {
                child.optimize();
                !child.is_empty()
            });

        self.wildcard_children
            .retain_mut(|child| {
                child.optimize();
                !child.is_empty()
            });

        self.end_wildcard_children
            .retain_mut(|child| {
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
