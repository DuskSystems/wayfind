use crate::{node::Node, specificity::Specificity};

impl<S> Node<S> {
    /// Optimizes the tree structure.
    pub(crate) fn optimize(&mut self) {
        self.optimize_inner(Specificity::default());
    }

    /// Recursively optimizes nodes from root to leaf.
    /// We can skip optimization if the current node hasn't changed.
    fn optimize_inner(&mut self, parent: Specificity) {
        if !self.needs_optimization {
            return;
        }

        if let Some(data) = &mut self.data {
            data.specificity = parent.clone();
        }

        for child in &mut self.static_children {
            let child_specificity = parent.clone().with_static(child.state.prefix.len());
            child.optimize_inner(child_specificity);
        }

        for child in &mut self.dynamic_children {
            let child_specificity = parent.clone().with_dynamic();
            child.optimize_inner(child_specificity);
        }

        for child in &mut self.wildcard_children {
            let child_specificity = parent.clone().with_wildcard();
            child.optimize_inner(child_specificity);
        }

        if let Some(child) = &mut self.end_wildcard {
            let child_specificity = parent.with_wildcard();
            child.optimize_inner(child_specificity);
        }

        self.static_children.sort();
        self.dynamic_children.sort();
        self.wildcard_children.sort();

        self.dynamic_segment_only = self
            .dynamic_children
            .iter()
            .all(|node| Self::is_segment_only(node, &node.state.name));

        self.wildcard_segment_only = self
            .wildcard_children
            .iter()
            .all(|node| Self::is_segment_only(node, &node.state.name));

        self.needs_optimization = false;
    }

    /// Check if this node is segment only.
    fn is_segment_only<T>(node: &Node<T>, name: &str) -> bool {
        // Parameter name starts with '/'
        if name.as_bytes().first() == Some(&b'/') {
            return true;
        }

        // Has no children
        if node.static_children.is_empty()
            && node.dynamic_children.is_empty()
            && node.wildcard_children.is_empty()
            && node.end_wildcard.is_none()
        {
            return true;
        }

        // All static children start with '/'
        node.static_children
            .iter()
            .all(|child| child.state.prefix.first() == Some(&b'/'))
    }
}
