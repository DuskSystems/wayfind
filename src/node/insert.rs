use alloc::boxed::Box;
use alloc::vec;

use crate::node::{Data, Node};
use crate::parser::{Part, Template};
use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};

impl<S, T> Node<S, T> {
    /// Inserts a new route into the node tree with associated data.
    /// Recursively traverses the node tree, creating new nodes as necessary.
    pub(crate) fn insert(&mut self, template: &mut Template<'_>, data: Data<T>) {
        if let Some(part) = template.parts.pop() {
            match part {
                Part::Static { prefix } => self.insert_static(template, data, prefix),
                Part::Dynamic { name } => self.insert_dynamic(template, data, name),
                Part::Wildcard { name } if template.parts.is_empty() => {
                    self.insert_end_wildcard(data, name);
                }
                Part::Wildcard { name } => self.insert_wildcard(template, data, name),
            }
        } else {
            self.data = Some(data);
            self.flags.set_needs_optimization(true);
        }
    }

    fn insert_static(&mut self, template: &mut Template<'_>, data: Data<T>, prefix: &[u8]) {
        if let Some(child) = self
            .static_children
            .iter_mut()
            .find(|child| child.state.prefix[0] == prefix[0])
        {
            let common_prefix = prefix
                .iter()
                .zip(&child.state.prefix)
                .take_while(|&(a, b)| a == b)
                .count();

            // If the new prefix matches or extends the existing prefix, insert directly.
            if common_prefix >= child.state.prefix.len() {
                if common_prefix >= prefix.len() {
                    child.insert(template, data);
                } else {
                    child.insert_static(template, data, &prefix[common_prefix..]);
                }

                self.flags.set_needs_optimization(true);
                return;
            }

            // Not a clean insert, need to split the existing child node.
            let new_child_a = Node {
                state: StaticState::new(&child.state.prefix[common_prefix..]),
                data: child.data.take(),

                static_children: core::mem::take(&mut child.static_children),
                dynamic_children: core::mem::take(&mut child.dynamic_children),
                wildcard_children: core::mem::take(&mut child.wildcard_children),
                end_wildcard: core::mem::take(&mut child.end_wildcard),

                flags: child.flags.clone(),
                bounds: child.bounds.clone(),
                reachable: core::mem::take(&mut child.reachable),
                suffixes: core::mem::take(&mut child.suffixes),
            };

            let new_child_b = Node::new(StaticState::new(&prefix[common_prefix..]));

            child.state = StaticState::new(&child.state.prefix[..common_prefix]);
            child.flags.set_needs_optimization(true);

            if prefix[common_prefix..].is_empty() {
                child.static_children = vec![new_child_a];
                child.insert(template, data);
            } else {
                child.static_children = vec![new_child_a, new_child_b];
                child.static_children[1].insert(template, data);
            }

            self.flags.set_needs_optimization(true);
            return;
        }

        let mut child = Node::new(StaticState::new(prefix));
        child.insert(template, data);
        self.static_children.push(child);

        self.flags.set_needs_optimization(true);
    }

    fn insert_dynamic(&mut self, template: &mut Template<'_>, data: Data<T>, name: &str) {
        if let Some(child) = self
            .dynamic_children
            .iter_mut()
            .find(|child| *child.state.name == *name)
        {
            child.insert(template, data);
        } else {
            let mut child = Node::new(DynamicState::new(name));
            child.insert(template, data);
            self.dynamic_children.push(child);
        }

        self.flags.set_needs_optimization(true);
    }

    fn insert_wildcard(&mut self, template: &mut Template<'_>, data: Data<T>, name: &str) {
        if let Some(child) = self
            .wildcard_children
            .iter_mut()
            .find(|child| *child.state.name == *name)
        {
            child.insert(template, data);
        } else {
            let mut child = Node::new(WildcardState::new(name));
            child.insert(template, data);
            self.wildcard_children.push(child);
        }

        self.flags.set_needs_optimization(true);
    }

    fn insert_end_wildcard(&mut self, data: Data<T>, name: &str) {
        let mut node = Node::new(EndWildcardState::new(name));
        node.data = Some(data);
        self.end_wildcard = Some(Box::new(node));
        self.flags.set_needs_optimization(true);
    }
}
