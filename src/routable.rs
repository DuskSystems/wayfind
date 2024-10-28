use crate::errors::RoutableError;

/// A routable endpoint that can be inserted into a [`Router`](`crate::Router`).
#[derive(Debug, Clone)]
pub struct Routable<'a> {
    pub(crate) route: &'a str,
}

impl<'a> Routable<'a> {
    #[must_use]
    pub const fn builder<'b>() -> RoutableBuilder<'b> {
        RoutableBuilder::new()
    }
}

impl<'a> From<&'a str> for Routable<'a> {
    fn from(value: &'a str) -> Self {
        Self { route: value }
    }
}

/// Builder pattern for creating a [`Routable`].
#[derive(Debug, Clone)]
pub struct RoutableBuilder<'a> {
    route: Option<&'a str>,
}

impl<'a> RoutableBuilder<'a> {
    #[must_use]
    pub const fn new() -> Self {
        Self { route: None }
    }

    #[must_use]
    pub const fn route(mut self, route: &'a str) -> Self {
        self.route = Some(route);
        self
    }

    /// Builds a new [`Routable`] instance from the builder.
    ///
    /// # Errors
    ///
    /// Return a [`RoutableError`] if a required field was not populated.
    pub fn build(self) -> Result<Routable<'a>, RoutableError> {
        let route = self.route.ok_or(RoutableError::MissingRoute)?;
        Ok(Routable { route })
    }
}

impl<'a> Default for RoutableBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
