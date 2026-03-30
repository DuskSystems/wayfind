use alloc::borrow::ToOwned as _;
use core::fmt;

use smallvec::SmallVec;

use crate::errors::InsertError;
use crate::node::{Data, Node, Search};
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

/// A mutable router builder for inserting routes.
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
    root: Node<RootState, T>,
}

impl<T> RouterBuilder<T> {
    /// Creates a new router builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: Node::new(RootState::new()),
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

        self.root.insert(
            &mut parsed,
            Data {
                data,
                template: template.into(),
            },
        );

        Ok(())
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
        self.root.optimize();
        Router { root: self.root }
    }
}

impl<T> Default for RouterBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// An immutable, optimized router for searching routes.
///
/// Produced by [`RouterBuilder::build`].
///
/// See [the crate documentation](crate) for usage.
#[derive(Clone)]
pub struct Router<T> {
    /// The root node of the tree.
    root: Node<RootState, T>,
}

impl<T> Router<T> {
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
        let mut search = Search::new();
        let node = self.root.search(&mut search, path)?;

        Some(Match {
            data: &node.data,
            template: &node.template,
            parameters: search.parameters,
        })
    }
}

impl<T> fmt::Display for Router<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}
