use core::fmt;

use smallvec::SmallVec;

use crate::node::{Node, SearchContext};
use crate::state::RootState;

/// Stores data from a successful router match.
#[derive(Debug)]
pub struct Match<'r, 'p, T> {
    data: &'r T,
    template: &'r str,
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

/// An immutable, optimized router.
#[derive(Clone)]
pub struct Router<T> {
    root: Node<RootState, T>,
}

impl<T> Router<T> {
    pub(crate) const fn new(root: Node<RootState, T>) -> Self {
        Self { root }
    }

    /// Searches for a matching template in the router.
    ///
    /// Returns `None` if no template matches the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::RouterBuilder;
    ///
    /// let mut builder = RouterBuilder::new();
    /// builder.insert("/users/<id>", 1)?;
    ///
    /// let router = builder.build();
    ///
    /// let search = router.search("/users/123").unwrap();
    /// assert_eq!(search.data(), &1);
    /// assert_eq!(search.template(), "/users/<id>");
    /// assert_eq!(search.parameters(), &[("id", "123")]);
    ///
    /// assert!(router.search("/not/found").is_none());
    /// # Ok::<_, Box<dyn core::error::Error>>(())
    /// ```
    #[must_use]
    pub fn search<'r, 'p>(&'r self, path: &'p str) -> Option<Match<'r, 'p, T>> {
        let mut ctx = SearchContext::new();
        let node = self.root.search(&mut ctx, path)?;

        Some(Match {
            data: &node.data,
            template: &node.template,
            parameters: ctx.parameters,
        })
    }
}

impl<T> fmt::Display for Router<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}
