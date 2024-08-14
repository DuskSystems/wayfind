use super::{Node, NodeData, NodeKind};
use crate::{
    errors::insert::InsertError,
    parts::{Part, Parts},
};
use std::cmp::Ordering;

impl<T> Node<T> {
    pub fn insert(&mut self, parts: &mut Parts<'_>, data: NodeData<T>) -> Result<(), InsertError> {
        if let Some(segment) = parts.pop() {
            match segment {
                Part::Static { prefix } => self.insert_static(parts, data, prefix)?,
                Part::Dynamic { name, constraint } => {
                    self.insert_dynamic(parts, data, name, constraint)?;
                }
                Part::Wildcard { name, constraint } if parts.is_empty() => {
                    self.insert_end_wildcard(data, name, constraint)?;
                }
                Part::Wildcard { name, constraint } => {
                    self.insert_wildcard(parts, data, name, constraint)?;
                }
            };
        } else {
            if self.data.is_some() {
                return Err(InsertError::DuplicatePath);
            }

            self.data = Some(data);
        }

        self.update_quicks();
        self.sort_children();

        Ok(())
    }

    fn insert_static(
        &mut self,
        parts: &mut Parts<'_>,
        data: NodeData<T>,
        prefix: &[u8],
    ) -> Result<(), InsertError> {
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
                    constraint: None,

                    static_children: vec![],
                    dynamic_children: vec![],
                    wildcard_children: vec![],
                    end_wildcard_children: vec![],

                    quick_dynamic: false,
                };

                new_child.insert(parts, data)?;
                new_child
            });

            return Ok(());
        };

        let common_prefix = prefix
            .iter()
            .zip(&child.prefix)
            .take_while(|&(x, y)| x == y)
            .count();

        if common_prefix >= child.prefix.len() {
            if common_prefix >= prefix.len() {
                child.insert(parts, data)?;
            } else {
                child.insert_static(parts, data, &prefix[common_prefix..])?;
            }

            return Ok(());
        }

        let new_child_a = Self {
            kind: NodeKind::Static,

            prefix: child.prefix[common_prefix..].to_vec(),
            data: child.data.take(),
            constraint: None,

            static_children: std::mem::take(&mut child.static_children),
            dynamic_children: std::mem::take(&mut child.dynamic_children),
            wildcard_children: std::mem::take(&mut child.wildcard_children),
            end_wildcard_children: std::mem::take(&mut child.end_wildcard_children),

            quick_dynamic: false,
        };

        let new_child_b = Self {
            kind: NodeKind::Static,

            prefix: prefix[common_prefix..].to_vec(),
            data: None,
            constraint: None,

            static_children: vec![],
            dynamic_children: vec![],
            wildcard_children: vec![],
            end_wildcard_children: vec![],

            quick_dynamic: false,
        };

        child.prefix = child.prefix[..common_prefix].to_vec();

        if prefix[common_prefix..].is_empty() {
            child.static_children = vec![new_child_a];
            child.insert(parts, data)?;
        } else {
            child.static_children = vec![new_child_a, new_child_b];
            child.static_children[1].insert(parts, data)?;
        }

        Ok(())
    }

    fn insert_dynamic(
        &mut self,
        parts: &mut Parts<'_>,
        data: NodeData<T>,
        name: &[u8],
        constraint: Option<Vec<u8>>,
    ) -> Result<(), InsertError> {
        if let Some(child) = self
            .dynamic_children
            .iter_mut()
            .find(|child| child.prefix == name && child.constraint == constraint)
        {
            child.insert(parts, data)?;
        } else {
            self.dynamic_children.push({
                let mut new_child = Self {
                    kind: NodeKind::Dynamic,

                    prefix: name.to_vec(),
                    data: None,
                    constraint,

                    static_children: vec![],
                    dynamic_children: vec![],
                    wildcard_children: vec![],
                    end_wildcard_children: vec![],

                    quick_dynamic: false,
                };

                new_child.insert(parts, data)?;
                new_child
            });
        }

        Ok(())
    }

    fn insert_wildcard(
        &mut self,
        parts: &mut Parts<'_>,
        data: NodeData<T>,
        name: &[u8],
        constraint: Option<Vec<u8>>,
    ) -> Result<(), InsertError> {
        if let Some(child) = self
            .wildcard_children
            .iter_mut()
            .find(|child| child.prefix == name && child.constraint == constraint)
        {
            child.insert(parts, data)?;
        } else {
            self.wildcard_children.push({
                let mut new_child = Self {
                    kind: NodeKind::Wildcard,

                    prefix: name.to_vec(),
                    data: None,
                    constraint,

                    static_children: vec![],
                    dynamic_children: vec![],
                    wildcard_children: vec![],
                    end_wildcard_children: vec![],

                    quick_dynamic: false,
                };

                new_child.insert(parts, data)?;
                new_child
            });
        }

        Ok(())
    }

    fn insert_end_wildcard(
        &mut self,
        data: NodeData<T>,
        name: &[u8],
        constraint: Option<Vec<u8>>,
    ) -> Result<(), InsertError> {
        if self
            .end_wildcard_children
            .iter()
            .any(|child| child.prefix == name && child.constraint == constraint)
        {
            return Err(InsertError::DuplicatePath);
        }

        self.end_wildcard_children.push(Self {
            kind: NodeKind::EndWildcard,

            prefix: name.to_vec(),
            data: Some(data),
            constraint,

            static_children: vec![],
            dynamic_children: vec![],
            wildcard_children: vec![],
            end_wildcard_children: vec![],

            quick_dynamic: false,
        });

        Ok(())
    }

    pub(super) fn update_quicks(&mut self) {
        self.quick_dynamic = self.dynamic_children.iter().all(|child| {
            // Leading slash?
            if child.prefix.first() == Some(&b'/') {
                return true;
            }

            // No children?
            if child.static_children.is_empty()
                && child.dynamic_children.is_empty()
                && child.wildcard_children.is_empty()
                && child.end_wildcard_children.is_empty()
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
            child.update_quicks();
        }

        for child in &mut self.dynamic_children {
            child.update_quicks();
        }

        for child in &mut self.end_wildcard_children {
            child.update_quicks();
        }
    }

    fn sort_children(&mut self) {
        self.static_children.sort_by(|a, b| a.prefix.cmp(&b.prefix));

        self.dynamic_children.sort_by(|a, b| {
            match (a.constraint.is_some(), b.constraint.is_some()) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.prefix.cmp(&b.prefix),
            }
        });

        self.wildcard_children.sort_by(|a, b| {
            match (a.constraint.is_some(), b.constraint.is_some()) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.prefix.cmp(&b.prefix),
            }
        });

        self.end_wildcard_children.sort_by(|a, b| {
            match (a.constraint.is_some(), b.constraint.is_some()) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.prefix.cmp(&b.prefix),
            }
        });

        for child in &mut self.static_children {
            child.sort_children();
        }

        for child in &mut self.dynamic_children {
            child.sort_children();
        }

        for child in &mut self.wildcard_children {
            child.sort_children();
        }

        for child in &mut self.end_wildcard_children {
            child.sort_children();
        }
    }
}
