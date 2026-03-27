use alloc::borrow::ToOwned as _;
use alloc::vec::Vec;
use core::fmt;

use hashbrown::HashMap;
use rustc_hash::FxBuildHasher;
use slab::Slab;
use smallvec::SmallVec;

use crate::errors::{DeleteError, InsertError};
use crate::node::search::SearchState;
use crate::node::{Node, NodeData};
use crate::parser::Template;
use crate::state::RootState;

/// Stores data from a successful router match.
#[derive(Eq, PartialEq, Debug)]
pub struct Match<'r, 'p, T> {
    /// A reference to the matching template data.
    pub data: &'r T,

    /// The matching template.
    pub template: &'r str,

    /// Key-value pairs of parameters.
    /// The key is tied to the lifetime of the router.
    /// The value is tied to the lifetime of the path.
    pub parameters: SmallVec<[(&'r str, &'p str); 4]>,
}

/// A mutable router builder for inserting and deleting routes.
///
/// Call [`build`](RouterBuilder::build) to produce an immutable [`Router`] for searching.
///
/// # Examples
///
/// ```rust
/// use wayfind::RouterBuilder;
///
/// let mut builder = RouterBuilder::new();
/// builder.insert("/hello", 1).unwrap();
/// let router = builder.build();
/// ```
#[derive(Clone)]
pub struct RouterBuilder<T> {
    /// The root node of the tree.
    root: Node<RootState>,

    /// Needle-to-ID mapping.
    needles: HashMap<Vec<u8>, usize, FxBuildHasher>,

    /// Keyed storage map containing the inserted data.
    storage: Slab<T>,
}

impl<T> RouterBuilder<T> {
    /// Creates a new router builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: Node::new(RootState::new()),
            needles: HashMap::with_hasher(FxBuildHasher),
            storage: Slab::new(),
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
    /// use wayfind::RouterBuilder;
    ///
    /// let mut builder: RouterBuilder<usize> = RouterBuilder::new();
    /// builder.insert("/hello", 1).unwrap();
    /// ```
    pub fn insert(&mut self, template: &str, data: T) -> Result<(), InsertError> {
        let mut parsed =
            Template::new(template.as_bytes()).map_err(|error| InsertError::Template {
                template: template.to_owned(),
                error,
            })?;

        // Check for conflicts up front to prevent partial inserts.
        if let Some(found) = self.root.conflict(&parsed.parts) {
            return Err(InsertError::Conflict {
                new: template.to_owned(),
                existing: found.template.clone().into(),
            });
        }

        let key = self.storage.insert(data);
        self.root.insert(
            &mut parsed,
            NodeData {
                key,
                template: template.into(),
            },
        );

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
    /// use wayfind::RouterBuilder;
    ///
    /// let mut builder: RouterBuilder<usize> = RouterBuilder::new();
    /// builder.insert("/hello", 1).unwrap();
    /// builder.delete("/hello").unwrap();
    /// ```
    pub fn delete(&mut self, template: &str) -> Result<T, DeleteError> {
        let mut parsed =
            Template::new(template.as_bytes()).map_err(|error| DeleteError::Template {
                template: template.to_owned(),
                error,
            })?;

        let Some(data) = self.root.delete(&mut parsed) else {
            return Err(DeleteError::NotFound {
                template: template.to_owned(),
            });
        };

        let entry = self.storage.remove(data.key);
        Ok(entry)
    }

    /// Checks if a template exists in the builder and returns a reference to its data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::RouterBuilder;
    ///
    /// let mut builder: RouterBuilder<usize> = RouterBuilder::new();
    /// builder.insert("/hello", 1).unwrap();
    /// assert_eq!(builder.get("/hello").unwrap(), &1);
    /// ```
    #[must_use]
    pub fn get(&self, template: &str) -> Option<&T> {
        let Ok(parsed) = Template::new(template.as_bytes()) else {
            return None;
        };

        let found = self.root.find(&parsed.parts)?;
        if *found.template == *template {
            self.storage.get(found.key)
        } else {
            None
        }
    }

    /// Checks if a template exists in the builder and returns a mutable reference to its data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::RouterBuilder;
    ///
    /// let mut builder: RouterBuilder<usize> = RouterBuilder::new();
    /// builder.insert("/hello", 1).unwrap();
    /// if let Some(data) = builder.get_mut("/hello") {
    ///     *data = 2;
    /// }
    ///
    /// assert_eq!(builder.get("/hello").unwrap(), &2);
    /// ```
    #[must_use]
    pub fn get_mut(&mut self, template: &str) -> Option<&mut T> {
        let Ok(parsed) = Template::new(template.as_bytes()) else {
            return None;
        };

        let found = self.root.find(&parsed.parts)?;
        if *found.template == *template {
            self.storage.get_mut(found.key)
        } else {
            None
        }
    }

    /// Optimizes the tree and produces an immutable [`Router`] for searching.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::RouterBuilder;
    ///
    /// let mut builder = RouterBuilder::new();
    /// builder.insert("/users/<id>", 1).unwrap();
    /// builder.insert("/posts/<id>", 2).unwrap();
    /// let router = builder.build();
    /// router.search("/users/123").unwrap();
    /// ```
    #[must_use]
    pub fn build(mut self) -> Router<T> {
        let mut counter = 0;
        self.root.optimize(&mut self.needles, &mut counter);

        Router {
            root: self.root,
            count: counter,
            needles: self.needles.len(),
            storage: self.storage,
        }
    }
}

impl<T> Default for RouterBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// An immutable, optimized router for searching routes.
///
/// Produced by [`RouterBuilder::build`]. Supports searching and data access,
/// but not insertion or deletion.
///
/// Call [`into_builder`](Router::into_builder) to convert back to a [`RouterBuilder`]
/// for modification.
///
/// See [the crate documentation](crate) for usage.
#[derive(Clone)]
pub struct Router<T> {
    /// The root node of the tree.
    root: Node<RootState>,

