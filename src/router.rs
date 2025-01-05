use std::{
    collections::HashMap,
    fmt::Display,
    net::{Ipv4Addr, Ipv6Addr},
    sync::Arc,
};

use smallvec::{smallvec, SmallVec};

use crate::{
    constraints::Constraint,
    errors::{ConstraintError, DeleteError, InsertError},
    node::{Node, NodeData},
    parser::{ParsedTemplate, Part},
    sorted::SortedNode,
    state::RootState,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Match<'r, 'p, T> {
    pub data: &'r T,
    pub template: &'r str,
    pub expanded: Option<&'r str>,
    pub parameters: Parameters<'r, 'p>,
}

pub type Parameters<'r, 'p> = SmallVec<[(&'r str, &'p str); 4]>;

#[derive(Clone)]
pub struct StoredConstraint {
    pub type_name: &'static str,
    pub check: fn(&str) -> bool,
}

#[derive(Clone)]
pub struct Router<'r, T> {
    pub root: Node<'r, T, RootState>,
    pub constraints: HashMap<&'r str, StoredConstraint>,
}

impl<'r, T> Router<'r, T> {
    #[must_use]
    pub fn new() -> Self {
        let mut router = Self {
            root: Node {
                state: RootState::new(),
                data: None,

                static_children: SortedNode::default(),
                dynamic_constrained_children: SortedNode::default(),
                dynamic_children: SortedNode::default(),
                dynamic_children_shortcut: false,
                wildcard_constrained_children: SortedNode::default(),
                wildcard_children: SortedNode::default(),
                wildcard_children_shortcut: false,
                end_wildcard_constrained_children: SortedNode::default(),
                end_wildcard_children: SortedNode::default(),

                needs_optimization: false,
            },
            constraints: HashMap::default(),
        };

        router.constraint::<u8>().unwrap();
        router.constraint::<u16>().unwrap();
        router.constraint::<u32>().unwrap();
        router.constraint::<u64>().unwrap();
        router.constraint::<u128>().unwrap();
        router.constraint::<usize>().unwrap();
        router.constraint::<i8>().unwrap();
        router.constraint::<i16>().unwrap();
        router.constraint::<i32>().unwrap();
        router.constraint::<i64>().unwrap();
        router.constraint::<i128>().unwrap();
        router.constraint::<isize>().unwrap();
        router.constraint::<f32>().unwrap();
        router.constraint::<f64>().unwrap();
        router.constraint::<bool>().unwrap();
        router.constraint::<Ipv4Addr>().unwrap();
        router.constraint::<Ipv6Addr>().unwrap();

        router
    }

    pub fn constraint<C: Constraint>(&mut self) -> Result<(), ConstraintError> {
        if let Some(existing) = self.constraints.get(C::NAME) {
            return Err(ConstraintError::DuplicateName {
                name: C::NAME,
                existing_type: existing.type_name,
                new_type: std::any::type_name::<C>(),
            });
        }

        self.constraints.insert(
            C::NAME,
            StoredConstraint {
                type_name: std::any::type_name::<C>(),
                check: C::check,
            },
        );

        Ok(())
    }

    pub fn insert(&mut self, template: &'r str, data: T) -> Result<(), InsertError> {
        let mut parsed = ParsedTemplate::new(template.as_bytes())?;

        // Check for invalid constraints.
        for template in &parsed.templates {
            for part in &template.parts {
                if let Part::DynamicConstrained {
                    constraint: name, ..
                }
                | Part::WildcardConstrained {
                    constraint: name, ..
                } = part
                {
                    if !self.constraints.contains_key(name.as_str()) {
                        return Err(InsertError::UnknownConstraint {
                            constraint: name.to_string(),
                        });
                    }
                }
            }
        }

        // Check for any conflicts.
        for template in &parsed.templates {
            if self.root.find(&mut template.clone()).is_some() {
                return Err(InsertError::Conflict);
            }
        }

        if parsed.templates.len() > 1 {
            let data = Arc::from(data);
            for mut parsed_template in parsed.templates {
                let expanded = Arc::from(String::from_utf8_lossy(&parsed_template.raw));

                #[allow(clippy::naive_bytecount)]
                let slashes = parsed_template.raw.iter().filter(|&b| *b == b'/').count();
                let final_length = parsed_template
                    .raw
                    .rsplit(|&b| b == b'/')
                    .next()
                    .map_or(0, <[u8]>::len);

                self.root.insert(
                    &mut parsed_template,
                    NodeData::Shared {
                        data: Arc::clone(&data),
                        template,
                        expanded,
                        specificity: slashes + final_length,
                    },
                );
            }
        } else if let Some(parsed_template) = parsed.templates.first_mut() {
            #[allow(clippy::naive_bytecount)]
            let slashes = parsed_template.raw.iter().filter(|&b| *b == b'/').count();
            let final_length = parsed_template
                .raw
                .rsplit(|&b| b == b'/')
                .next()
                .map_or(0, <[u8]>::len);

            self.root.insert(
                parsed_template,
                NodeData::Inline {
                    data,
                    template,
                    specificity: slashes + final_length,
                },
            );
        };

        self.root.optimize();
        Ok(())
    }

    pub fn delete(&mut self, template: &str) -> Result<T, DeleteError> {
        let parsed = ParsedTemplate::new(template.as_bytes())?;

        // Check for any conflicts or mismatches.
        for parsed_template in &parsed.templates {
            let found = match self.root.find(&mut parsed_template.clone()) {
                Some(found) => found,
                _ => {
                    continue;
                }
            };

            if found.template() == template {
                continue;
            }

            return Err(DeleteError::Mismatch {
                template: template.to_owned(),
                inserted: found.template().to_owned(),
            });
        }

        for parsed_template in &parsed.templates {
            if self.root.find(&mut parsed_template.clone()).is_none() {
                return Err(DeleteError::NotFound {
                    template: template.to_owned(),
                });
            };
        }

        let mut output = None;
        for mut template in parsed.templates {
            if let Some(data) = self.root.delete(&mut template) {
                output = Some(data);
            }
        }

        let data = match output {
            Some(data) => data,
            _ => {
                return Err(DeleteError::NotFound {
                    template: template.to_owned(),
                });
            }
        };

        self.root.optimize();
        Ok(data)
    }

    pub fn search<'p>(&'r self, path: &'p str) -> Option<Match<'r, 'p, T>> {
        let mut parameters = smallvec![];
        let data = match self
            .root
            .search(path.as_bytes(), &mut parameters, &self.constraints)
        {
            Some(data) => data,
            _ => {
                return None;
            }
        };

        let (data, template, expanded) = match data {
            NodeData::Inline { data, template, .. } => (data, template, None),
            NodeData::Shared {
                data,
                template,
                expanded,
                ..
            } => (data.as_ref(), template, Some(expanded.as_ref())),
        };

        Some(Match {
            data,
            template,
            expanded,
            parameters,
        })
    }
}

impl<T> Display for Router<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
