use alloc::format;
use alloc::string::String;
use core::fmt;
use core::fmt::Write as _;

use crate::node::Node;
use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};

struct DisplayContext<'a> {
    output: &'a mut String,
    padding: String,
    is_root: bool,
}

impl DisplayContext<'_> {
    fn write_children(
        &mut self,
        key: &str,
        static_children: &[Node<StaticState>],
        dynamic_children: &[Node<DynamicState>],
        wildcard_children: &[Node<WildcardState>],
        end_wildcard: Option<&Node<EndWildcardState>>,
    ) -> fmt::Result {
        if self.is_root {
            writeln!(self.output, "{key}")?;
        }

        let prev_padding = self.padding.clone();
        let prev_root = self.is_root;

        let mut count = static_children.len()
            + dynamic_children.len()
            + wildcard_children.len()
            + usize::from(end_wildcard.is_some());

        for child in static_children {
            count -= 1;
            let child_key = String::from_utf8_lossy(&child.state.prefix).into_owned();
            self.write_static_entry(&child_key, child, count == 0, key.is_empty())?;
        }

        for child in dynamic_children {
            count -= 1;
            let child_key = format!("<{}>", child.state.name);
            self.write_dynamic_entry(&child_key, child, count == 0, key.is_empty())?;
        }

        for child in wildcard_children {
            count -= 1;
            let child_key = format!("<*{}>", child.state.name);
            self.write_wildcard_entry(&child_key, child, count == 0, key.is_empty())?;
        }

        if let Some(child) = end_wildcard {
            let branch = if key.is_empty() { "" } else { "╰─ " };
            writeln!(
                self.output,
                "{}{branch}<*{}>",
                self.padding, child.state.name
            )?;
        }

        self.padding = prev_padding;
        self.is_root = prev_root;

        Ok(())
    }

    fn write_static_entry(
        &mut self,
        key: &str,
        node: &Node<StaticState>,
        is_last: bool,
        parent_empty: bool,
    ) -> fmt::Result {
        self.write_header(key, is_last, parent_empty)?;

        let saved_padding = self.padding.clone();
        self.advance_padding(key, is_last);

        self.write_children(
            key,
            &node.static_children,
            &node.dynamic_children,
            &node.wildcard_children,
            node.end_wildcard.as_deref(),
        )?;

        self.padding = saved_padding;

        Ok(())
    }

    fn write_dynamic_entry(
        &mut self,
        key: &str,
        node: &Node<DynamicState>,
        is_last: bool,
        parent_empty: bool,
    ) -> fmt::Result {
        self.write_header(key, is_last, parent_empty)?;

        let saved_padding = self.padding.clone();
        self.advance_padding(key, is_last);

        self.write_children(
            key,
            &node.static_children,
            &node.dynamic_children,
            &node.wildcard_children,
            node.end_wildcard.as_deref(),
        )?;

        self.padding = saved_padding;

        Ok(())
    }

    fn write_wildcard_entry(
        &mut self,
        key: &str,
        node: &Node<WildcardState>,
        is_last: bool,
        parent_empty: bool,
    ) -> fmt::Result {
        self.write_header(key, is_last, parent_empty)?;

        let saved_padding = self.padding.clone();
        self.advance_padding(key, is_last);

        self.write_children(
            key,
            &node.static_children,
            &node.dynamic_children,
            &node.wildcard_children,
            node.end_wildcard.as_deref(),
        )?;

        self.padding = saved_padding;

        Ok(())
    }

    fn write_header(&mut self, key: &str, is_last: bool, parent_empty: bool) -> fmt::Result {
        if parent_empty {
            self.is_root = true;
        }

        if !key.is_empty() && !self.is_root {
            let branch = if is_last { "╰─" } else { "├─" };
            writeln!(self.output, "{}{branch} {key}", self.padding)?;
        } else if !key.is_empty() {
            writeln!(self.output, "{key}")?;
        }

        Ok(())
    }

    fn advance_padding(&mut self, key: &str, is_last: bool) {
        if !self.is_root && !key.is_empty() {
            if is_last {
                self.padding = format!("{}   ", self.padding);
            } else {
                self.padding = format!("{}│  ", self.padding);
            }
        }

        self.is_root = false;
    }
}

impl<S> fmt::Display for Node<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let mut ctx = DisplayContext {
            output: &mut output,
            padding: String::new(),
            is_root: true,
        };

        ctx.write_children(
            "",
            &self.static_children,
            &self.dynamic_children,
            &self.wildcard_children,
            self.end_wildcard.as_deref(),
        )?;

        write!(f, "{}", output.trim_end())
    }
}
