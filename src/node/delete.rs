use super::{Node, NodeKind};
use crate::{
    errors::delete::DeleteError,
    parts::{Part, Parts},
};
use regex::bytes::Regex;

impl<T> Node<T> {
    pub fn delete(&mut self, parts: &mut Parts<'_>) -> Result<(), DeleteError> {
        if let Some(segment) = parts.pop() {
            let result = match segment {
                Part::Static { prefix } => self.delete_static(parts, prefix),
                Part::Regex { name, pattern } => self.delete_regex(parts, name, &pattern),
                Part::Dynamic { name } => self.delete_dynamic(parts, name),
                Part::Wildcard { name } if parts.is_empty() => self.delete_end_wildcard(name),
                Part::Wildcard { name } => self.delete_wildcard(parts, name),
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
                // NOTE: This was previously a "starts_with" call, but turns out this is much faster.
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

    fn delete_regex(&mut self, parts: &mut Parts<'_>, name: &[u8], pattern: &Regex) -> Result<(), DeleteError> {
        let index = self
            .regex_children
            .iter()
            .position(|child| child.prefix == name && child.kind == NodeKind::Regex(pattern.clone()))
            .ok_or(DeleteError::NotFound)?;

        let child = &mut self.regex_children[index];
        let result = child.delete(parts);

        if result.is_ok() {
            child.optimize();

            if child.is_empty() {
                self.regex_children.remove(index);
            }
        }

        result
    }

    fn delete_dynamic(&mut self, parts: &mut Parts<'_>, name: &[u8]) -> Result<(), DeleteError> {
        let index = self
            .dynamic_children
            .iter()
            .position(|child| child.prefix == name)
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

    fn delete_wildcard(&mut self, parts: &mut Parts<'_>, name: &[u8]) -> Result<(), DeleteError> {
        let index = self
            .wildcard_children
            .iter()
            .position(|child| child.prefix == name)
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

    fn delete_end_wildcard(&mut self, name: &[u8]) -> Result<(), DeleteError> {
        if let Some(end_wildcard) = &self.end_wildcard {
            if end_wildcard.prefix == name {
                self.end_wildcard = None;
                return Ok(());
            }
        }

        Err(DeleteError::NotFound)
    }

    fn optimize(&mut self) {
        self.static_children
            .retain_mut(|child| {
                child.optimize();
                !child.is_empty()
            });

        self.regex_children.retain_mut(|child| {
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

        if let Some(end_wildcard) = &mut self.end_wildcard {
            if end_wildcard.is_empty() {
                self.end_wildcard = None;
            }
        }

        self.update_quicks();
    }

    pub(super) fn is_empty(&self) -> bool {
        self.data.is_none()
            && self.static_children.is_empty()
            && self.regex_children.is_empty()
            && self.dynamic_children.is_empty()
            && self.wildcard_children.is_empty()
            && self.end_wildcard.is_none()
    }
}
