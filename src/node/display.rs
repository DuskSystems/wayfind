use super::Node;
use crate::node::Kind;
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
                Kind::Root => "▽".to_string(),
                Kind::Static => String::from_utf8_lossy(&node.prefix).to_string(),
                Kind::Dynamic => {
                    let name = String::from_utf8_lossy(&node.prefix);
                    constraint.map_or_else(
                        || format!("{{{name}}}"),
                        |constraint| format!("{{{name}:{constraint}}}"),
                    )
                }
                Kind::Wildcard | Kind::EndWildcard => {
                    let name = String::from_utf8_lossy(&node.prefix);
                    constraint.map_or_else(
                        || format!("{{*{name}}}"),
                        |constraint| format!("{{*{name}:{constraint}}}"),
                    )
                }
            };

            if is_root {
                writeln!(output, "{key}")?;
            } else {
                let branch = if is_last { "╰─" } else { "├─" };
                if node.data.is_some() {
                    writeln!(output, "{padding}{branch} {key} ○")?;
                } else {
                    writeln!(output, "{padding}{branch} {key}")?;
                }
            }

            let new_prefix = if is_root {
                padding.to_string()
            } else if is_last {
                format!("{padding}   ")
            } else {
                format!("{padding}│  ")
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
        let padding = " ".repeat(self.prefix.len().saturating_sub(1));
        debug_node(&mut output, self, &padding, true, true)?;
        write!(f, "{}", output.trim_end())
    }
}
