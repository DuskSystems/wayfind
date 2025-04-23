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
    nodes::Nodes,
    parser::{ParsedTemplate, Part},
    state::RootState,
};

/// Stores data from a successful router match.
#[derive(Debug, Eq, PartialEq)]
pub struct Match<'r, 'p, T> {
    /// A reference to the matching template data.
    pub data: &'r T,

    /// The matching template.
    pub template: &'r str,

    /// The matching expanded template, if applicable.
    pub expanded: Option<&'r str>,

    /// Key-value pairs of parameters, extracted from the template and path.
    pub parameters: Parameters<'r, 'p>,
}

/// All the parameter pairs of a given match.
///
/// The key of the parameter is tied to the lifetime of the router, since it is a ref to the template of a given node.
/// The value is extracted from the path.
pub type Parameters<'r, 'p> = SmallVec<[(&'r str, &'p str); 4]>;

/// A constraint function with its type name.
#[derive(Clone)]
pub struct StoredConstraint {
    pub type_name: &'static str,
    pub check: fn(&str) -> bool,
}

/// The [`wayfind`](crate) router.
///
/// See [the crate documentation](crate) for usage.
#[derive(Clone)]
pub struct Router<T> {
    /// The root node of the tree.
    root: Node<T, RootState>,

    /// A map of constraint names to [`StoredConstraint`].
    constraints: HashMap<&'static str, StoredConstraint>,
}

impl<T> Router<T> {
    /// Creates a new Router with default constraints.
    ///
    /// # Panics
    ///
    /// Can only panic if the default constraint registrations fail, which should never happen.
    #[must_use]
    pub fn new() -> Self {
        let mut router = Self {
            root: Node {
                state: RootState::new(),
                data: None,

                static_children: Nodes::default(),
                dynamic_constrained_children: Nodes::default(),
                dynamic_children: Nodes::default(),
                dynamic_children_shortcut: false,
                wildcard_constrained_children: Nodes::default(),
                wildcard_children: Nodes::default(),
                wildcard_children_shortcut: false,
                end_wildcard_constrained_children: Nodes::default(),
                end_wildcard_children: Nodes::default(),

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

    /// Registers a new constraint to the router.
    ///
    /// # Errors
    ///
    /// Returns a [`ConstraintError`] if the constraint could not be added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::{Constraint, Router};
    ///
    /// struct HelloConstraint;
    /// impl Constraint for HelloConstraint {
    ///     const NAME: &'static str = "hello";
    ///
    ///     fn check(segment: &str) -> bool {
    ///         segment == "hello"
    ///     }
    /// }
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.constraint::<HelloConstraint>().unwrap();
    /// ```
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

    /// Inserts a new template with associated data into the router.
    ///
    /// # Errors
    ///
    /// Returns an [`InsertError`] if the template is invalid or cannot be inserted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::{Constraint, Router};
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    /// ```
    pub fn insert(&mut self, template: &str, data: T) -> Result<(), InsertError> {
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
        let mut conflicts = vec![];
        for parsed_template in &parsed.templates {
            if let Some(found) = self.root.find(&mut parsed_template.clone()) {
                conflicts.push(found.template().to_owned());
            }
        }

        if !conflicts.is_empty() {
            conflicts.dedup();
            conflicts.sort();

            return Err(InsertError::Conflict {
                template: template.to_owned(),
                conflicts,
            });
        }

        // All good, proceed with insert.
        let template = Arc::from(template);

        if parsed.templates.len() > 1 {
            let data = Arc::from(data);
            for mut parsed_template in parsed.templates {
                let expanded = Arc::from(String::from_utf8_lossy(&parsed_template.raw));

                #[allow(clippy::naive_bytecount)]
                let depth = parsed_template.raw.iter().filter(|&b| *b == b'/').count();
                let length = parsed_template.raw.len();

                self.root.insert(
                    &mut parsed_template,
                    NodeData::Shared {
                        data: Arc::clone(&data),
                        template: Arc::clone(&template),
                        expanded,
                        depth,
                        length,
                    },
                );
            }
        } else if let Some(parsed_template) = parsed.templates.first_mut() {
            #[allow(clippy::naive_bytecount)]
            let depth = parsed_template.raw.iter().filter(|&b| *b == b'/').count();
            let length = parsed_template.raw.len();

            self.root.insert(
                parsed_template,
                NodeData::Inline {
                    data,
                    template,
                    depth,
                    length,
                },
            );
        }

        self.root.optimize();
        Ok(())
    }

    /// Deletes a template from the router.
    ///
    /// The template provided must exactly match the template inserted.
    ///
    /// Returns the associated data previously stored.
    ///
    /// # Errors
    ///
    /// Returns a [`DeleteError`] if the template is invalid, cannot be deleted, or cannot be found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::{Constraint, Router};
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    /// router.delete("/hello").unwrap();
    /// ```
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
            }
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

    /// Searches for a matching template in the router for a path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::{Constraint, Router};
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/{user}", 1).unwrap();
    /// router.search("/me").unwrap();
    /// ```
    pub fn search<'r, 'p>(&'r self, path: &'p str) -> Option<Match<'r, 'p, T>> {
        let mut parameters = smallvec![];
        let search = match self
            .root
            .search(path.as_bytes(), &mut parameters, &self.constraints)
        {
            Some(data) => data,
            _ => {
                return None;
            }
        };

        Some(Match {
            data: search.data(),
            template: search.template(),
            expanded: search.expanded(),
            parameters,
        })
    }
}

impl<T> Display for Router<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
