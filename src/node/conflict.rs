use crate::node::{Node, NodeData};
use crate::parser::Part;

impl<S, T> Node<S, T> {
    /// Checks if a template conflicts with an existing template.
    /// Handles both direct and structural conflicts.
    pub(crate) fn conflict(&self, parts: &[Part]) -> Option<&NodeData<T>> {
        let Some((part, remaining)) = parts.split_last() else {
            return self.data.as_ref();
        };

        match part {
            Part::Static { prefix } => self.conflict_static(remaining, prefix),
            Part::Dynamic { .. } => self.conflict_dynamic(remaining),
            Part::Wildcard { .. } if remaining.is_empty() => self.conflict_end_wildcard(),
            Part::Wildcard { .. } => self.conflict_wildcard(remaining),
        }
    }

    fn conflict_static(&self, parts: &[Part], prefix: &[u8]) -> Option<&NodeData<T>> {
        let first = *prefix.first()?;

        for child in &self.static_children {
            if child.state.first != first {
                continue;
            }

            if prefix.len() >= child.state.prefix.len()
                && child.state.prefix.iter().zip(prefix).all(|(a, b)| a == b)
            {
                let remaining_prefix = &prefix[child.state.prefix.len()..];
                if remaining_prefix.is_empty() {
                    if let Some(data) = child.conflict(parts) {
                        return Some(data);
                    }
                } else if let Some(data) = child.conflict_static(parts, remaining_prefix) {
                    return Some(data);
                }
            }
        }

        None
    }

    fn conflict_dynamic(&self, parts: &[Part]) -> Option<&NodeData<T>> {
        for child in &self.dynamic_children {
            if let Some(data) = child.conflict(parts) {
                return Some(data);
            }
        }

        None
    }

    fn conflict_wildcard(&self, parts: &[Part]) -> Option<&NodeData<T>> {
        for child in &self.wildcard_children {
            if let Some(data) = child.conflict(parts) {
                return Some(data);
            }
        }

        None
    }

    fn conflict_end_wildcard(&self) -> Option<&NodeData<T>> {
        self.end_wildcard.as_deref()?.data.as_ref()
    }
}
