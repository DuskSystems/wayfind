use super::{Node, NodeConstraint, NodeData, NodeKind};
use crate::{errors::insert::InsertError, parts::Part, route::Route};
use std::cmp::Ordering;

impl<T> Node<T> {
    pub fn insert(&mut self, route: &mut Route<'_>, data: NodeData<T>) -> Result<(), InsertError> {
        if let Some(segment) = route.parts.pop() {
            match segment {
                Part::Static { prefix } => self.insert_static(route, data, prefix)?,
                Part::Dynamic { name } => {
                    let constraint = route
                        .constraints
                        .iter()
                        .position(|&(constraint_name, _)| constraint_name.as_bytes() == name)
                        .map(|index| route.constraints.remove(index).1);

                    self.insert_dynamic(route, data, name, constraint)?;
                }
                Part::Wildcard { name } if route.parts.is_empty() => {
                    let constraint = route
                        .constraints
                        .iter()
                        .position(|&(constraint_name, _)| constraint_name.as_bytes() == name)
                        .map(|index| route.constraints.remove(index).1);

                    self.insert_end_wildcard(data, name, constraint)?;
                }
                Part::Wildcard { name } => {
                    let constraint = route
                        .constraints
                        .iter()
                        .position(|&(constraint_name, _)| constraint_name.as_bytes() == name)
                        .map(|index| route.constraints.remove(index).1);

                    self.insert_wildcard(route, data, name, constraint)?;
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

    fn insert_static(&mut self, route: &mut Route<'_>, data: NodeData<T>, prefix: &[u8]) -> Result<(), InsertError> {
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
                    end_wildcard: None,

                    quick_dynamic: false,
                };

                new_child.insert(route, data)?;
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
                child.insert(route, data)?;
            } else {
                child.insert_static(route, data, &prefix[common_prefix..])?;
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
            end_wildcard: std::mem::take(&mut child.end_wildcard),

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
            end_wildcard: None,

            quick_dynamic: false,
        };

        child.prefix = child.prefix[..common_prefix].to_vec();

        if prefix[common_prefix..].is_empty() {
            child.static_children = vec![new_child_a];
            child.insert(route, data)?;
        } else {
            child.static_children = vec![new_child_a, new_child_b];
            child.static_children[1].insert(route, data)?;
        }

        Ok(())
    }

    fn insert_dynamic(
        &mut self,
        route: &mut Route<'_>,
        data: NodeData<T>,
        name: &[u8],
        constraint: Option<NodeConstraint>,
    ) -> Result<(), InsertError> {
        if let Some(child) = self
            .dynamic_children
            .iter_mut()
            .find(|child| child.prefix == name && child.constraint == constraint)
        {
            child.insert(route, data)?;
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
                    end_wildcard: None,

                    quick_dynamic: false,
                };

                new_child.insert(route, data)?;
                new_child
            });
        }

        Ok(())
    }

    fn insert_wildcard(
        &mut self,
        route: &mut Route<'_>,
        data: NodeData<T>,
        name: &[u8],
        constraint: Option<NodeConstraint>,
    ) -> Result<(), InsertError> {
        if let Some(child) = self
            .wildcard_children
            .iter_mut()
            .find(|child| child.prefix == name && child.constraint == constraint)
        {
            child.insert(route, data)?;
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
                    end_wildcard: None,

                    quick_dynamic: false,
                };

                new_child.insert(route, data)?;
                new_child
            });
        }

        Ok(())
    }

    fn insert_end_wildcard(
        &mut self,
        data: NodeData<T>,
        name: &[u8],
        constraint: Option<NodeConstraint>,
    ) -> Result<(), InsertError> {
        if let Some(end_wildcard) = &self.end_wildcard {
            if end_wildcard.prefix == name && end_wildcard.constraint == constraint {
                return Err(InsertError::DuplicatePath);
            }
        }

        self.end_wildcard = Some(Box::new(Self {
            kind: NodeKind::EndWildcard,

            prefix: name.to_vec(),
            data: Some(data),
            constraint,

            static_children: vec![],
            dynamic_children: vec![],
            wildcard_children: vec![],
            end_wildcard: None,

            quick_dynamic: false,
        }));

        Ok(())
    }

    pub(super) fn update_quicks(&mut self) {
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
            child.update_quicks();
        }

        for child in &mut self.dynamic_children {
            child.update_quicks();
        }

        if let Some(child) = self.end_wildcard.as_mut() {
            child.update_quicks();
        }
    }

    fn sort_children(&mut self) {
        self.dynamic_children
            .sort_by(|a, b| match (&a.constraint, &b.constraint) {
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                _ => Ordering::Equal,
            });

        self.wildcard_children
            .sort_by(|a, b| match (&a.constraint, &b.constraint) {
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                _ => Ordering::Equal,
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

        if let Some(ref mut end_wildcard) = self.end_wildcard {
            end_wildcard.sort_children();
        }
    }
}
