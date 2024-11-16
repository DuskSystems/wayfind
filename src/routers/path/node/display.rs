use super::{Node, State};
use alloc::{borrow::ToOwned, format, string::String};
use core::fmt::{Display, Write};

impl<'r, S: State> Display for Node<'r, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fn debug_node<S: State>(
            output: &mut String,
            node: &Node<'_, S>,
            padding: &str,
            is_top: bool,
            is_last: bool,
        ) -> core::fmt::Result {
            let key = node.state.key();

            if is_top {
                if node.data.is_some() {
                    writeln!(output, "{key} [*]")?;
                } else {
                    writeln!(output, "{key}")?;
                }
            } else {
                let branch = if is_last { "╰─" } else { "├─" };
                if node.data.is_some() {
                    writeln!(output, "{padding}{branch} {key} [*]")?;
                } else {
                    writeln!(output, "{padding}{branch} {key}")?;
                }
            }

            let new_prefix = if is_top {
                padding.to_owned()
            } else if is_last {
                format!("{padding}   ")
            } else {
                format!("{padding}│  ")
            };

            let mut total_children = node.static_children.nodes.len()
                + node.dynamic_children.nodes.len()
                + node.wildcard_children.nodes.len()
                + node.end_wildcard_children.nodes.len();

            for child in node.static_children.iter() {
                total_children -= 1;
                debug_node(output, child, &new_prefix, false, total_children == 0)?;
            }

            for child in node.dynamic_children.iter() {
                total_children -= 1;
                debug_node(output, child, &new_prefix, false, total_children == 0)?;
            }

            for child in node.wildcard_children.iter() {
                total_children -= 1;
                debug_node(output, child, &new_prefix, false, total_children == 0)?;
            }

            for child in node.end_wildcard_children.iter() {
                total_children -= 1;
                debug_node(output, child, &new_prefix, false, total_children == 0)?;
            }

            Ok(())
        }

        let mut output = String::new();
        let padding = " ".repeat(self.state.padding());

        // Handle root node manually
        if self.state.key().is_empty() {
            let total_children = self.static_children.nodes.len()
                + self.dynamic_children.nodes.len()
                + self.wildcard_children.nodes.len()
                + self.end_wildcard_children.nodes.len();

            let mut remaining = total_children;

            for child in self.static_children.iter() {
                remaining -= 1;
                debug_node(&mut output, child, "", true, remaining == 0)?;
            }

            for child in self.dynamic_children.iter() {
                remaining -= 1;
                debug_node(&mut output, child, "", true, remaining == 0)?;
            }

            for child in self.wildcard_children.iter() {
                remaining -= 1;
                debug_node(&mut output, child, "", true, remaining == 0)?;
            }

            for child in self.end_wildcard_children.iter() {
                remaining -= 1;
                debug_node(&mut output, child, "", true, remaining == 0)?;
            }
        } else {
            debug_node(&mut output, self, &padding, true, true)?;
        }

        write!(f, "{}", output.trim_end())
    }
}
