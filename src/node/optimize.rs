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

        self.update_quicks();
        self.needs_optimization = false;
    }

    /// Check if we can short-cut our searching logic for dynamic children.
    /// Instead of walking each path byte-by-byte, we can instead just to the next '/' character.
    /// This only works if there are no inline dynamic children, e.g. `/{name}.{ext}`.
    fn update_quicks(&mut self) {
        self.quick_dynamic = self.dynamic_children.iter().all(|child| {
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
}
