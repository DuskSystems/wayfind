use crate::{
    errors::delete::DeleteError,
    node::Node,
    parts::{Part, Parts},
};

impl<T> Node<T> {
    pub fn delete(&mut self, parts: &mut Parts<'_>) -> Result<(), DeleteError> {
        if let Some(segment) = parts.pop() {
            let result = match segment {
                Part::Static { prefix } => self.delete_static(parts, prefix),
                Part::Dynamic { name, constraint } => self.delete_dynamic(parts, name, &constraint),
                Part::Wildcard { name, constraint } if parts.is_empty() => {
                    self.delete_end_wildcard(name, &constraint)
                }
                Part::Wildcard { name, constraint } => {
                    self.delete_wildcard(parts, name, &constraint)
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

    fn delete_static(&mut self, parts: &mut Parts<'_>, prefix: &[u8]) -> Result<(), DeleteError> {
        let index = self
            .static_children
            .iter()
            .position(|child| {
                prefix.len() >= child.prefix.len()
                    && child.prefix.iter().zip(prefix).all(|(a, b)| a == b)
            })
            .ok_or(DeleteError::NotFound)?;

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
        parts: &mut Parts<'_>,
        name: &[u8],
        constraint: &Option<Vec<u8>>,
    ) -> Result<(), DeleteError> {
        let index = self
            .dynamic_children
            .iter()
            .position(|child| child.prefix == name && child.constraint == *constraint)
            .ok_or(DeleteError::NotFound)?;

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
        parts: &mut Parts<'_>,
        name: &[u8],
        constraint: &Option<Vec<u8>>,
    ) -> Result<(), DeleteError> {
        let index = self
            .wildcard_children
            .iter()
            .position(|child| child.prefix == name && child.constraint == *constraint)
            .ok_or(DeleteError::NotFound)?;

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
        name: &[u8],
        constraint: &Option<Vec<u8>>,
    ) -> Result<(), DeleteError> {
        let index = self
            .end_wildcard_children
            .iter()
            .position(|child| child.prefix == name && child.constraint == *constraint)
            .ok_or(DeleteError::NotFound)?;

        self.end_wildcard_children.remove(index);
        Ok(())
    }

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
