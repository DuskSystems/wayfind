use super::{
    node::Node,
    state::{DynamicState, EndWildcardState, State, StaticState, WildcardState},
    PathData,
};
use crate::{
    routers::path::{
        errors::PathInsertError,
        parser::{ParsedRoute, Part},
    },
    vec::SortedVec,
};

impl<'r, S: State> Node<'r, S> {
    /// Inserts a new route into the node tree with associated data.
    ///
    /// Recursively traverses the node tree, creating new nodes as necessary.
    /// Will error if there's already data at the end node.
    pub fn insert(
        &mut self,
        route: &mut ParsedRoute,
        data: PathData<'r>,
    ) -> Result<(), PathInsertError> {
        if let Some(part) = route.parts.pop() {
            match part {
                Part::Static { prefix } => self.insert_static(route, data, &prefix)?,
                Part::Dynamic {
                    name, constraint, ..
                } => {
                    self.insert_dynamic(route, data, name, constraint)?;
                }
                Part::Wildcard {
                    name, constraint, ..
                } if route.parts.is_empty() => {
                    self.insert_end_wildcard(route, data, name, constraint)?;
                }
                Part::Wildcard {
                    name, constraint, ..
                } => {
                    self.insert_wildcard(route, data, name, constraint)?;
                }
            };
        } else {
            if let Some(data) = &self.data {
                return Err(PathInsertError::DuplicateRoute {
                    route: String::from_utf8_lossy(&route.input).to_string(),
                    conflict: data.route.to_owned(),
                });
            }

            self.data = Some(data);
            self.needs_optimization = true;
        }

        Ok(())
    }

    fn insert_static(
        &mut self,
        route: &mut ParsedRoute,
        data: PathData<'r>,
        prefix: &[u8],
    ) -> Result<(), PathInsertError> {
        // Check if the first byte is already a child here.
        let Some(child) = self
            .static_children
            .iter_mut()
            .find(|child| child.state.prefix[0] == prefix[0])
        else {
            self.static_children.push({
                let mut new_child = Node {
                    state: StaticState::new(prefix.to_vec()),
                    data: None,

                    static_children: SortedVec::default(),
                    dynamic_children: SortedVec::default(),
                    dynamic_children_shortcut: false,
                    wildcard_children: SortedVec::default(),
                    wildcard_children_shortcut: false,
                    end_wildcard_children: SortedVec::default(),

                    priority: 0,
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
            .zip::<&[u8]>(child.state.prefix.as_ref())
            .take_while(|&(x, y)| x == y)
            .count();

        // If the new prefix matches or extends the existing prefix, we can just insert it directly.
        if common_prefix >= child.state.prefix.len() {
            if common_prefix >= prefix.len() {
                child.insert(route, data)?;
            } else {
                child.insert_static(route, data, &prefix[common_prefix..])?;
            }

            self.needs_optimization = true;
            return Ok(());
        }

        // Not a clean insert, need to split the existing child node.
        let new_child_a = Node {
            state: StaticState::new(child.state.prefix[common_prefix..].to_vec()),
            data: child.data.take(),

            static_children: std::mem::take(&mut child.static_children),
            dynamic_children: std::mem::take(&mut child.dynamic_children),
            dynamic_children_shortcut: child.dynamic_children_shortcut,
            wildcard_children: std::mem::take(&mut child.wildcard_children),
            wildcard_children_shortcut: child.wildcard_children_shortcut,
            end_wildcard_children: std::mem::take(&mut child.end_wildcard_children),

            priority: child.priority,
            needs_optimization: child.needs_optimization,
        };

        let new_child_b = Node {
            state: StaticState::new(prefix[common_prefix..].to_vec()),
            data: None,

            static_children: SortedVec::default(),
            dynamic_children: SortedVec::default(),
            dynamic_children_shortcut: false,
            wildcard_children: SortedVec::default(),
            wildcard_children_shortcut: false,
            end_wildcard_children: SortedVec::default(),

            priority: 0,
            needs_optimization: false,
        };

        child.state = StaticState::new(child.state.prefix[..common_prefix].to_vec());
        child.needs_optimization = true;

        if prefix[common_prefix..].is_empty() {
            child.static_children = SortedVec::new(vec![new_child_a]);
            child.insert(route, data)?;
        } else {
            child.static_children = SortedVec::new(vec![new_child_a, new_child_b]);
            child.static_children[1].insert(route, data)?;
        }

        self.needs_optimization = true;
        Ok(())
    }

    fn insert_dynamic(
        &mut self,
        route: &mut ParsedRoute,
        data: PathData<'r>,
        name: String,
        constraint: Option<String>,
    ) -> Result<(), PathInsertError> {
        if let Some(child) = self
            .dynamic_children
            .find_mut(|child| child.state.name == name && child.state.constraint == constraint)
        {
            child.insert(route, data)?;
        } else {
            self.dynamic_children.push({
                let mut new_child = Node {
                    state: DynamicState::new(name, constraint),
                    data: None,

                    static_children: SortedVec::default(),
                    dynamic_children: SortedVec::default(),
                    dynamic_children_shortcut: false,
                    wildcard_children: SortedVec::default(),
                    wildcard_children_shortcut: false,
                    end_wildcard_children: SortedVec::default(),

                    priority: 0,
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
        route: &mut ParsedRoute,
        data: PathData<'r>,
        name: String,
        constraint: Option<String>,
    ) -> Result<(), PathInsertError> {
        if let Some(child) = self
            .wildcard_children
            .find_mut(|child| child.state.name == name && child.state.constraint == constraint)
        {
            child.insert(route, data)?;
        } else {
            self.wildcard_children.push({
                let mut new_child = Node {
                    state: WildcardState::new(name, constraint),
                    data: None,

                    static_children: SortedVec::default(),
                    dynamic_children: SortedVec::default(),
                    dynamic_children_shortcut: false,
                    wildcard_children: SortedVec::default(),
                    wildcard_children_shortcut: false,
                    end_wildcard_children: SortedVec::default(),

                    priority: 0,
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
        route: &ParsedRoute,
        data: PathData<'r>,
        name: String,
        constraint: Option<String>,
    ) -> Result<(), PathInsertError> {
        if let Some(child) = self
            .end_wildcard_children
            .iter()
            .find(|child| child.state.name == name && child.state.constraint == constraint)
        {
            return Err(PathInsertError::DuplicateRoute {
                route: String::from_utf8_lossy(&route.input).to_string(),
                conflict: child.data.as_ref().unwrap().route.to_owned(),
            });
        }

        self.end_wildcard_children.push(Node {
            state: EndWildcardState::new(name, constraint),
            data: Some(data),

            static_children: SortedVec::default(),
            dynamic_children: SortedVec::default(),
            dynamic_children_shortcut: false,
            wildcard_children: SortedVec::default(),
            wildcard_children_shortcut: false,
            end_wildcard_children: SortedVec::default(),

            priority: 0,
            needs_optimization: false,
        });

        self.needs_optimization = true;
        Ok(())
    }
}
