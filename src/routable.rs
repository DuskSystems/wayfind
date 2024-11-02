use crate::errors::RoutableError;

/// A routable endpoint that can be inserted into a [`Router`](`crate::Router`).
#[derive(Debug, Clone)]
pub struct Routable<'r> {
    pub(crate) route: &'r str,
}

impl<'r> Routable<'r> {
    #[must_use]
    pub const fn builder<'b>() -> RoutableBuilder<'b> {
        RoutableBuilder::new()
    }
}

impl<'r> From<&'r str> for Routable<'r> {
    fn from(value: &'r str) -> Self {
        Self { route: value }
    }
}

/// Builder pattern for creating a [`Routable`].
#[derive(Debug, Clone)]
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
        Ok(Routable { route })
    }
}

impl<'r> Default for RoutableBuilder<'r> {
    fn default() -> Self {
        Self::new()
    }
}
