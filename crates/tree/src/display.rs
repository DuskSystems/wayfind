use crate::{
    node::{Config, Data, Node},
    state::State,
};
use std::fmt::{Display, Write};

impl<C: Config, S: State> Display for Node<C, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn debug_node<C: Config, S: State>(
            output: &mut String,
            node: &Node<C, S>,
            padding: &str,
            is_top: bool,
            is_last: bool,
        ) -> std::fmt::Result {
            let key = node.state.key();
            let data_str = if node.data.is_empty() {
                String::new()
            } else {
                let entries: Vec<_> = node
                    .data
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "{}:{}",
                            k.map_or("*".to_owned(), |id| id.to_string()),
                            v.id().map_or("*".to_owned(), |id| id.to_string())
                        )
                    })
                    .collect();
                format!(" [{}]", entries.join(", "))
            };

            if is_top {
                writeln!(output, "{key}{data_str}")?;
            } else {
                let branch = if is_last { "╰─" } else { "├─" };
                writeln!(output, "{padding}{branch} {key}{data_str}")?;
            }

            let new_prefix = if is_top {
                padding.to_owned()
            } else if is_last {
                format!("{padding}   ")
            } else {
                format!("{padding}│  ")
            };

            let mut total_children = node.static_children.len()
                + node.dynamic_children.len()
                + node.wildcard_children.len()
                + node.end_wildcard_children.len();

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

        if self.state.key().is_empty() {
            let total_children = self.static_children.len()
                + self.dynamic_children.len()
                + self.wildcard_children.len()
                + self.end_wildcard_children.len();

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
