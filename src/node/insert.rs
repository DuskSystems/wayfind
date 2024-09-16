use super::{Children, Data, Kind, Node};
use crate::{
    errors::InsertError,
    parser::{Part, Route},
};

impl<T> Node<T> {
    /// Inserts a new route into the node tree with associated data.
    ///
    /// Recursively traverses the node tree, creating new nodes as necessary.
    /// Will error if there's already data at the end node.
    pub fn insert(&mut self, route: &mut Route, data: Data<T>) -> Result<(), InsertError> {
        if let Some(part) = route.parts.pop() {
            match part {
                Part::Static { prefix } => self.insert_static(route, data, &prefix)?,
                Part::Dynamic {
                    name, constraint, ..
                } => {
                    self.insert_dynamic(route, data, &name, constraint)?;
                }
                Part::Wildcard {
                    name, constraint, ..
                } if route.parts.is_empty() => {
                    self.insert_end_wildcard(route, data, &name, constraint)?;
                }
                Part::Wildcard {
                    name, constraint, ..
                } => {
                    self.insert_wildcard(route, data, &name, constraint)?;
                }
            };
        } else {
            if let Some(data) = &self.data {
                let conflict = match data {
                    Data::Inline { route, .. } | Data::Shared { route, .. } => route.to_string(),
                };

                return Err(InsertError::DuplicateRoute {
                    route: String::from_utf8_lossy(&route.raw).to_string(),
                    conflict,
                });
            }

            self.data = Some(data);
            self.needs_optimization = true;
        }

        Ok(())
    }

    fn insert_static(
        &mut self,
        route: &mut Route,
        data: Data<T>,
        prefix: &[u8],
    ) -> Result<(), InsertError> {
        // Check if the first byte is already a child here.
        let Some(child) = self
            .static_children
            .iter_mut()
            .find(|child| child.prefix[0] == prefix[0])
        else {
            self.static_children.push({
                let mut new_child = Self {
                    kind: Kind::Static,

                    prefix: prefix.to_vec(),
                    data: None,
                    constraint: None,

                    static_children: Children::default(),
                    dynamic_children: Children::default(),
                    wildcard_children: Children::default(),
                    end_wildcard_children: Children::default(),

                    quick_dynamic: false,
                    needs_optimization: false,
                };

                new_child.insert(route, data)?;
                new_child
            });

            self.needs_optimization = true;
            return Ok(());
        };

        let common_prefix = prefix
            .iter()
            .zip(&child.prefix)
            .take_while(|&(x, y)| x == y)
            .count();

        // If the new prefix matches or extends the existing prefix, we can just insert it directly.
        if common_prefix >= child.prefix.len() {
            if common_prefix >= prefix.len() {
                child.insert(route, data)?;
            } else {
                child.insert_static(route, data, &prefix[common_prefix..])?;
            }

            self.needs_optimization = true;
            return Ok(());
        }

        // Not a clean insert, need to split the existing child node.
        let new_child_a = Self {
            kind: Kind::Static,

            prefix: child.prefix[common_prefix..].to_vec(),
            data: child.data.take(),
            constraint: None,

            static_children: std::mem::take(&mut child.static_children),
            dynamic_children: std::mem::take(&mut child.dynamic_children),
            wildcard_children: std::mem::take(&mut child.wildcard_children),
            end_wildcard_children: std::mem::take(&mut child.end_wildcard_children),

            quick_dynamic: child.quick_dynamic,
            needs_optimization: child.needs_optimization,
        };

        let new_child_b = Self {
            kind: Kind::Static,

            prefix: prefix[common_prefix..].to_vec(),
            data: None,
            constraint: None,

            static_children: Children::default(),
            dynamic_children: Children::default(),
            wildcard_children: Children::default(),
            end_wildcard_children: Children::default(),

            quick_dynamic: false,
            needs_optimization: false,
        };

        child.prefix = child.prefix[..common_prefix].to_vec();
        child.needs_optimization = true;

        if prefix[common_prefix..].is_empty() {
            child.static_children = vec![new_child_a].into();
            child.insert(route, data)?;
        } else {
            child.static_children = vec![new_child_a, new_child_b].into();
            child.static_children[1].insert(route, data)?;
        }

        self.needs_optimization = true;
        Ok(())
    }

    fn insert_dynamic(
        &mut self,
        route: &mut Route,
        data: Data<T>,
        name: &[u8],
        constraint: Option<Vec<u8>>,
    ) -> Result<(), InsertError> {
        if let Some(child) = self
            .dynamic_children
            .find_mut(|child| child.prefix == name && child.constraint == constraint)
        {
            child.insert(route, data)?;
        } else {
            self.dynamic_children.push({
                let mut new_child = Self {
                    kind: Kind::Dynamic,

                    prefix: name.to_vec(),
                    data: None,
                    constraint,

                    static_children: Children::default(),
                    dynamic_children: Children::default(),
                    wildcard_children: Children::default(),
                    end_wildcard_children: Children::default(),

                    quick_dynamic: false,
                    needs_optimization: false,
                };

                new_child.insert(route, data)?;
                new_child
            });
        }

        self.needs_optimization = true;
        Ok(())
    }

    fn insert_wildcard(
        &mut self,
        route: &mut Route,
        data: Data<T>,
        name: &[u8],
        constraint: Option<Vec<u8>>,
    ) -> Result<(), InsertError> {
        if let Some(child) = self
            .wildcard_children
            .find_mut(|child| child.prefix == name && child.constraint == constraint)
        {
            child.insert(route, data)?;
        } else {
            self.wildcard_children.push({
                let mut new_child = Self {
                    kind: Kind::Wildcard,

                    prefix: name.to_vec(),
                    data: None,
                    constraint,

                    static_children: Children::default(),
                    dynamic_children: Children::default(),
                    wildcard_children: Children::default(),
                    end_wildcard_children: Children::default(),

                    quick_dynamic: false,
                    needs_optimization: false,
                };

                new_child.insert(route, data)?;
                new_child
            });
        }

        self.needs_optimization = true;
        Ok(())
    }

    fn insert_end_wildcard(
        &mut self,
        route: &Route,
        data: Data<T>,
        name: &[u8],
        constraint: Option<Vec<u8>>,
    ) -> Result<(), InsertError> {
        if let Some(child) = self
            .end_wildcard_children
            .iter()
            .find(|child| child.prefix == name && child.constraint == constraint)
        {
            let conflict = match &child.data {
                Some(Data::Inline { route, .. } | Data::Shared { route, .. }) => route.to_string(),
                None => "Unknown".to_string(),
            };

            return Err(InsertError::DuplicateRoute {
                route: String::from_utf8_lossy(&route.raw).to_string(),
                conflict,
            });
        }

        self.end_wildcard_children.push(Self {
            kind: Kind::EndWildcard,

            prefix: name.to_vec(),
            data: Some(data),
            constraint,

            static_children: Children::default(),
            dynamic_children: Children::default(),
            wildcard_children: Children::default(),
            end_wildcard_children: Children::default(),

            quick_dynamic: false,
            needs_optimization: false,
        });

        self.needs_optimization = true;
        Ok(())
    }
}
