use super::Node;
use crate::node::NodeKind;
use std::fmt::Display;

impl<T: Display> Display for Node<T> {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn debug_node<T: Display>(
            f: &mut std::fmt::Formatter,
            node: &Node<T>,
            padding: &str,
            is_root: bool,
            is_last: bool,
        ) -> std::fmt::Result {
            let key = match &node.kind {
                NodeKind::Root => "$".to_string(),
                NodeKind::Static => String::from_utf8_lossy(&node.prefix).to_string(),
                NodeKind::Dynamic => {
                    let name = String::from_utf8_lossy(&node.prefix);
                    format!("<{name}>")
                }
                NodeKind::Wildcard | NodeKind::EndWildcard => {
                    let name = String::from_utf8_lossy(&node.prefix);
                    format!("<{name}:*>")
                }
            };

            let value = node
                .data
                .as_ref()
                .map(|node_data| &node_data.value);

            let constraints = if node.constraints.is_empty() {
                String::new()
            } else {
                format!(" {:?}", node.constraints)
            };

            if is_root {
                writeln!(f, "{key}")?;
            } else {
                let branch = if is_last { "╰─" } else { "├─" };
                let value = value.map_or(String::new(), |v| format!(" [{v}]"));
                writeln!(f, "{padding}{branch} {key}{value}{constraints}")?;
            }

            // Ensure we align children correctly
            let extra_spacing = " ".repeat(key.len() - 1);
            let new_prefix = if is_root {
                padding.to_string()
            } else if is_last {
                format!("{padding}   {extra_spacing}")
            } else {
                format!("{padding}│  {extra_spacing}")
            };

            let has_dynamic_children = !node.dynamic_children.is_empty();
            let has_wildcard_children = !node.wildcard_children.is_empty();
            let has_end_wildcard = !node.end_wildcard_children.is_empty();

            // Recursively print the static children
            let static_count = node.static_children.len();
            for (index, child) in node.static_children.iter().enumerate() {
                let is_last = if has_dynamic_children || has_wildcard_children || has_end_wildcard {
                    false
                } else {
                    index == (static_count - 1)
                };

                debug_node(f, child, &new_prefix, false, is_last)?;
            }

            // Recursively print dynamic children
            let dynamic_count = node.dynamic_children.len();
            for (index, child) in node.dynamic_children.iter().enumerate() {
                let is_last = if has_wildcard_children || has_end_wildcard {
                    false
                } else {
                    index == (dynamic_count - 1)
                };

                debug_node(f, child, &new_prefix, false, is_last)?;
            }

            // Recursively print wildcard children
            let wildcard_count = node.wildcard_children.len();
            for (index, child) in node
                .wildcard_children
                .iter()
                .enumerate()
            {
                let is_last = if has_end_wildcard {
                    false
                } else {
                    index == (wildcard_count - 1)
                };

                debug_node(f, child, &new_prefix, false, is_last)?;
            }

            // Recursively print end wildcard children
            let end_wildcard_count = node.end_wildcard_children.len();
            for (index, child) in node
                .end_wildcard_children
                .iter()
                .enumerate()
            {
                let is_last = index == (end_wildcard_count - 1);
                debug_node(f, child, &new_prefix, false, is_last)?;
            }

            Ok(())
        }

        let padding = if self.prefix.is_empty() {
            String::new()
        } else {
            " ".repeat(self.prefix.len() - 1)
        };

        debug_node(f, self, &padding, true, true)?;
        Ok(())
    }
}
