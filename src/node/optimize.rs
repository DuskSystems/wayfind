use crate::{node::Node, state::NodeState};

impl<T, S: NodeState> Node<T, S> {
    pub(crate) fn optimize(&mut self) {
        if !self.needs_optimization {
            return;
        }

        for child in &mut self.static_children {
            child.optimize();
        }

        for child in &mut self.dynamic_constrained_children {
            child.optimize();
        }

        for child in &mut self.dynamic_children {
            child.optimize();
        }

        for child in &mut self.wildcard_constrained_children {
            child.optimize();
        }

        for child in &mut self.wildcard_children {
            child.optimize();
        }

        for child in &mut self.end_wildcard_constrained_children {
            child.optimize();
        }

        for child in &mut self.end_wildcard_children {
            child.optimize();
        }

        self.static_children.sort();
        self.dynamic_constrained_children.sort();
        self.dynamic_children.sort();
        self.wildcard_constrained_children.sort();
        self.wildcard_children.sort();
        self.end_wildcard_constrained_children.sort();
        self.end_wildcard_children.sort();

        self.update_dynamic_children_shortcut();
        self.update_wildcard_children_shortcut();

        self.needs_optimization = false;
    }

    fn update_dynamic_children_shortcut(&mut self) {
        let constrained_check = self.dynamic_constrained_children.iter().all(|child| {
            // Leading slash?
            if child.state.name.as_bytes().first() == Some(&b'/') {
                return true;
            }

            // No children?
            if child.static_children.is_empty()
                && child.dynamic_constrained_children.is_empty()
                && child.dynamic_children.is_empty()
                && child.wildcard_constrained_children.is_empty()
                && child.wildcard_children.is_empty()
                && child.end_wildcard_constrained_children.is_empty()
                && child.end_wildcard_children.is_empty()
            {
                return true;
            }

            // All static children start with a slash?
            if child
                .static_children
                .iter()
                .all(|child| child.state.prefix.first() == Some(&b'/'))
            {
                return true;
            }

            false
        });

        let unconstrained_check = self.dynamic_children.iter().all(|child| {
            // Leading slash?
            if child.state.name.as_bytes().first() == Some(&b'/') {
                return true;
            }

            // No children?
            if child.static_children.is_empty()
                && child.dynamic_constrained_children.is_empty()
                && child.dynamic_children.is_empty()
                && child.wildcard_constrained_children.is_empty()
                && child.wildcard_children.is_empty()
                && child.end_wildcard_constrained_children.is_empty()
                && child.end_wildcard_children.is_empty()
            {
                return true;
            }

            // All static children start with a slash?
            if child
                .static_children
                .iter()
                .all(|child| child.state.prefix.first() == Some(&b'/'))
            {
                return true;
            }

            false
        });

        self.dynamic_children_shortcut = constrained_check && unconstrained_check;
    }

    fn update_wildcard_children_shortcut(&mut self) {
        let constrained_check = self.wildcard_constrained_children.iter().all(|child| {
            // No children?
            if child.static_children.is_empty()
                && child.dynamic_constrained_children.is_empty()
                && child.dynamic_children.is_empty()
                && child.wildcard_constrained_children.is_empty()
                && child.wildcard_children.is_empty()
                && child.end_wildcard_constrained_children.is_empty()
                && child.end_wildcard_children.is_empty()
            {
                return true;
            }

            // All static children start with a slash?
            if child
                .static_children
                .iter()
                .all(|child| child.state.prefix.first() == Some(&b'/'))
            {
                return true;
            }

            false
        });

        let unconstrained_check = self.wildcard_children.iter().all(|child| {
            // No children?
            if child.static_children.is_empty()
                && child.dynamic_constrained_children.is_empty()
                && child.dynamic_children.is_empty()
                && child.wildcard_constrained_children.is_empty()
                && child.wildcard_children.is_empty()
                && child.end_wildcard_constrained_children.is_empty()
                && child.end_wildcard_children.is_empty()
            {
                return true;
            }

            // All static children start with a slash?
            if child
                .static_children
                .iter()
                .all(|child| child.state.prefix.first() == Some(&b'/'))
            {
                return true;
            }

            false
        });

        self.wildcard_children_shortcut = constrained_check && unconstrained_check;
    }
}
