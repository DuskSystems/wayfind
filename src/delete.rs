use std::sync::Arc;

use crate::{
    node::{Node, NodeData},
    parser::{Part, Template},
    state::{NodeState, StaticState},
};

impl<T, S: NodeState> Node<'_, T, S> {
    /// Deletes a route from the node tree.
    ///
    /// This method recursively traverses the tree to find and remove the specified template.
    /// Logic should match that used by the insert method.
    ///
    /// If the route is found and deleted, we re-optimize the tree structure.
    pub fn delete(&mut self, template: &mut Template) -> Option<T> {
        if let Some(part) = template.parts.pop() {
            match part {
                Part::Static { prefix } => self.delete_static(template, &prefix),
                Part::Dynamic {
                    name, constraint, ..
                } => self.delete_dynamic(template, &name, constraint.as_ref()),
                Part::Wildcard {
                    name, constraint, ..
                } if template.parts.is_empty() => {
                    self.delete_end_wildcard(&name, constraint.as_ref())
                }
                Part::Wildcard {
                    name, constraint, ..
                } => self.delete_wildcard(template, &name, constraint.as_ref()),
            }
        } else {
            let data = self.data.take()?;
            self.needs_optimization = true;

            match data {
                NodeData::Inline { data, .. } => Some(data),
                NodeData::Shared { data, .. } => Arc::try_unwrap(data).ok(),
            }
        }
    }

    fn delete_static(&mut self, template: &mut Template, prefix: &[u8]) -> Option<T> {
        let index = self.static_children.iter().position(|child| {
            prefix.len() >= child.state.prefix.len()
                && child.state.prefix.iter().zip(prefix).all(|(a, b)| a == b)
        })?;

        let child = &mut self.static_children[index];
        child.needs_optimization = true;

        let remaining_prefix = &prefix[child.state.prefix.len()..];
        let result = if remaining_prefix.is_empty() {
            child.delete(template)
        } else {
            child.delete_static(template, remaining_prefix)
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
        template: &mut Template,
        name: &str,
        constraint: Option<&String>,
    ) -> Option<T> {
        let index = self.dynamic_children.iter().position(|child| {
            child.state.name == name && child.state.constraint.as_ref() == constraint
        })?;

        let child = &mut self.dynamic_children[index];
        let result = child.delete(template);

        if child.is_empty() {
            self.dynamic_children.remove(index);
            self.needs_optimization = true;
        }

        result
    }

    fn delete_wildcard(
        &mut self,
        template: &mut Template,
        name: &str,
        constraint: Option<&String>,
    ) -> Option<T> {
        let index = self.wildcard_children.iter().position(|child| {
            child.state.name == name && child.state.constraint.as_ref() == constraint
        })?;

        let child = &mut self.wildcard_children[index];
        let result = child.delete(template);

        if child.is_empty() {
            self.wildcard_children.remove(index);
            self.needs_optimization = true;
        }

        result
    }

    fn delete_end_wildcard(&mut self, name: &str, constraint: Option<&String>) -> Option<T> {
        let index = self.end_wildcard_children.iter().position(|child| {
            child.state.name == name && child.state.constraint.as_ref() == constraint
        })?;

        let mut child = self.end_wildcard_children.remove(index);

        let data = child.data.take()?;
        self.needs_optimization = true;

        match data {
            NodeData::Inline { data, .. } => Some(data),
            NodeData::Shared { data, .. } => Arc::try_unwrap(data).ok(),
        }
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
