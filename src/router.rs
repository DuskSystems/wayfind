use alloc::borrow::ToOwned as _;
use core::fmt;

use smallvec::SmallVec;

use crate::errors::InsertError;
use crate::node::{Data, Node, Search};
use crate::parser::Template;
use crate::state::RootState;

/// Stores data from a successful router match.
#[derive(Debug)]
pub struct Match<'r, 'p, T> {
    /// A reference to the matching template data.
    data: &'r T,

    /// The matching template.
    template: &'r str,

    /// Key-value pairs of parameters.
    /// The key is tied to the lifetime of the router.
    /// The value is tied to the lifetime of the path.
    parameters: SmallVec<[(&'r str, &'p str); 4]>,
}

impl<'r, 'p, T> Match<'r, 'p, T> {
    /// Returns a reference to the data associated with the matched template.
    #[must_use]
    pub const fn data(&self) -> &'r T {
        self.data
    }

    /// Returns the matched template string.
    #[must_use]
    pub const fn template(&self) -> &'r str {
        self.template
    }

    /// Returns the matched parameters as key-value pairs.
    #[must_use]
    pub fn parameters(&self) -> &[(&'r str, &'p str)] {
        &self.parameters
    }
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
/// builder.insert("/hello", 1)?;
///
/// let router = builder.build();
/// # Ok::<_, Box<dyn core::error::Error>>(())
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
    /// builder.insert("/hello", 1)?;
    /// # Ok::<_, Box<dyn core::error::Error>>(())
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
    /// builder.insert("/users/<id>", 1)?;
    /// builder.insert("/posts/<id>", 2)?;
    ///
    /// let router = builder.build();
    ///
    /// let search = router.search("/users/123").ok_or("no match")?;
    /// # Ok::<_, Box<dyn core::error::Error>>(())
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
    /// builder.insert("/<user>", 1)?;
    ///
    /// let router = builder.build();
    ///
    /// let search = router.search("/me").ok_or("no match")?;
    /// # Ok::<_, Box<dyn core::error::Error>>(())
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
