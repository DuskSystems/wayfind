use crate::node::{Node, NodeData};
use crate::parser::Part;

impl<S> Node<S> {
    /// Finds an exact template match in the node tree.
    /// Essentially the same as the `Node::insert` logic, without any tree modifications.
    pub fn find(&self, parts: &[Part]) -> Option<&NodeData> {
        let Some(part) = parts.last() else {
            return self.data.as_ref();
        };

        let remaining = &parts[..parts.len() - 1];
        match part {
            Part::Static { prefix } => self.find_static(remaining, prefix),
            Part::Dynamic { name } => self.find_dynamic(remaining, name),
            Part::Wildcard { name } if remaining.is_empty() => {
                self.find_end_wildcard(remaining, name)
            }
            Part::Wildcard { name } => self.find_wildcard(remaining, name),
        }
    }

    fn find_static(&self, parts: &[Part], prefix: &[u8]) -> Option<&NodeData> {
        for child in &self.static_children {
            if !child.state.prefix.is_empty() && child.state.prefix[0] == prefix[0] {
                let common_prefix = prefix
                    .iter()
                    .zip(&child.state.prefix)
                    .take_while(|&(x, y)| x == y)
                    .count();

                if common_prefix >= child.state.prefix.len() {
                    if common_prefix >= prefix.len() {
                        return child.find(parts);
                    }

                    let remaining_prefix = &prefix[common_prefix..];
                    if !remaining_prefix.is_empty() {
                        return child.find_static(parts, remaining_prefix);
                    }
                }
            }
        }

        None
    }

    fn find_dynamic(&self, parts: &[Part], name: &str) -> Option<&NodeData> {
        for child in &self.dynamic_children {
            if child.state.name == name {
                return child.find(parts);
            }
        }

        None
    }

    fn find_end_wildcard(&self, parts: &[Part], name: &str) -> Option<&NodeData> {
        if let Some(child) = &self.end_wildcard
            && child.state.name == name
        {
            return child.find(parts);
        }

        None
    }

    fn find_wildcard(&self, parts: &[Part], name: &str) -> Option<&NodeData> {
        for child in &self.wildcard_children {
            if child.state.name == name {
                return child.find(parts);
            }
        }

        None
    }
}
