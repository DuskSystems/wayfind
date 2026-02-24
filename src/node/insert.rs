use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;

use crate::node::{Node, NodeData};
use crate::parser::{Part, Template};
use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};

impl<S> Node<S> {
    /// Inserts a new route into the node tree with associated data.
    /// Recursively traverses the node tree, creating new nodes as necessary.
    pub fn insert(&mut self, template: &mut Template, data: NodeData) {
        if let Some(part) = template.parts.pop() {
            match part {
                Part::Static { prefix } => self.insert_static(template, data, &prefix),
                Part::Dynamic { name } => self.insert_dynamic(template, data, name),
                Part::Wildcard { name } if template.parts.is_empty() => {
                    self.insert_end_wildcard(data, name);
                }
                Part::Wildcard { name } => self.insert_wildcard(template, data, name),
            }
        } else {
            self.data = Some(data);
            self.needs_optimization = true;
        }
    }

    fn insert_static(&mut self, template: &mut Template, data: NodeData, prefix: &[u8]) {
        if let Some(child) = self
            .static_children
            .iter_mut()
            .find(|child| child.state.prefix[0] == prefix[0])
        {
            let common_prefix = prefix
                .iter()
                .zip(&child.state.prefix)
                .take_while(|&(x, y)| x == y)
                .count();

            // If the new prefix matches or extends the existing prefix, we can just insert it directly.
            if common_prefix >= child.state.prefix.len() {
                if common_prefix >= prefix.len() {
                    child.insert(template, data);
                } else {
                    child.insert_static(template, data, &prefix[common_prefix..]);
                }

                self.needs_optimization = true;
                return;
            }

            // Not a clean insert, need to split the existing child node.
            let new_child_a = Node {
                state: StaticState::new(child.state.prefix[common_prefix..].to_vec()),
                data: child.data.take(),

                static_children: core::mem::take(&mut child.static_children),
                dynamic_children: core::mem::take(&mut child.dynamic_children),
                wildcard_children: core::mem::take(&mut child.wildcard_children),
                end_wildcard: core::mem::take(&mut child.end_wildcard),

                dynamic_segment_only: child.dynamic_segment_only,
                wildcard_segment_only: child.wildcard_segment_only,
                shortest: child.shortest,
                longest: child.longest,
                tails: core::mem::take(&mut child.tails),

                needs_optimization: child.needs_optimization,
            };

            let new_child_b = Node::new(StaticState::new(prefix[common_prefix..].to_vec()));

            child.state = StaticState::new(child.state.prefix[..common_prefix].to_vec());
            child.needs_optimization = true;

            if prefix[common_prefix..].is_empty() {
                child.static_children = vec![new_child_a];
                child.insert(template, data);
            } else {
                child.static_children = vec![new_child_a, new_child_b];
                child.static_children[1].insert(template, data);
            }

            self.needs_optimization = true;
            return;
        }

        self.static_children.push({
            let mut new_child = Node::new(StaticState::new(prefix.to_vec()));
            new_child.insert(template, data);
            new_child
        });

        self.needs_optimization = true;
    }

    fn insert_dynamic(&mut self, template: &mut Template, data: NodeData, name: String) {
        if let Some(child) = self
            .dynamic_children
            .iter_mut()
            .find(|child| child.state.name == name)
        {
            child.insert(template, data);
        } else {
            self.dynamic_children.push({
                let mut new_child = Node::new(DynamicState::new(name));
                new_child.insert(template, data);
                new_child
            });
        }

        self.needs_optimization = true;
    }

    fn insert_wildcard(&mut self, template: &mut Template, data: NodeData, name: String) {
        if let Some(child) = self
            .wildcard_children
            .iter_mut()
            .find(|child| child.state.name == name)
        {
            child.insert(template, data);
        } else {
            self.wildcard_children.push({
                let mut new_child = Node::new(WildcardState::new(name));
                new_child.insert(template, data);
                new_child
            });
        }

        self.needs_optimization = true;
    }

    fn insert_end_wildcard(&mut self, data: NodeData, name: String) {
        self.end_wildcard = Some(Box::new(EndWildcardState { name, data }));

        self.needs_optimization = true;
    }
}
