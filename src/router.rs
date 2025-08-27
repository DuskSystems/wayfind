use alloc::{borrow::ToOwned, string::ToString, vec};
use core::fmt;

use slab::Slab;
use smallvec::{SmallVec, smallvec};

use crate::{
    errors::{DeleteError, InsertError},
    node::{Node, NodeData},
    parser::Template,
    priority::Priority,
    state::RootState,
};

/// Stores data from a successful router match.
#[derive(Eq, PartialEq, Debug)]
pub struct Match<'r, 'p, T> {
    /// A reference to the matching template data.
    pub data: &'r T,

    /// The matching template.
    pub template: &'r str,

    /// Key-value pairs of parameters.
    /// The key of the parameter is tied to the lifetime of the.
    /// The value is extracted from the path.
    pub parameters: SmallVec<[(&'r str, &'p str); 4]>,
}

/// The [`wayfind`](crate) router.
///
/// See [the crate documentation](crate) for usage.
#[derive(Clone)]
pub struct Router<T> {
    /// The root node of the tree.
    root: Node<RootState>,

    /// Keyed storage map containing the inserted data.
    storage: Slab<T>,
}

impl<T> Router<T> {
    /// Creates a new Router.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            root: Node {
                state: RootState::new(),
                data: None,

                static_children: vec![],
                dynamic_children: vec![],
                wildcard_children: vec![],
                end_wildcard: None,

                dynamic_segment_only: false,
                wildcard_segment_only: false,

                needs_optimization: false,
            },
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
    /// use wayfind::Router;
    ///
    /// let mut router: Router<usize> = Router::new();
    /// router.insert("/hello", 1).unwrap();
    /// ```
    pub fn insert(&mut self, template: &str, data: T) -> Result<(), InsertError> {
        let mut parsed = Template::new(template.as_bytes())?;

        // Check for conflicts up front.
        // Prevent partial inserts.
        if let Some(found) = self.root.conflict(&mut parsed.clone()) {
            return Err(InsertError::Conflict {
                template: template.to_owned(),
                conflict: found.template.to_string(),
            });
        }

        let key = self.storage.insert(data);
        self.root.insert(
            &mut parsed,
            NodeData {
                key,
                template: template.to_owned(),
                priority: Priority::default(),
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

        let entry = self.storage.remove(data.key);

        self.root.optimize();
        Ok(entry)
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
