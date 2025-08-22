use alloc::{borrow::ToOwned, string::ToString};
use core::fmt;

use smallvec::{SmallVec, smallvec};

use crate::{
    errors::{DeleteError, InsertError},
    node::{Node, NodeData},
    nodes::Nodes,
    parser::Template,
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
        let mut parsed = Template::new(template.as_bytes())?;

        // Check for any conflicts.
        if let Some(found) = self.root.conflict(&mut parsed.clone()) {
            return Err(InsertError::Conflict {
                template: template.to_owned(),
                conflict: found.template.to_string(),
            });
        }

        // All good, proceed with insert.
        let key = self.storage.insert(data);

        let depth = template.bytes().filter(|&b| b == b'/').count();
        let length = template.len();

        self.root.insert(
            &mut parsed,
            NodeData {
                key,
                template: template.to_owned(),
                depth,
                length,
            },
        );

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
        let mut parsed = Template::new(template.as_bytes())?;

        let Some(data) = self.root.delete(&mut parsed) else {
            return Err(DeleteError::NotFound {
                template: template.to_owned(),
            });
        };

        let Some(stored_data) = self.storage.remove(data.key) else {
            return Err(DeleteError::NotFound {
                template: template.to_owned(),
            });
        };

        self.root.optimize();
        Ok(stored_data)
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
        let Ok(mut parsed) = Template::new(template.as_bytes()) else {
            return None;
        };

        let found = self.root.find(&mut parsed)?;
        if found.template == template {
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
        let Ok(mut parsed) = Template::new(template.as_bytes()) else {
            return None;
        };

        let found = self.root.find(&mut parsed)?;
        if found.template == template {
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
            parameters,
        })
    }
}

impl<T> fmt::Display for Router<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}
