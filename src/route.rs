use crate::{errors::route::RouteError, node::NodeConstraint, parts::Parts};

pub struct RouteBuilder<'a> {
    pub path: &'a str,
    pub constraints: Vec<(&'a str, NodeConstraint)>,
}

impl<'a> RouteBuilder<'a> {
    #[must_use]
    pub const fn new(path: &'a str) -> Self {
        Self {
            path,
            constraints: vec![],
        }
    }

    #[must_use]
    pub fn constraint(mut self, name: &'a str, value: NodeConstraint) -> Self {
        self.constraints.push((name, value));
        self
    }

    pub fn build(self) -> Result<Route<'a>, RouteError> {
        Ok(Route {
            path: self.path,
            parts: Parts::new(self.path.as_bytes())?,
            constraints: self.constraints,
        })
    }
}

pub struct Route<'a> {
    pub path: &'a str,
    pub parts: Parts<'a>,
    pub constraints: Vec<(&'a str, NodeConstraint)>,
}

impl<'a> TryFrom<&'a str> for Route<'a> {
    type Error = RouteError;

    fn try_from(path: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            path,
            parts: Parts::new(path.as_bytes())?,
            constraints: vec![],
        })
    }
}

pub trait IntoRoute<'a> {
    fn into_route(self) -> Result<Route<'a>, RouteError>;
}

impl<'a> IntoRoute<'a> for Route<'a> {
    fn into_route(self) -> Result<Self, RouteError> {
        Ok(self)
    }
}

impl<'a> IntoRoute<'a> for &'a str {
    fn into_route(self) -> Result<Route<'a>, RouteError> {
        Route::try_from(self)
    }
}
