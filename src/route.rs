use wayfind_authority::parser::ParsedAuthority;
use wayfind_path::parser::ParsedPath;
use wayfind_percent::percent_decode;
use wayfind_punycode::punycode_decode;

use crate::errors::{EncodingError, RouteError};

/// A route that can be inserted into a [`Router`](`crate::Router`).
#[derive(Debug, PartialEq, Eq)]
pub struct Route<'r> {
    pub(crate) authority: Option<ParsedAuthority>,
    pub(crate) route: ParsedPath,
    pub(crate) methods: Option<Vec<&'r str>>,
}

/// Builder pattern for creating a [`Route`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteBuilder<'r> {
    authority: Option<&'r str>,
    route: Option<&'r str>,
    methods: Option<Vec<&'r str>>,
}

impl<'r> RouteBuilder<'r> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            authority: None,
            route: None,
            methods: None,
        }
    }

    #[must_use]
    pub const fn authority(mut self, authority: &'r str) -> Self {
        self.authority = Some(authority);
        self
    }

    #[must_use]
    pub const fn route(mut self, route: &'r str) -> Self {
        self.route = Some(route);
        self
    }

    #[must_use]
    pub fn methods(mut self, methods: Vec<&'r str>) -> Self {
        self.methods = Some(methods);
        self
    }

    /// Builds a new [`Route`] instance from the builder.
    ///
    /// # Errors
    ///
    /// Return a [`RouteError`] if a required field was not populated.
    pub fn build(self) -> Result<Route<'r>, RouteError> {
        // Verify authority is punycode-decoded
        let authority = if let Some(authority) = self.authority {
            let decoded = punycode_decode(authority.as_bytes()).map_err(EncodingError::from)?;
            if authority != decoded {
                return Err(RouteError::EncodedAuthority {
                    input: authority.to_owned(),
                    decoded: decoded.to_string(),
                })?;
            }

            Some(ParsedAuthority::new(authority)?)
        } else {
            None
        };

        let route = self.route.ok_or(RouteError::MissingRoute)?;

        // Verify path is percent-decoded
        let decoded = percent_decode(route.as_bytes()).map_err(EncodingError::from)?;
        if route.as_bytes() != decoded.as_ref() {
            return Err(RouteError::EncodedPath {
                input: route.to_owned(),
                decoded: String::from_utf8_lossy(&decoded).to_string(),
            })?;
        }

        let route = ParsedPath::new(route)?;

        Ok(Route {
            authority,
            route,
            methods: self.methods,
        })
    }
}
