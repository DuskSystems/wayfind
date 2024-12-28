use super::{
    node::Node,
    parser::{ParsedTemplate, Part},
    state::{State, StaticState},
};

impl<S: State> Node<'_, S> {
    /// Deletes an authority route from the node tree.
    ///
    /// This method recursively traverses the tree to find and remove the specified authority.
    /// Logic should match that used by the insert method.
    ///
    /// If the authority is found and deleted, we re-optimize the tree structure.
    pub fn delete(&mut self, authority: &mut ParsedTemplate) {
        if let Some(part) = authority.parts.pop() {
            match part {
                Part::Static { prefix } => self.delete_static(authority, &prefix),
                Part::Dynamic {
                    name, constraint, ..
                } => self.delete_dynamic(authority, &name, constraint.as_ref()),
                Part::Wildcard {
                    name, constraint, ..
                } if authority.parts.is_empty() => {
                    self.delete_end_wildcard(&name, constraint.as_ref());
                }
                Part::Wildcard {
                    name, constraint, ..
                } => self.delete_wildcard(authority, &name, constraint.as_ref()),
            }
        } else {
            self.data.take();
            self.needs_optimization = true;
        }
    }

    fn delete_static(&mut self, authority: &mut ParsedTemplate, prefix: &[u8]) {
        let Some(index) = self.static_children.iter().position(|child| {
            prefix.len() >= child.state.prefix.len()
                && child.state.prefix.iter().zip(prefix).all(|(a, b)| a == b)
        }) else {
            return;
        };

        let child = &mut self.static_children[index];
        child.needs_optimization = true;

        let remaining_prefix = &prefix[child.state.prefix.len()..];
        if remaining_prefix.is_empty() {
            child.delete(authority);
        } else {
            child.delete_static(authority, remaining_prefix);
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
    }

    fn delete_dynamic(
        &mut self,
        authority: &mut ParsedTemplate,
        name: &str,
        constraint: Option<&String>,
    ) {
        let Some(index) = self.dynamic_children.iter().position(|child| {
            child.state.name == name && child.state.constraint.as_ref() == constraint
        }) else {
            return;
        };

        let child = &mut self.dynamic_children[index];
        child.delete(authority);

        if child.is_empty() {
            self.dynamic_children.remove(index);
            self.needs_optimization = true;
        }
    }

    fn delete_wildcard(
        &mut self,
        authority: &mut ParsedTemplate,
        name: &str,
        constraint: Option<&String>,
    ) {
        let Some(index) = self.wildcard_children.iter().position(|child| {
            child.state.name == name && child.state.constraint.as_ref() == constraint
        }) else {
            return;
        };

        let child = &mut self.wildcard_children[index];
        child.delete(authority);

        if child.is_empty() {
            self.wildcard_children.remove(index);
            self.needs_optimization = true;
        }
    }

    fn delete_end_wildcard(&mut self, name: &str, constraint: Option<&String>) {
        let Some(index) = self.end_wildcard_children.iter().position(|child| {
            child.state.name == name && child.state.constraint.as_ref() == constraint
        }) else {
            return;
        };

        let mut child = self.end_wildcard_children.remove(index);
        child.data.take();
        self.needs_optimization = true;
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
