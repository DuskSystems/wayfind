use crate::node::Node;

impl<T> Node<T> {
    /// Re-optimizes the tree after an insert/delete.
    pub(crate) fn optimize(&mut self) {
        if !self.needs_optimization {
            return;
        }

        self.static_children.sort();
        self.dynamic_children.sort();
        self.wildcard_children.sort();
        self.end_wildcard_children.sort();

        for children in [
            &mut self.static_children,
            &mut self.dynamic_children,
            &mut self.wildcard_children,
            &mut self.end_wildcard_children,
        ] {
            for child in children.iter_mut() {
                child.optimize();
            }
        }

        self.update_dynamic_children_shortcut();
        self.update_wildcard_children_shortcut();

        self.needs_optimization = false;
    }

    fn update_dynamic_children_shortcut(&mut self) {
        self.dynamic_children_shortcut = self.dynamic_children.iter().all(|child| {
            // Leading slash?
            if child.prefix.first() == Some(&b'/') {
                return true;
            }

            // No children?
            if child.static_children.is_empty()
                && child.dynamic_children.is_empty()
                && child.wildcard_children.is_empty()
                && child.end_wildcard_children.is_empty()
            {
                return true;
            }

            // All static children start with a slash?
            if child
                .static_children
                .iter()
                .all(|child| child.prefix.first() == Some(&b'/'))
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
                && child.end_wildcard_children.is_empty()
            {
                return true;
            }

            // All static children start with a slash?
            if child
                .static_children
                .iter()
                .all(|child| child.prefix.first() == Some(&b'/'))
            {
                return true;
            }

            false
        });
    }
}
