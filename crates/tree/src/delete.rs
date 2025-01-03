use super::{
    node::Node,
    state::{State, StaticState},
};
use crate::{
    node::Config,
    parser::{Part, Template},
};

impl<C: Config, S: State> Node<C, S> {
    /// Deletes a route from the node tree.
    ///
    /// This method recursively traverses the tree to find and remove the specified route.
    /// Logic should match that used by the insert method.
    ///
    /// If the route is found and deleted, we re-optimize the tree structure.
    pub fn delete(&mut self, key: Option<usize>, route: &Template) {
        self.delete_at_position(key, route, route.parts.len());
    }

    fn delete_at_position(&mut self, key: Option<usize>, route: &Template, position: usize) {
        if position == 0 {
            self.data.remove(&key);
            self.needs_optimization = true;
            return;
        }

        let part = &route.parts[position - 1];
        match part {
            Part::Static { prefix } => self.delete_static(key, route, position - 1, prefix),
            Part::Dynamic { name, constraint } => {
                self.delete_dynamic(key, route, position - 1, name, constraint.as_deref());
            }
            Part::Wildcard { name, constraint } if position == 1 => {
                self.delete_end_wildcard(key, name, constraint.as_deref());
            }
            Part::Wildcard { name, constraint } => {
                self.delete_wildcard(key, route, position - 1, name, constraint.as_deref());
            }
        }
    }

    fn delete_static(
        &mut self,
        key: Option<usize>,
        route: &Template,
        position: usize,
        prefix: &[u8],
    ) {
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
            child.delete_at_position(key, route, position);
        } else {
            child.delete_static(key, route, position, remaining_prefix);
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
        key: Option<usize>,
        route: &Template,
        position: usize,
        name: &str,
        constraint: Option<&str>,
    ) {
        let Some(index) = self.dynamic_children.iter().position(|child| {
            child.state.name == name && child.state.constraint.as_deref() == constraint
        }) else {
            return;
        };

        let child = &mut self.dynamic_children[index];
        child.delete_at_position(key, route, position);

        if child.is_empty() {
            self.dynamic_children.remove(index);
            self.needs_optimization = true;
        }
    }

    fn delete_wildcard(
        &mut self,
        key: Option<usize>,
        route: &Template,
        position: usize,
        name: &str,
        constraint: Option<&str>,
    ) {
        let Some(index) = self.wildcard_children.iter().position(|child| {
            child.state.name == name && child.state.constraint.as_deref() == constraint
        }) else {
            return;
        };

        let child = &mut self.wildcard_children[index];
        child.delete_at_position(key, route, position);

        if child.is_empty() {
            self.wildcard_children.remove(index);
            self.needs_optimization = true;
        }
    }

    fn delete_end_wildcard(&mut self, key: Option<usize>, name: &str, constraint: Option<&str>) {
        let Some(index) = self.end_wildcard_children.iter().position(|child| {
            child.state.name == name && child.state.constraint.as_deref() == constraint
        }) else {
            return;
        };

        let child = &mut self.end_wildcard_children[index];
        child.data.remove(&key);
        if child.data.is_empty() {
            self.end_wildcard_children.remove(index);
        }

        self.needs_optimization = true;
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
            && self.static_children.is_empty()
            && self.dynamic_children.is_empty()
            && self.wildcard_children.is_empty()
            && self.end_wildcard_children.is_empty()
    }

    fn is_compressible(&self) -> bool {
        self.data.is_empty()
            && self.static_children.len() == 1
            && self.dynamic_children.is_empty()
            && self.wildcard_children.is_empty()
            && self.end_wildcard_children.is_empty()
    }
}
