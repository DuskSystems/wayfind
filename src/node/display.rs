use super::Node;
use crate::node::NodeKind;
use std::fmt::{Display, Write};

impl<T> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn debug_node<T>(
            output: &mut String,
            node: &Node<T>,
            padding: &str,
            is_root: bool,
            is_last: bool,
        ) -> std::fmt::Result {
            let constraint = node.constraint.as_ref().map(|c| String::from_utf8_lossy(c));
            let key = match &node.kind {
                NodeKind::Root => "$".to_string(),
                NodeKind::Static => String::from_utf8_lossy(&node.prefix).to_string(),
                NodeKind::Dynamic => {
                    let name = String::from_utf8_lossy(&node.prefix);
                    constraint.map_or_else(
                        || format!("{{{name}}}"),
                        |constraint| format!("{{{name}:{constraint}}}"),
                    )
                }
                NodeKind::Wildcard | NodeKind::EndWildcard => {
                    let name = String::from_utf8_lossy(&node.prefix);
                    constraint.map_or_else(
                        || format!("{{*{name}}}"),
                        |constraint| format!("{{*{name}:{constraint}}}"),
                    )
                }
            };

            let value = node
                .data
                .as_ref()
                .map_or(String::new(), |_node_data| " [*]".to_string());

            if is_root {
                writeln!(output, "{key}")?;
            } else {
                let branch = if is_last { "╰─" } else { "├─" };
                writeln!(output, "{padding}{branch} {key}{value}")?;
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

            // Chain all children together, in order
            let mut children = node
                .static_children
                .iter()
                .chain(node.dynamic_children.iter())
                .chain(node.wildcard_children.iter())
                .chain(node.end_wildcard_children.iter())
                .peekable();

            while let Some(child) = children.next() {
                let is_last = children.peek().is_none();
                debug_node(output, child, &new_prefix, false, is_last)?;
            }

            Ok(())
        }

        let mut output = String::new();

        let padding = if self.prefix.is_empty() {
            String::new()
        } else {
            " ".repeat(self.prefix.len() - 1)
        };

        debug_node(&mut output, self, &padding, true, true)?;
        write!(f, "\n{}", output.trim_end())
    }
}
