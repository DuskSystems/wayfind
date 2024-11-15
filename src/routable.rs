use crate::{
    decode::percent_decode,
    errors::{EncodingError, RoutableError},
};
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
};

/// A routable endpoint that can be inserted into a [`Router`](`crate::Router`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Routable<'r> {
    pub(crate) route: &'r str,
}

/// Builder pattern for creating a [`Routable`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutableBuilder<'r> {
    route: Option<&'r str>,
}

impl<'r> RoutableBuilder<'r> {
    #[must_use]
    pub const fn new() -> Self {
        Self { route: None }
    }

    #[must_use]
    pub const fn route(mut self, route: &'r str) -> Self {
        self.route = Some(route);
        self
    }

    /// Builds a new [`Routable`] instance from the builder.
    ///
    /// # Errors
    ///
    /// Return a [`RoutableError`] if a required field was not populated.
    pub fn build(self) -> Result<Routable<'r>, RoutableError> {
        let route = self.route.ok_or(RoutableError::MissingRoute)?;

        let decoded = percent_decode(route.as_bytes())?;
        if route.as_bytes() != decoded.as_ref() {
            return Err(EncodingError::EncodedRoute {
                input: route.to_owned(),
                decoded: String::from_utf8_lossy(&decoded).to_string(),
            })?;
        }

        Ok(Routable { route })
    }
}

impl<'r> Default for RoutableBuilder<'r> {
    fn default() -> Self {
        Self::new()
    }
}
