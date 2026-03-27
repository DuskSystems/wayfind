use alloc::vec::Vec;

use crate::node::{Node, NodeData};
use crate::parser::{Part, Template};
use crate::state::StaticState;

impl<S> Node<S> {
    /// Deletes a route from the node tree.
    /// Recursively traverses the tree to find and remove the specified template.
    pub(crate) fn delete(&mut self, template: &mut Template) -> Option<NodeData> {
        if let Some(part) = template.parts.pop() {
            match part {
                Part::Static { prefix } => self.delete_static(template, &prefix),
                Part::Dynamic { name } => self.delete_dynamic(template, &name),
                Part::Wildcard { name } if template.parts.is_empty() => {
                    self.delete_end_wildcard(&name)
                }
                Part::Wildcard { name } => self.delete_wildcard(template, &name),
            }
        } else {
            let data = self.data.take()?;
            self.flags.set_needs_optimization(true);
            Some(data)
        }
    }

    fn delete_static(&mut self, template: &mut Template, prefix: &[u8]) -> Option<NodeData> {
        let index = self.static_children.iter().position(|child| {
            prefix.len() >= child.state.prefix.len()
                && child.state.prefix.iter().zip(prefix).all(|(a, b)| a == b)
        })?;

        let child = &mut self.static_children[index];
        child.flags.set_needs_optimization(true);

        let child_prefix_len = child.state.prefix.len();
        let remaining_prefix = &prefix[child_prefix_len..];
        let result = if remaining_prefix.is_empty() {
            child.delete(template)
        } else {
            child.delete_static(template, remaining_prefix)
        };

        if child.is_empty() {
            self.static_children.remove(index);
            self.flags.set_needs_optimization(true);
        } else if child.is_compressible() {
            let child = &mut self.static_children[index];
            let merge = child.static_children.remove(0);

            let mut combined: Vec<u8> = core::mem::take(&mut child.state.prefix).into_vec();
            combined.extend_from_slice(&merge.state.prefix);

            let mut flags = merge.flags;
            flags.set_needs_optimization(true);

            let child = &mut self.static_children[index];
            child.state = StaticState::new(combined);
            child.flags = flags;
            child.data = merge.data;
            child.static_children = merge.static_children;
            child.dynamic_children = merge.dynamic_children;
            child.wildcard_children = merge.wildcard_children;
            child.end_wildcard = merge.end_wildcard;
            child.shortest = merge.shortest;
            child.longest = merge.longest;
            child.tails = merge.tails;
            child.reachable = merge.reachable;
            child.suffixes = merge.suffixes;
            child.id = merge.id;
        }

        result
    }

    fn delete_dynamic(&mut self, template: &mut Template, name: &str) -> Option<NodeData> {
        let index = self
            .dynamic_children
            .iter()
            .position(|child| *child.state.name == *name)?;

        let child = &mut self.dynamic_children[index];
        let result = child.delete(template);

        if child.is_empty() {
            self.dynamic_children.remove(index);
            self.flags.set_needs_optimization(true);
        }

        result
    }

    fn delete_wildcard(&mut self, template: &mut Template, name: &str) -> Option<NodeData> {
        let index = self
            .wildcard_children
            .iter()
            .position(|child| *child.state.name == *name)?;

        let child = &mut self.wildcard_children[index];
        let result = child.delete(template);

        if child.is_empty() {
            self.wildcard_children.remove(index);
            self.flags.set_needs_optimization(true);
        }

        result
    }

    fn delete_end_wildcard(&mut self, name: &str) -> Option<NodeData> {
        let child = self.end_wildcard.as_deref()?;
        if *child.state.name != *name {
            return None;
        }

        let child = self.end_wildcard.take()?;
        self.flags.set_needs_optimization(true);

        child.data
    }

    #[must_use]
    const fn is_empty(&self) -> bool {
        self.data.is_none()
            && self.static_children.is_empty()
            && self.dynamic_children.is_empty()
            && self.wildcard_children.is_empty()
            && self.end_wildcard.is_none()
    }

    #[must_use]
    const fn is_compressible(&self) -> bool {
        self.data.is_none()
            && self.static_children.len() == 1
            && self.dynamic_children.is_empty()
            && self.wildcard_children.is_empty()
            && self.end_wildcard.is_none()
    }
}
