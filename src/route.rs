use crate::{
    errors::route::RouteError,
    node::{Constraint, NodeConstraint},
    parts::Parts,
};
use std::collections::HashMap;

pub struct RouteBuilder<'a> {
    pub path: &'a str,
    pub constraints: HashMap<&'a [u8], NodeConstraint>,
}

impl<'a> RouteBuilder<'a> {
    #[must_use]
    pub fn new(path: &'a str) -> Self {
        Self {
            path,
            constraints: HashMap::new(),
        }
    }

    #[must_use]
    pub fn constraint<C: Constraint>(mut self, param: &'a str) -> Self {
        self.constraints.insert(
            param.as_bytes(),
            NodeConstraint {
                name: C::name(),
                check: C::check,
            },
        );
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
    pub constraints: HashMap<&'a [u8], NodeConstraint>,
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
        RouteBuilder::new(self).build()
    }
}
