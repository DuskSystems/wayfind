use alloc::{borrow::ToOwned, fmt, format, string::String};
use core::fmt::Write;

use crate::{node::Node, state::NodeState};

impl<S: NodeState> fmt::Display for Node<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn debug_node<S: NodeState>(
            output: &mut String,
            node: &Node<S>,
            padding: &str,
            is_root: bool,
            is_last: bool,
        ) -> fmt::Result {
            let key = node.state.key();
            if !key.is_empty() {
                if is_root {
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
            }

            let padding = if !is_root && !key.is_empty() {
                if is_last {
                    format!("{padding}   ")
                } else {
                    format!("{padding}│  ")
                }
            } else {
                padding.to_owned()
            };

            let mut count = node.static_children.len()
                + node.dynamic_constrained_children.len()
                + node.dynamic_children.len()
                + node.wildcard_constrained_children.len()
                + node.wildcard_children.len()
                + node.end_wildcard_constrained_children.len()
                + node.end_wildcard_children.len();

            for child in &node.static_children {
                count -= 1;
                debug_node(output, child, &padding, key.is_empty(), count == 0)?;
            }

            for child in &node.dynamic_constrained_children {
                count -= 1;
                debug_node(output, child, &padding, key.is_empty(), count == 0)?;
            }

            for child in &node.dynamic_children {
                count -= 1;
                debug_node(output, child, &padding, key.is_empty(), count == 0)?;
            }

            for child in &node.wildcard_constrained_children {
                count -= 1;
                debug_node(output, child, &padding, key.is_empty(), count == 0)?;
            }

            for child in &node.wildcard_children {
                count -= 1;
                debug_node(output, child, &padding, key.is_empty(), count == 0)?;
            }

            for child in &node.end_wildcard_constrained_children {
                count -= 1;
                debug_node(output, child, &padding, key.is_empty(), count == 0)?;
            }

            for child in &node.end_wildcard_children {
                count -= 1;
                debug_node(output, child, &padding, key.is_empty(), count == 0)?;
            }

            Ok(())
        }

        let mut output = String::new();
        let padding = " ".repeat(self.state.padding());
        debug_node(&mut output, self, &padding, true, true)?;
        write!(f, "{}", output.trim_end())
    }
}
