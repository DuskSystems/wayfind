use crate::segment::{Segment, Segments};

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
}

impl<'a, T> Node<'a, T> {
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
        };

        let new_child_b = Self {
            kind: NodeKind::Static,

            prefix: &prefix[common_prefix..],
            data: None,

            static_children: vec![],
            dynamic_children: vec![],
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
                };

                new_child.insert(segments, data);
                new_child
            });
        }
    }
}
