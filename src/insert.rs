use crate::{
    node::{Node, NodeData},
    parser::{Part, Template},
    state::{DynamicState, EndWildcardState, NodeState, StaticState, WildcardState},
    vec::SortedNode,
};

impl<'r, T, S: NodeState> Node<'r, T, S> {
    /// Inserts a new route into the node tree with associated data.
    /// Recursively traverses the node tree, creating new nodes as necessary.
    pub fn insert(&mut self, template: &mut Template, data: NodeData<'r, T>) {
        if let Some(part) = template.parts.pop() {
            match part {
                Part::Static { prefix } => self.insert_static(template, data, &prefix),
                Part::Dynamic {
                    name, constraint, ..
                } => {
                    self.insert_dynamic(template, data, name, constraint);
                }
                Part::Wildcard {
                    name, constraint, ..
                } if template.parts.is_empty() => {
                    self.insert_end_wildcard(data, name, constraint);
                }
                Part::Wildcard {
                    name, constraint, ..
                } => {
                    self.insert_wildcard(template, data, name, constraint);
                }
            };
        } else {
            self.data = Some(data);
            self.needs_optimization = true;
        }
    }

    fn insert_static(&mut self, template: &mut Template, data: NodeData<'r, T>, prefix: &[u8]) {
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

                    static_children: SortedNode::default(),
                    dynamic_children: SortedNode::default(),
                    dynamic_children_shortcut: false,
                    wildcard_children: SortedNode::default(),
                    wildcard_children_shortcut: false,
                    end_wildcard_children: SortedNode::default(),

                    priority: 0,
                    needs_optimization: false,
                };

                new_child.insert(template, data);
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

        // If the new prefix matches or extends the existing prefix, we can just insert it directly.
        if common_prefix >= child.state.prefix.len() {
            if common_prefix >= prefix.len() {
                child.insert(template, data);
            } else {
                child.insert_static(template, data, &prefix[common_prefix..]);
            }

            self.needs_optimization = true;
            return;
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

            static_children: SortedNode::default(),
            dynamic_children: SortedNode::default(),
            dynamic_children_shortcut: false,
            wildcard_children: SortedNode::default(),
            wildcard_children_shortcut: false,
            end_wildcard_children: SortedNode::default(),

            priority: 0,
            needs_optimization: false,
        };

        child.state = StaticState::new(child.state.prefix[..common_prefix].to_vec());
        child.needs_optimization = true;

        if prefix[common_prefix..].is_empty() {
            child.static_children = SortedNode::new(vec![new_child_a]);
            child.insert(template, data);
        } else {
            child.static_children = SortedNode::new(vec![new_child_a, new_child_b]);
            child.static_children[1].insert(template, data);
        }

        self.needs_optimization = true;
    }

    fn insert_dynamic(
        &mut self,
        template: &mut Template,
        data: NodeData<'r, T>,
        name: String,
        constraint: Option<String>,
    ) {
        if let Some(child) = self
            .dynamic_children
            .find_mut(|child| child.state.name == name && child.state.constraint == constraint)
        {
            child.insert(template, data);
        } else {
            self.dynamic_children.push({
                let mut new_child = Node {
                    state: DynamicState::new(name, constraint),
                    data: None,

                    static_children: SortedNode::default(),
                    dynamic_children: SortedNode::default(),
                    dynamic_children_shortcut: false,
                    wildcard_children: SortedNode::default(),
                    wildcard_children_shortcut: false,
                    end_wildcard_children: SortedNode::default(),

                    priority: 0,
                    needs_optimization: false,
                };

                new_child.insert(template, data);
                new_child
            });
        }

        self.needs_optimization = true;
    }

    fn insert_wildcard(
        &mut self,
        template: &mut Template,
        data: NodeData<'r, T>,
        name: String,
        constraint: Option<String>,
    ) {
        if let Some(child) = self
            .wildcard_children
            .find_mut(|child| child.state.name == name && child.state.constraint == constraint)
        {
            child.insert(template, data);
        } else {
            self.wildcard_children.push({
                let mut new_child = Node {
                    state: WildcardState::new(name, constraint),
                    data: None,

                    static_children: SortedNode::default(),
                    dynamic_children: SortedNode::default(),
                    dynamic_children_shortcut: false,
                    wildcard_children: SortedNode::default(),
                    wildcard_children_shortcut: false,
                    end_wildcard_children: SortedNode::default(),

                    priority: 0,
                    needs_optimization: false,
                };

                new_child.insert(template, data);
                new_child
            });
        }

        self.needs_optimization = true;
    }

    fn insert_end_wildcard(
        &mut self,
        data: NodeData<'r, T>,
        name: String,
        constraint: Option<String>,
    ) {
        if self
            .end_wildcard_children
            .iter()
            .any(|child| child.state.name == name && child.state.constraint == constraint)
        {
            return;
        }

        self.end_wildcard_children.push(Node {
            state: EndWildcardState::new(name, constraint),
            data: Some(data),

            static_children: SortedNode::default(),
            dynamic_children: SortedNode::default(),
            dynamic_children_shortcut: false,
            wildcard_children: SortedNode::default(),
            wildcard_children_shortcut: false,
            end_wildcard_children: SortedNode::default(),

            priority: 0,
            needs_optimization: false,
        });

        self.needs_optimization = true;
    }
}
