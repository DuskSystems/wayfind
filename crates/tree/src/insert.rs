use super::{
    node::Node,
    state::{DynamicState, EndWildcardState, State, StaticState, WildcardState},
};
use crate::{
    node::Config,
    parser::{Part, Template},
    vec::SortedVec,
};
use wayfind_storage::Storage;

impl<C: Config, S: State> Node<C, S> {
    /// Inserts a new route into the node tree with associated data.
    /// Recursively traverses the node tree, creating new nodes as necessary.
    pub fn insert(&mut self, key: Option<usize>, route: &Template, data: C::Data) {
        self.insert_at_position(key, route, route.parts.len(), data);
    }

    fn insert_at_position(
        &mut self,
        key: Option<usize>,
        route: &Template,
        position: usize,
        data: C::Data,
    ) {
        if position > 0 {
            let part = &route.parts[position - 1];
            match part {
                Part::Static { prefix } => {
                    self.insert_static(key, route, position - 1, data, prefix);
                }
                Part::Dynamic { name, constraint } => {
                    self.insert_dynamic(
                        key,
                        route,
                        position - 1,
                        data,
                        name,
                        constraint.as_deref(),
                    );
                }
                Part::Wildcard { name, constraint } if position == 1 => {
                    self.insert_end_wildcard(key, data, name, constraint.as_deref());
                }
                Part::Wildcard { name, constraint } => {
                    self.insert_wildcard(
                        key,
                        route,
                        position - 1,
                        data,
                        name,
                        constraint.as_deref(),
                    );
                }
            };
        } else {
            self.data.insert(key, data);
            self.needs_optimization = true;
        }
    }

    fn insert_static(
        &mut self,
        key: Option<usize>,
        route: &Template,
        position: usize,
        data: C::Data,
        prefix: &[u8],
    ) {
        // Check if the first byte is already a child here.
        let Some(child) = self
            .static_children
            .iter_mut()
            .find(|child| child.state.prefix[0] == prefix[0])
        else {
            self.static_children.push({
                let mut new_child = Node {
                    state: StaticState::new(prefix.to_vec()),
                    data: Storage::default(),

                    static_children: SortedVec::default(),
                    dynamic_children: SortedVec::default(),
                    dynamic_children_shortcut: false,
                    wildcard_children: SortedVec::default(),
                    wildcard_children_shortcut: false,
                    end_wildcard_children: SortedVec::default(),

                    priority: 0,
                    needs_optimization: false,
                };

                new_child.insert_at_position(key, route, position, data);
                new_child
            });

            self.needs_optimization = true;
            return;
        };

        let common_prefix = prefix
            .iter()
            .zip::<&[u8]>(child.state.prefix.as_ref())
            .take_while(|&(x, y)| x == y)
            .count();

        if common_prefix >= child.state.prefix.len() {
            if common_prefix >= prefix.len() {
                child.insert_at_position(key, route, position, data);
            } else {
                child.insert_static(key, route, position, data, &prefix[common_prefix..]);
            }

            self.needs_optimization = true;
            return;
        }

        let new_child_a = Node {
            state: StaticState::new(child.state.prefix[common_prefix..].to_vec()),
            data: std::mem::take(&mut child.data),

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
            data: Storage::default(),

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
            child.insert_at_position(key, route, position, data);
        } else {
            child.static_children = SortedVec::new(vec![new_child_a, new_child_b]);
            child.static_children[1].insert_at_position(key, route, position, data);
        }

        self.needs_optimization = true;
    }

    fn insert_dynamic(
        &mut self,
        key: Option<usize>,
        route: &Template,
        position: usize,
        data: C::Data,
        name: &str,
        constraint: Option<&str>,
    ) {
        if let Some(child) = self.dynamic_children.find_mut(|child| {
            child.state.name == name && child.state.constraint.as_deref() == constraint
        }) {
            child.insert_at_position(key, route, position, data);
        } else {
            self.dynamic_children.push({
                let mut new_child = Node {
                    state: DynamicState::new(name, constraint),
                    data: Storage::default(),

                    static_children: SortedVec::default(),
                    dynamic_children: SortedVec::default(),
                    dynamic_children_shortcut: false,
                    wildcard_children: SortedVec::default(),
                    wildcard_children_shortcut: false,
                    end_wildcard_children: SortedVec::default(),

                    priority: 0,
                    needs_optimization: false,
                };

                new_child.insert_at_position(key, route, position, data);
                new_child
            });
        }

        self.needs_optimization = true;
    }

    fn insert_wildcard(
        &mut self,
        key: Option<usize>,
        route: &Template,
        position: usize,
        data: C::Data,
        name: &str,
        constraint: Option<&str>,
    ) {
        if let Some(child) = self.wildcard_children.find_mut(|child| {
            child.state.name == name && child.state.constraint.as_deref() == constraint
        }) {
            child.insert_at_position(key, route, position, data);
        } else {
            self.wildcard_children.push({
                let mut new_child = Node {
                    state: WildcardState::new(name, constraint),
                    data: Storage::default(),

                    static_children: SortedVec::default(),
                    dynamic_children: SortedVec::default(),
                    dynamic_children_shortcut: false,
                    wildcard_children: SortedVec::default(),
                    wildcard_children_shortcut: false,
                    end_wildcard_children: SortedVec::default(),

                    priority: 0,
                    needs_optimization: false,
                };

                new_child.insert_at_position(key, route, position, data);
                new_child
            });
        }

        self.needs_optimization = true;
    }

    fn insert_end_wildcard(
        &mut self,
        key: Option<usize>,
        data: C::Data,
        name: &str,
        constraint: Option<&str>,
    ) {
        if self.end_wildcard_children.iter().any(|child| {
            child.state.name == name && child.state.constraint.as_deref() == constraint
        }) {
            return;
        }

        self.end_wildcard_children.push(Node {
            state: EndWildcardState::new(name, constraint),
            data: Storage::from((key, data)),

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
    }
}
