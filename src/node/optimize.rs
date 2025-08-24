use crate::node::Node;

impl<S> Node<S> {
    pub(crate) fn optimize(&mut self) {
        if !self.needs_optimization {
            return;
        }

        for child in &mut self.static_children {
            child.optimize();
        }

        for child in &mut self.dynamic_children {
            child.optimize();
        }

        for child in &mut self.wildcard_children {
            child.optimize();
        }

        if let Some(child) = self.end_wildcard.as_mut() {
            child.optimize();
        }

        self.static_children.sort();
        self.dynamic_children.sort();
        self.wildcard_children.sort();

        self.update_dynamic_children_shortcut();
        self.update_wildcard_children_shortcut();

        self.needs_optimization = false;
    }

    fn update_dynamic_children_shortcut(&mut self) {
        self.dynamic_children_shortcut = self.dynamic_children.iter().all(|child| {
            // Leading slash?
            if child.state.name.as_bytes().first() == Some(&b'/') {
                return true;
            }

            // No children?
            if child.static_children.is_empty()
                && child.dynamic_children.is_empty()
                && child.wildcard_children.is_empty()
                && child.end_wildcard.is_none()
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
    }

    fn update_wildcard_children_shortcut(&mut self) {
        self.wildcard_children_shortcut = self.wildcard_children.iter().all(|child| {
            // No children?
            if child.static_children.is_empty()
                && child.dynamic_children.is_empty()
                && child.wildcard_children.is_empty()
                && child.end_wildcard.is_none()
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
    }
}
