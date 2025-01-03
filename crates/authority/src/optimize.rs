use super::{node::Node, state::State, AuthorityData};

impl<S: State> Node<'_, S> {
    pub(crate) fn optimize(&mut self) {
        self.optimize_inner(0);
    }

    fn optimize_inner(&mut self, priority: usize) {
        self.priority = priority + self.calculate_priority();

        if !self.needs_optimization {
            return;
        }

        for child in self.static_children.iter_mut() {
            child.optimize_inner(self.priority);
        }

        for child in self.dynamic_children.iter_mut() {
            child.optimize_inner(self.priority);
        }

        for child in self.wildcard_children.iter_mut() {
            child.optimize_inner(self.priority);
        }

        for child in self.end_wildcard_children.iter_mut() {
            child.optimize_inner(self.priority);
        }

        self.static_children.sort();
        self.dynamic_children.sort();
        self.wildcard_children.sort();
        self.end_wildcard_children.sort();

        self.update_dynamic_children_shortcut();
        self.update_wildcard_children_shortcut();

        self.needs_optimization = false;
    }

    fn calculate_priority(&self) -> usize {
        let mut priority = self.state.priority();
        if self.data.is_some() {
            priority += 1_000;
            priority += match &self.data {
                Some(AuthorityData { authority, .. }) => {
                    authority.len() + (authority.bytes().filter(|&b| b == b'.').count() * 100)
                }
                None => 0,
            };
        }

        priority
    }

    fn update_dynamic_children_shortcut(&mut self) {
        self.dynamic_children_shortcut = self.dynamic_children.iter().all(|child| {
            // Leading dot?
            if child.state.name.as_bytes().first() == Some(&b'.') {
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

            // All static children start with a dot?
            if child
                .static_children
                .iter()
                .all(|child| child.state.prefix.first() == Some(&b'.'))
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

            // All static children start with a dot?
            if child
                .static_children
                .iter()
                .all(|child| child.state.prefix.first() == Some(&b'.'))
            {
                return true;
            }

            false
        });
    }
}
