use crate::{decode::percent_decode, errors::RouteError};
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
};

/// A route that can be inserted into a [`Router`](`crate::Router`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route<'r> {
    pub(crate) route: &'r str,
}

/// Builder pattern for creating a [`Route`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteBuilder<'r> {
    route: Option<&'r str>,
}

impl<'r> RouteBuilder<'r> {
    #[must_use]
    pub const fn new() -> Self {
        Self { route: None }
    }

    #[must_use]
    pub const fn route(mut self, route: &'r str) -> Self {
        self.route = Some(route);
        self
    }

    /// Builds a new [`Route`] instance from the builder.
    ///
    /// # Errors
    ///
    /// Return a [`RouteError`] if a required field was not populated.
    pub fn build(self) -> Result<Route<'r>, RouteError> {
        let route = self.route.ok_or(RouteError::MissingRoute)?;

        let decoded = percent_decode(route.as_bytes())?;
        if route.as_bytes() != decoded.as_ref() {
            return Err(RouteError::EncodedRoute {
                input: route.to_owned(),
                decoded: String::from_utf8_lossy(&decoded).to_string(),
            })?;
        }

        Ok(Route { route })
    }
}
