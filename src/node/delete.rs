use crate::{
    node::{Node, NodeData},
    parser::{Part, Template},
    state::{NodeState, StaticState},
};

impl<S: NodeState> Node<S> {
    /// Deletes a route from the node tree.
    ///
    /// This method recursively traverses the tree to find and remove the specified template.
    /// Logic should match that used by the insert method.
    ///
    /// If the route is found and deleted, we re-optimize the tree structure.
    pub fn delete(&mut self, template: &mut Template) -> Option<NodeData> {
        if let Some(part) = template.parts.pop() {
            match part {
                Part::Static { prefix } => self.delete_static(template, &prefix),
                Part::DynamicConstrained { name, constraint } => {
                    self.delete_dynamic_constrained(template, &name, constraint)
                }
                Part::Dynamic { name } => self.delete_dynamic(template, &name),
                Part::WildcardConstrained { name, constraint } if template.parts.is_empty() => {
                    self.delete_end_wildcard_constrained(&name, constraint)
                }
                Part::Wildcard { name } if template.parts.is_empty() => {
                    self.delete_end_wildcard(&name)
                }
                Part::WildcardConstrained { name, constraint } => {
                    self.delete_wildcard_constrained(template, &name, constraint)
                }
                Part::Wildcard { name } => self.delete_wildcard(template, &name),
            }
        } else {
            let data = self.data.take()?;
            self.needs_optimization = true;
            Some(data)
        }
    }

    fn delete_static(&mut self, template: &mut Template, prefix: &[u8]) -> Option<NodeData> {
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
            self.static_children.remove(index);
            self.needs_optimization = true;
        } else if child.is_compressible() {
            let merge = child.static_children.remove(0);

            let mut prefix = core::mem::take(&mut child.state.prefix);
            prefix.extend(&merge.state.prefix);

            *child = Node {
                state: StaticState::new(prefix),
                needs_optimization: true,
                ..merge
            };
        }

        result
    }

    fn delete_dynamic_constrained(
        &mut self,
        template: &mut Template,
        name: &str,
        constraint: usize,
    ) -> Option<NodeData> {
        let index = self
            .dynamic_constrained_children
            .iter()
            .position(|child| child.state.name == name && child.state.constraint == constraint)?;

        let child = &mut self.dynamic_constrained_children[index];
        let result = child.delete(template);

        if child.is_empty() {
            self.dynamic_constrained_children.remove(index);
            self.needs_optimization = true;
        }

        result
    }

    fn delete_dynamic(&mut self, template: &mut Template, name: &str) -> Option<NodeData> {
        let index = self
            .dynamic_children
            .iter()
            .position(|child| child.state.name == name)?;

        let child = &mut self.dynamic_children[index];
        let result = child.delete(template);

        if child.is_empty() {
            self.dynamic_children.remove(index);
            self.needs_optimization = true;
        }

        result
    }

    fn delete_wildcard_constrained(
        &mut self,
        template: &mut Template,
        name: &str,
        constraint: usize,
    ) -> Option<NodeData> {
        let index = self
            .wildcard_constrained_children
            .iter()
            .position(|child| child.state.name == name && child.state.constraint == constraint)?;

        let child = &mut self.wildcard_constrained_children[index];
        let result = child.delete(template);

        if child.is_empty() {
            self.wildcard_constrained_children.remove(index);
            self.needs_optimization = true;
        }

        result
    }

    fn delete_wildcard(&mut self, template: &mut Template, name: &str) -> Option<NodeData> {
        let index = self
            .wildcard_children
            .iter()
            .position(|child| child.state.name == name)?;

        let child = &mut self.wildcard_children[index];
        let result = child.delete(template);

        if child.is_empty() {
            self.wildcard_children.remove(index);
            self.needs_optimization = true;
        }

        result
    }

    fn delete_end_wildcard_constrained(
        &mut self,
        name: &str,
        constraint: usize,
    ) -> Option<NodeData> {
        let index = self
            .end_wildcard_constrained_children
            .iter()
            .position(|child| child.state.name == name && child.state.constraint == constraint)?;

        let mut child = self.end_wildcard_constrained_children.remove(index);

        let data = child.data.take()?;
        self.needs_optimization = true;

        Some(data)
    }

    fn delete_end_wildcard(&mut self, name: &str) -> Option<NodeData> {
        let index = self
            .end_wildcard_children
            .iter()
            .position(|child| child.state.name == name)?;

        let mut child = self.end_wildcard_children.remove(index);

        let data = child.data.take()?;
        self.needs_optimization = true;

        Some(data)
    }

    const fn is_empty(&self) -> bool {
        self.data.is_none()
            && self.static_children.is_empty()
            && self.dynamic_constrained_children.is_empty()
            && self.dynamic_children.is_empty()
            && self.wildcard_constrained_children.is_empty()
            && self.wildcard_children.is_empty()
            && self.end_wildcard_constrained_children.is_empty()
            && self.end_wildcard_children.is_empty()
    }

    const fn is_compressible(&self) -> bool {
        self.data.is_none()
            && self.static_children.len() == 1
            && self.dynamic_constrained_children.is_empty()
            && self.dynamic_children.is_empty()
            && self.wildcard_constrained_children.is_empty()
            && self.wildcard_children.is_empty()
            && self.end_wildcard_constrained_children.is_empty()
            && self.end_wildcard_children.is_empty()
    }
}
