use crate::node::{Node, NodeData};
use crate::parser::Part;

impl<S> Node<S> {
    /// Checks if a template conflicts with an existing template.
    /// Handles both direct and structural conflicts.
    pub fn conflict(&self, parts: &[Part]) -> Option<&NodeData> {
        let Some(part) = parts.last() else {
            return self.data.as_ref();
        };

        let remaining = &parts[..parts.len() - 1];
        match part {
            Part::Static { prefix } => self.conflict_static(remaining, prefix),
            Part::Dynamic { .. } => self.conflict_dynamic(remaining),
            Part::Wildcard { .. } if remaining.is_empty() => self.conflict_end_wildcard(),
            Part::Wildcard { .. } => self.conflict_wildcard(remaining),
        }
    }

    fn conflict_static(&self, parts: &[Part], prefix: &[u8]) -> Option<&NodeData> {
        let first = *prefix.first()?;

        for child in &self.static_children {
            if child.state.first != first {
                continue;
            }

            let common_prefix = child
                .state
                .prefix
                .iter()
                .zip(prefix)
                .take_while(|&(a, b)| a == b)
                .count();

            if common_prefix >= child.state.prefix.len() {
                if common_prefix >= prefix.len() {
                    if let Some(data) = child.conflict(parts) {
                        return Some(data);
                    }
                } else if let Some(data) = child.conflict_static(parts, &prefix[common_prefix..]) {
                    return Some(data);
                }
            }
        }

        None
    }

    fn conflict_dynamic(&self, parts: &[Part]) -> Option<&NodeData> {
        for child in &self.dynamic_children {
            if let Some(data) = child.conflict(parts) {
                return Some(data);
            }
        }

        None
    }

    fn conflict_wildcard(&self, parts: &[Part]) -> Option<&NodeData> {
        for child in &self.wildcard_children {
            if let Some(data) = child.conflict(parts) {
                return Some(data);
            }
        }

        None
    }

    fn conflict_end_wildcard(&self) -> Option<&NodeData> {
        Some(&self.end_wildcard.as_ref()?.data)
    }
}
