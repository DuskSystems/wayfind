use super::{
    node::Node,
    parser::{ParsedRoute, Part},
    state::State,
    PathData,
};

impl<'r, S: State> Node<'r, S> {
    pub fn find(&'r self, route: &mut ParsedRoute) -> Option<&'r PathData<'r>> {
        if route.parts.is_empty() {
            return self.data.as_ref();
        }

        match route.parts.pop()? {
            Part::Static { prefix } => {
                for child in self.static_children.iter() {
                    if prefix[0] == child.state.prefix[0] {
                        let common_prefix = prefix
                            .iter()
                            .zip(child.state.prefix.iter())
                            .take_while(|(a, b)| a == b)
                            .count();

                        if common_prefix >= child.state.prefix.len() {
                            if common_prefix >= prefix.len() {
                                if let Some(id) = child.find(route) {
                                    return Some(id);
                                }
                            } else {
                                route.parts.push(Part::Static {
                                    prefix: prefix[common_prefix..].to_vec(),
                                });

                                if let Some(id) = child.find(route) {
                                    return Some(id);
                                }

                                route.parts.pop();
                            }
                        }
                    }
                }

                None
            }
            Part::Dynamic {
                name, constraint, ..
            } => {
                for child in self.dynamic_children.iter() {
                    if child.state.name == name && child.state.constraint == constraint {
                        if let Some(id) = child.find(route) {
                            return Some(id);
                        }
                    }
                }

                None
            }
            Part::Wildcard {
                name, constraint, ..
            } if route.parts.is_empty() => {
                for child in self.end_wildcard_children.iter() {
                    if child.state.name == name && child.state.constraint == constraint {
                        return child.data.as_ref();
                    }
                }

                None
            }
            Part::Wildcard {
                name, constraint, ..
            } => {
                for child in self.wildcard_children.iter() {
                    if child.state.name == name && child.state.constraint == constraint {
                        if let Some(id) = child.find(route) {
                            return Some(id);
                        }
                    }
                }

                None
            }
        }
    }
}
