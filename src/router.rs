use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec,
};
use core::fmt;

use smallvec::{SmallVec, smallvec};

use crate::{
    errors::{DeleteError, InsertError},
    node::{Node, NodeData},
    nodes::Nodes,
    parser::ParsedTemplate,
    state::RootState,
    storage::Storage,
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

/// The [`wayfind`](crate) router.
///
/// See [the crate documentation](crate) for usage.
#[derive(Clone)]
pub struct Router<T> {
    /// The root node of the tree.
    root: Node<RootState>,

    /// Keyed storage map containing the inserted data.
    storage: Storage<T>,
}

impl<T> Router<T> {
    /// Creates a new Router.
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: Node {
                state: RootState::new(),
                data: None,

                static_children: Nodes::default(),
                dynamic_children: Nodes::default(),
                dynamic_children_shortcut: false,
                wildcard_children: Nodes::default(),
                wildcard_children_shortcut: false,
                end_wildcard_children: Nodes::default(),

                needs_optimization: false,
            },
            storage: Storage::new(),
        }
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
    /// use wayfind::Router;
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    /// ```
    pub fn insert(&mut self, template: &str, data: T) -> Result<(), InsertError> {
        let mut parsed = ParsedTemplate::new(template.as_bytes())?;

        // Check for any conflicts.
        let mut conflicts = vec![];
        for parsed_template in &parsed.templates {
            if let Some(found) = self.root.find(&mut parsed_template.clone()) {
                conflicts.push(found.template.to_string());
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
        let key = self.storage.insert(data);

        if parsed.templates.len() > 1 {
            for mut parsed_template in parsed.templates {
                let expanded = Some(String::from_utf8_lossy(&parsed_template.raw).to_string());

                #[allow(clippy::naive_bytecount)]
                let depth = parsed_template.raw.iter().filter(|&b| *b == b'/').count();
                let length = parsed_template.raw.len();

                self.root.insert(
                    &mut parsed_template,
                    NodeData {
                        key,
                        template: template.to_owned(),
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
                NodeData {
                    key,
                    template: template.to_owned(),
                    expanded: None,
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
    /// use wayfind::Router;
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    /// router.delete("/hello").unwrap();
    /// ```
    pub fn delete(&mut self, template: &str) -> Result<T, DeleteError> {
        let parsed = ParsedTemplate::new(template.as_bytes())?;

        // Check for any conflicts or mismatches.
        for parsed_template in &parsed.templates {
            let Some(found) = self.root.find(&mut parsed_template.clone()) else {
                continue;
            };

            if found.template == template {
                continue;
            }

            return Err(DeleteError::Mismatch {
                template: template.to_owned(),
                inserted: found.template.to_string(),
            });
        }

        for parsed_template in &parsed.templates {
            if self.root.find(&mut parsed_template.clone()).is_none() {
                return Err(DeleteError::NotFound {
                    template: template.to_owned(),
                });
            }
        }

        let mut key = None;
        for mut template in parsed.templates {
            if let Some(data) = self.root.delete(&mut template) {
                key = Some(data.key);
            }
        }

        let Some(key) = key else {
            return Err(DeleteError::NotFound {
                template: template.to_owned(),
            });
        };

        let Some(data) = self.storage.remove(key) else {
            return Err(DeleteError::NotFound {
                template: template.to_owned(),
            });
        };

        self.root.optimize();
        Ok(data)
    }

    /// Checks if a template exists in the router and returns a reference to its data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::Router;
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    /// assert_eq!(router.get("/hello").unwrap(), &1);
    /// ```
    #[must_use]
    pub fn get(&self, template: &str) -> Option<&T> {
        let Ok(parsed) = ParsedTemplate::new(template.as_bytes()) else {
            return None;
        };

        for parsed_template in &parsed.templates {
            if let Some(found) = self.root.find(&mut parsed_template.clone()) {
                if found.template == template {
                    return self.storage.get(found.key);
                }
            }
        }

        None
    }

    /// Checks if a template exists in the router and returns a mutable reference to its data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::Router;
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    /// if let Some(data) = router.get_mut("/hello") {
    ///     *data = 2;
    /// }
    ///
    /// assert_eq!(router.get("/hello").unwrap(), &2);
    /// ```
    #[must_use]
    pub fn get_mut(&mut self, template: &str) -> Option<&mut T> {
        let Ok(parsed) = ParsedTemplate::new(template.as_bytes()) else {
            return None;
        };

        for parsed_template in &parsed.templates {
            if let Some(found) = self.root.find(&mut parsed_template.clone()) {
                if found.template == template {
                    return self.storage.get_mut(found.key);
                }
            }
        }

        None
    }

    /// Searches for a matching template in the router for a path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::Router;
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/<user>", 1).unwrap();
    /// router.search("/me").unwrap();
    /// ```
    #[must_use]
    pub fn search<'r, 'p>(&'r self, path: &'p str) -> Option<Match<'r, 'p, T>> {
        let mut parameters = smallvec![];
        let data = self.root.search(path.as_bytes(), &mut parameters)?;

        Some(Match {
            data: self.storage.get(data.key)?,
            template: data.template.as_ref(),
            expanded: data.expanded.as_deref(),
            parameters,
        })
    }
}

impl<T> fmt::Display for Router<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}
