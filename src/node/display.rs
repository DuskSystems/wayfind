use alloc::borrow::ToOwned as _;
use alloc::format;
use alloc::string::{String, ToString as _};
use core::fmt;
use core::fmt::Write as _;

use crate::node::Node;

impl<S: fmt::Display> fmt::Display for Node<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn display_node<S: fmt::Display>(
            output: &mut String,
            node: &Node<S>,
            padding: &str,
            is_root: bool,
            is_last: bool,
        ) -> fmt::Result {
            let key = node.state.to_string();
            if !key.is_empty() {
                if is_root {
                    writeln!(output, "{key}")?;
                } else {
                    let branch = if is_last { "╰─" } else { "├─" };
                    writeln!(output, "{padding}{branch} {key}")?;
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
                + node.dynamic_children.len()
                + node.wildcard_children.len()
                + usize::from(node.end_wildcard.is_some());

            for child in &node.static_children {
                count -= 1;
                display_node(output, child, &padding, key.is_empty(), count == 0)?;
            }

            for child in &node.dynamic_children {
                count -= 1;
                display_node(output, child, &padding, key.is_empty(), count == 0)?;
            }

            for child in &node.wildcard_children {
                count -= 1;
                display_node(output, child, &padding, key.is_empty(), count == 0)?;
            }

            if let Some(child) = &node.end_wildcard {
                let branch = if key.is_empty() { "" } else { "╰─ " };
                writeln!(output, "{padding}{branch}{child}")?;
            }

            Ok(())
        }

        let mut output = String::new();
        display_node(&mut output, self, "", true, true)?;
        write!(f, "{}", output.trim_end())
    }
}
