use crate::{
    matches::Parameter,
    parts::{Part, Parts},
};
use std::{fmt::Display, sync::Arc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeKind {
    Root,
    Static,
    Dynamic,
    Wildcard,
    EndWildcard,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeData<T> {
    pub path: Arc<str>,
    pub value: T,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<T> {
    pub kind: NodeKind,

    pub prefix: Vec<u8>,
    pub data: Option<NodeData<T>>,

    pub static_children: Vec<Node<T>>,
    pub dynamic_children: Vec<Node<T>>,
    pub wildcard_children: Vec<Node<T>>,
    pub end_wildcard: Option<Box<Node<T>>>,

    // TODO: Come up with a better name.
    pub quick_dynamic: bool,
}

impl<T> Node<T> {
    #[allow(clippy::missing_panics_doc)]
    pub fn insert(&mut self, mut parts: Parts<'_>, data: NodeData<T>) {
        if let Some(segment) = parts.pop() {
            match segment {
                Part::Static { prefix } => self.insert_static(parts, data, prefix),
                Part::Dynamic { name } => self.insert_dynamic(parts, data, name),
                Part::Wildcard { name } if parts.is_empty() => self.insert_end_wildcard(data, name),
                Part::Wildcard { name } => self.insert_wildcard(parts, data, name),
                Part::Regex { .. } => unimplemented!(),
            }
        } else {
            assert!(self.data.is_none(), "Duplicate path");
            self.data = Some(data);
        }

        self.update_quick_dynamic();
    }

    fn insert_static(&mut self, parts: Parts, data: NodeData<T>, prefix: &[u8]) {
        let Some(child) = self
            .static_children
            .iter_mut()
            .find(|child| child.prefix[0] == prefix[0])
        else {
            self.static_children.push({
                let mut new_child = Self {
                    kind: NodeKind::Static,

                    prefix: prefix.to_vec(),
                    data: None,

                    static_children: vec![],
                    dynamic_children: vec![],
                    wildcard_children: vec![],
                    end_wildcard: None,

                    quick_dynamic: false,
                };

                new_child.insert(parts, data);
                new_child
            });

            return;
        };

        let common_prefix = prefix
            .iter()
            .zip(&child.prefix)
            .take_while(|&(x, y)| x == y)
            .count();

        if common_prefix >= child.prefix.len() {
            if common_prefix >= prefix.len() {
                child.insert(parts, data);
            } else {
                child.insert_static(parts, data, &prefix[common_prefix..]);
            }

            return;
        }

        let new_child_a = Self {
            kind: NodeKind::Static,

            prefix: child.prefix[common_prefix..].to_vec(),
            data: child.data.take(),

            static_children: std::mem::take(&mut child.static_children),
            dynamic_children: std::mem::take(&mut child.dynamic_children),
            wildcard_children: std::mem::take(&mut child.wildcard_children),
            end_wildcard: std::mem::take(&mut child.end_wildcard),

            quick_dynamic: false,
        };

        let new_child_b = Self {
            kind: NodeKind::Static,

            prefix: prefix[common_prefix..].to_vec(),
            data: None,

            static_children: vec![],
            dynamic_children: vec![],
            wildcard_children: vec![],
            end_wildcard: None,

            quick_dynamic: false,
        };

        child.prefix = child.prefix[..common_prefix].to_vec();

        if prefix[common_prefix..].is_empty() {
            child.static_children = vec![new_child_a];
            child.insert(parts, data);
        } else {
            child.static_children = vec![new_child_a, new_child_b];
            child.static_children[1].insert(parts, data);
        }
    }

    fn insert_dynamic(&mut self, parts: Parts, data: NodeData<T>, name: &[u8]) {
        if let Some(child) = self
            .dynamic_children
            .iter_mut()
            .find(|child| child.prefix == name)
        {
            child.insert(parts, data);
        } else {
            self.dynamic_children.push({
                let mut new_child = Self {
                    kind: NodeKind::Dynamic,

                    prefix: name.to_vec(),
                    data: None,

                    static_children: vec![],
                    dynamic_children: vec![],
                    wildcard_children: vec![],
                    end_wildcard: None,

                    quick_dynamic: false,
                };

                new_child.insert(parts, data);
                new_child
            });
        }
    }

    fn insert_wildcard(&mut self, parts: Parts, data: NodeData<T>, name: &[u8]) {
        if let Some(child) = self
            .wildcard_children
            .iter_mut()
            .find(|child| child.prefix == name)
        {
            child.insert(parts, data);
        } else {
            self.wildcard_children.push({
                let mut new_child = Self {
                    kind: NodeKind::Wildcard,

                    prefix: name.to_vec(),
                    data: None,

                    static_children: vec![],
                    dynamic_children: vec![],
                    wildcard_children: vec![],
                    end_wildcard: None,

                    quick_dynamic: false,
                };

                new_child.insert(parts, data);
                new_child
            });
        }
    }

    fn insert_end_wildcard(&mut self, data: NodeData<T>, name: &[u8]) {
        // FIXME: We probably need splitting capabilities here, to change a end wildcard into a normal wildcard?
        self.end_wildcard = Some(Box::new(Self {
            kind: NodeKind::EndWildcard,

            prefix: name.to_vec(),
            data: Some(data),

            static_children: vec![],
            dynamic_children: vec![],
            wildcard_children: vec![],
            end_wildcard: None,

            quick_dynamic: false,
        }));
    }

    fn update_quick_dynamic(&mut self) {
        self.quick_dynamic = self
            .dynamic_children
            .iter()
            .all(|child| {
                // Leading slash?
                if child.prefix.first() == Some(&b'/') {
                    return true;
                }

                // No children?
                if child.static_children.is_empty() && child.dynamic_children.is_empty() && child.end_wildcard.is_none()
                {
                    return true;
                }

                // All static children start with a slash?
                if child
                    .static_children
                    .iter()
                    .all(|child| child.prefix.first() == Some(&b'/'))
                {
                    return true;
                }

                false
            });

        for child in &mut self.static_children {
            child.update_quick_dynamic();
        }

        for child in &mut self.dynamic_children {
            child.update_quick_dynamic();
        }

        if let Some(child) = self.end_wildcard.as_mut() {
            child.update_quick_dynamic();
        }
    }

    pub fn matches<'a>(&'a self, path: &'a [u8], parameters: &mut Vec<Parameter<'a>>) -> Option<&'a NodeData<T>> {
        if path.is_empty() {
            return self.data.as_ref();
        }

        if let Some(matches) = self.matches_static(path, parameters) {
            return Some(matches);
        }

        if let Some(matches) = self.matches_dynamic(path, parameters) {
            return Some(matches);
        }

        if let Some(matches) = self.matches_wildcard(path, parameters) {
            return Some(matches);
        }

        if let Some(matches) = self.matches_end_wildcard(path, parameters) {
            return Some(matches);
        }

        None
    }

    fn matches_static<'a>(&'a self, path: &'a [u8], parameters: &mut Vec<Parameter<'a>>) -> Option<&'a NodeData<T>> {
        for static_child in &self.static_children {
            // NOTE: This was previously a "starts_with" call, but turns out this is much faster.
            if path.len() >= static_child.prefix.len()
                && static_child
                    .prefix
                    .iter()
                    .zip(path)
                    .all(|(a, b)| a == b)
            {
                let remaining_path = &path[static_child.prefix.len()..];
                if let Some(node_data) = static_child.matches(remaining_path, parameters) {
                    return Some(node_data);
                }
            }
        }

        None
    }

    fn matches_dynamic<'a>(&'a self, path: &'a [u8], parameters: &mut Vec<Parameter<'a>>) -> Option<&'a NodeData<T>> {
        if self.quick_dynamic {
            self.matches_dynamic_segment(path, parameters)
        } else {
            self.matches_dynamic_inline(path, parameters)
        }
    }

    // Dynamic with support for inline dynamic sections, e.g. `{name}.{extension}`
    // NOTE: Parameters are greedy in nature:
    //   Route: `{name}.{extension}`
    //   Path: `my.long.file.txt`
    //   Name: `my.long.file`
    //   Ext: `txt`
    fn matches_dynamic_inline<'a>(
        &'a self,
        path: &'a [u8],
        parameters: &mut Vec<Parameter<'a>>,
    ) -> Option<&'a NodeData<T>> {
        for dynamic_child in &self.dynamic_children {
            let mut consumed = 0;

            let mut last_match = None;
            let mut last_match_parameters = vec![];

            while consumed < path.len() {
                if path[consumed] == b'/' {
                    break;
                }

                consumed += 1;

                let mut current_parameters = parameters.clone();
                current_parameters.push(Parameter {
                    key: &dynamic_child.prefix,
                    value: &path[..consumed],
                });

                if let Some(node_data) = dynamic_child.matches(&path[consumed..], &mut current_parameters) {
                    last_match = Some(node_data);
                    last_match_parameters = current_parameters;
                }
            }

            if let Some(node_data) = last_match {
                *parameters = last_match_parameters;
                return Some(node_data);
            }
        }

        None
    }

    // Doesn't support inline dynamic sections, e.g. `{name}.{extension}`, only `/{segment}/`
    fn matches_dynamic_segment<'a>(
        &'a self,
        path: &'a [u8],
        parameters: &mut Vec<Parameter<'a>>,
    ) -> Option<&'a NodeData<T>> {
        for dynamic_child in &self.dynamic_children {
            let segment_end = path
                .iter()
                .position(|&b| b == b'/')
                .unwrap_or(path.len());

            parameters.push(Parameter {
                key: &dynamic_child.prefix,
                value: &path[..segment_end],
            });

            if let Some(node_data) = dynamic_child.matches(&path[segment_end..], parameters) {
                return Some(node_data);
            }

            parameters.pop();
        }

        None
    }

    fn matches_wildcard<'a>(&'a self, path: &'a [u8], parameters: &mut Vec<Parameter<'a>>) -> Option<&'a NodeData<T>> {
        for wildcard_child in &self.wildcard_children {
            let mut consumed = 0;
            let mut remaining_path = path;
            let mut section_end = false;

            while !remaining_path.is_empty() {
                if section_end {
                    consumed += 1;
                }

                let segment_end = remaining_path
                    .iter()
                    .position(|&b| b == b'/')
                    .unwrap_or(remaining_path.len());

                if segment_end == 0 {
                    consumed += 1;
                    section_end = false;
                } else {
                    consumed += segment_end;
                    section_end = true;
                }

                parameters.push(Parameter {
                    key: &wildcard_child.prefix,
                    value: if path[..consumed].ends_with(b"/") {
                        &path[..consumed - 1]
                    } else {
                        &path[..consumed]
                    },
                });

                if let Some(node_data) = wildcard_child.matches(&remaining_path[segment_end..], parameters) {
                    return Some(node_data);
                }

                parameters.pop();

                if segment_end == remaining_path.len() {
                    break;
                }

                remaining_path = &remaining_path[segment_end + 1..];
            }
        }

        None
    }

    fn matches_end_wildcard<'a>(
        &'a self,
        path: &'a [u8],
        parameters: &mut Vec<Parameter<'a>>,
    ) -> Option<&'a NodeData<T>> {
        if let Some(end_wildcard) = &self.end_wildcard {
            parameters.push(Parameter {
                key: &end_wildcard.prefix,
                value: path,
            });

            return end_wildcard.data.as_ref();
        }

        None
    }
}