    /// The number of nodes in the tree.
    count: usize,

    /// The number of unique needles (for pre-allocating search state).
    needles: usize,

    /// Keyed storage map containing the inserted data.
    storage: Slab<T>,
}

impl<T> Router<T> {
    /// Creates a new [`RouterBuilder`].
    #[must_use]
    pub fn builder() -> RouterBuilder<T> {
        RouterBuilder::new()
    }

    /// Converts this router back into a [`RouterBuilder`] for modification.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::RouterBuilder;
    ///
    /// let mut builder = RouterBuilder::new();
    /// builder.insert("/hello", 1).unwrap();
    /// let router = builder.build();
    ///
    /// let mut builder = router.into_builder();
    /// builder.insert("/world", 2).unwrap();
    /// let router = builder.build();
    /// ```
    #[must_use]
    pub fn into_builder(self) -> RouterBuilder<T> {
        RouterBuilder {
            root: self.root,
            needles: HashMap::with_hasher(FxBuildHasher),
            storage: self.storage,
        }
    }

    /// Checks if a template exists in the router and returns a reference to its data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::RouterBuilder;
    ///
    /// let mut builder: RouterBuilder<usize> = RouterBuilder::new();
    /// builder.insert("/hello", 1).unwrap();
    /// let router = builder.build();
    /// assert_eq!(router.get("/hello").unwrap(), &1);
    /// ```
    #[must_use]
    pub fn get(&self, template: &str) -> Option<&T> {
        let Ok(parsed) = Template::new(template.as_bytes()) else {
            return None;
        };

        let found = self.root.find(&parsed.parts)?;
        if *found.template == *template {
            self.storage.get(found.key)
        } else {
            None
        }
    }

    /// Checks if a template exists in the router and returns a mutable reference to its data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::RouterBuilder;
    ///
    /// let mut builder: RouterBuilder<usize> = RouterBuilder::new();
    /// builder.insert("/hello", 1).unwrap();
    /// let mut router = builder.build();
    /// if let Some(data) = router.get_mut("/hello") {
    ///     *data = 2;
    /// }
    ///
    /// assert_eq!(router.get("/hello").unwrap(), &2);
    /// ```
    #[must_use]
    pub fn get_mut(&mut self, template: &str) -> Option<&mut T> {
        let Ok(parsed) = Template::new(template.as_bytes()) else {
            return None;
        };

        let found = self.root.find(&parsed.parts)?;
        if *found.template == *template {
            self.storage.get_mut(found.key)
        } else {
            None
        }
    }

    /// Searches for a matching template in the router for a path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::RouterBuilder;
    ///
    /// let mut builder: RouterBuilder<usize> = RouterBuilder::new();
    /// builder.insert("/<user>", 1).unwrap();
    /// let router = builder.build();
    /// router.search("/me").unwrap();
    /// ```
    #[must_use]
    pub fn search<'r, 'p>(&'r self, path: &'p str) -> Option<Match<'r, 'p, T>> {
        let mut parameters = SmallVec::new();
        let mut state = SearchState::new(self.count, self.needles);
        let data = self.root.search(path, &mut parameters, &mut state)?;

        Some(Match {
            data: self.storage.get(data.key)?,
            template: &data.template,
            parameters,
        })
    }
}

impl<T> fmt::Display for Router<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}
