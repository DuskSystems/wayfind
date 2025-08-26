use crate::{node::Node, specificity::Specificity};

impl<S> Node<S> {
    pub(crate) fn optimize(&mut self) {
        self.optimize_inner(Specificity::default());
    }

    fn optimize_inner(&mut self, parent: Specificity) {
        if !self.needs_optimization {
            return;
        }

        if let Some(ref mut data) = self.data {
            data.specificity = parent.clone();
        }

        for child in &mut self.static_children {
            let child_specificity = parent.clone().count_static(child.state.prefix.len());
            child.optimize_inner(child_specificity);
        }

        for child in &mut self.dynamic_children {
            let child_specificity = parent.clone().count_dynamic();
            child.optimize_inner(child_specificity);
        }

        for child in &mut self.wildcard_children {
            let child_specificity = parent.clone().count_wildcard();
            child.optimize_inner(child_specificity);
        }

        if let Some(child) = &mut self.end_wildcard {
            let child_specificity = parent.count_wildcard();
            child.optimize_inner(child_specificity);
        }

        self.static_children.sort();
        self.dynamic_children.sort();
        self.wildcard_children.sort();

        self.update_dynamic_children_shortcut();
        self.update_wildcard_children_shortcut();

        self.needs_optimization = false;
    }

    fn update_dynamic_children_shortcut(&mut self) {
        self.dynamic_segment_only = self.dynamic_children.iter().all(|child| {
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
        self.wildcard_segment_only = self.wildcard_children.iter().all(|child| {
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