// FIXME: Messy, but it works.
// FIXME: Doesn't handle split multi-byte characters.
impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn debug_node<T: Display>(
            f: &mut std::fmt::Formatter,
            node: &Node<T>,
            padding: &str,
            is_root: bool,
            is_last: bool,
        ) -> std::fmt::Result {
            let key = match node.kind {
                NodeKind::Root => "$",
                NodeKind::Static => &String::from_utf8_lossy(&node.prefix),
                NodeKind::Dynamic => {
                    let prefix = String::from_utf8_lossy(&node.prefix);
                    &format!("{{{prefix}}}")
                }
                NodeKind::Wildcard | NodeKind::EndWildcard => {
                    let prefix = String::from_utf8_lossy(&node.prefix);
                    &format!("{{{prefix}:*}}")
                }
            };

            let value = node
                .data
                .as_ref()
                .map(|node_data| &node_data.value);

            if is_root {
                writeln!(f, "{key}")?;
            } else if is_last {
                match value {
                    Some(value) => writeln!(f, "{padding}╰─ {key} [{value}]")?,
                    None => writeln!(f, "{padding}╰─ {key}")?,
                }
            } else {
                match value {
                    Some(value) => writeln!(f, "{padding}├─ {key} [{value}]")?,
                    None => writeln!(f, "{padding}├─ {key}")?,
                }
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
            let has_end_wildcard = node.end_wildcard.is_some();

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

            // Print end wildcard
            if let Some(child) = &node.end_wildcard {
                debug_node(f, child, &new_prefix, false, true)?;
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
