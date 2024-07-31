use crate::{
    matches::Parameter,
    segment::{Segment, Segments},
};
use smallvec::{smallvec, SmallVec};

#[derive(Debug, Eq, PartialEq)]
pub enum NodeKind {
    Root,
    Static,
    Dynamic,
}

#[derive(Debug, Eq, PartialEq)]
pub struct NodeData<'a, T> {
    pub path: &'a str,
    pub value: T,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Node<'a, T> {
    pub kind: NodeKind,

    pub prefix: &'a [u8],
    pub data: Option<NodeData<'a, T>>,

    pub static_children: Vec<Node<'a, T>>,
    pub dynamic_children: Vec<Node<'a, T>>,

    // TODO: Come up with a better name.
    pub quick_dynamic: bool,
}

impl<'a, T> Node<'a, T> {
    #[allow(clippy::missing_panics_doc)]
    pub fn insert(&mut self, mut segments: Segments<'a>, data: NodeData<'a, T>) {
        if let Some(segment) = segments.pop() {
            match segment {
                Segment::Static { prefix } => self.insert_static(segments, data, prefix),
                Segment::Dynamic { name } => self.insert_dynamic(segments, data, name),
                _ => unimplemented!(),
            }
        } else {
            assert!(self.data.is_none(), "Duplicate path");
            self.data = Some(data);
        }

        self.update_quick_dynamic();
    }

    fn insert_static(&mut self, segments: Segments<'a>, data: NodeData<'a, T>, prefix: &'a [u8]) {
        let Some(child) = self
            .static_children
            .iter_mut()
            .find(|child| child.prefix[0] == prefix[0])
        else {
            self.static_children.push({
                let mut new_child = Self {
                    kind: NodeKind::Static,

                    prefix,
                    data: None,

                    static_children: vec![],
                    dynamic_children: vec![],

                    quick_dynamic: false,
                };

                new_child.insert(segments, data);
                new_child
            });

            return;
        };

        let common_prefix = prefix
            .iter()
            .zip(child.prefix)
            .take_while(|&(x, y)| x == y)
            .count();

        if common_prefix >= child.prefix.len() {
            if common_prefix >= prefix.len() {
                child.insert(segments, data);
            } else {
                child.insert_static(segments, data, &prefix[common_prefix..]);
            }

            return;
        }

        let new_child_a = Self {
            kind: NodeKind::Static,

            prefix: &child.prefix[common_prefix..],
            data: child.data.take(),

            static_children: std::mem::take(&mut child.static_children),
            dynamic_children: std::mem::take(&mut child.dynamic_children),

            quick_dynamic: false,
        };

        let new_child_b = Self {
            kind: NodeKind::Static,

            prefix: &prefix[common_prefix..],
            data: None,

            static_children: vec![],
            dynamic_children: vec![],

            quick_dynamic: false,
        };

        child.prefix = &child.prefix[..common_prefix];

        if prefix[common_prefix..].is_empty() {
            child.static_children = vec![new_child_a];
            child.insert(segments, data);
        } else {
            child.static_children = vec![new_child_a, new_child_b];
            child.static_children[1].insert(segments, data);
        }
    }

    fn insert_dynamic(&mut self, segments: Segments<'a>, data: NodeData<'a, T>, name: &'a [u8]) {
        if let Some(child) = self
            .dynamic_children
            .iter_mut()
            .find(|child| child.prefix == name)
        {
            child.insert(segments, data);
        } else {
            self.dynamic_children.push({
                let mut new_child = Self {
                    kind: NodeKind::Dynamic,

                    prefix: name,
                    data: None,

                    static_children: vec![],
                    dynamic_children: vec![],

                    quick_dynamic: false,
                };

                new_child.insert(segments, data);
                new_child
            });
        }
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

                if child.static_children.is_empty() && child.dynamic_children.is_empty() {
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
    }

    pub fn matches(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
        if path.is_empty() {
            return self.data.as_ref();
        }

        if let Some(matches) = self.matches_static(path, parameters) {
            return Some(matches);
        }

        if let Some(matches) = self.matches_dynamic(path, parameters) {
            return Some(matches);
        }

        None
    }

    fn matches_static(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
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

    fn matches_dynamic(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
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
    fn matches_dynamic_inline(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
        for dynamic_child in &self.dynamic_children {
            let mut consumed = 0;

            let mut last_match = None;
            let mut last_match_parameters = smallvec![];

            while consumed < path.len() {
                if path[consumed] == b'/' {
                    break;
                }

                consumed += 1;

                let mut current_parameters = parameters.clone();
                current_parameters.push(Parameter {
                    key: dynamic_child.prefix,
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
    fn matches_dynamic_segment(
        &'a self,
        path: &'a [u8],
        parameters: &mut SmallVec<[Parameter<'a>; 4]>,
    ) -> Option<&'a NodeData<'a, T>> {
        for dynamic_child in &self.dynamic_children {
            let segment_end = path
                .iter()
                .position(|&b| b == b'/')
                .unwrap_or(path.len());

            parameters.push(Parameter {
                key: dynamic_child.prefix,
                value: &path[..segment_end],
            });

            if let Some(node_data) = dynamic_child.matches(&path[segment_end..], parameters) {
                return Some(node_data);
            }

            parameters.pop();
        }

        None
    }
}
