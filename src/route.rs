use crate::{
    constraints::{parameter::ParameterConstraint, request::RequestConstraint},
    errors::route::RouteError,
    parts::Parts,
};
use std::collections::HashMap;

pub struct RouteBuilder<'a, R> {
    pub path: &'a str,

    pub parameter_constraints: HashMap<&'a [u8], Vec<ParameterConstraint>>,
    pub request_constraints: Vec<RequestConstraint<R>>,
}

impl<'a, R> RouteBuilder<'a, R> {
    #[must_use]
    pub fn new(path: &'a str) -> Self {
        Self {
            path,

            parameter_constraints: HashMap::new(),
            request_constraints: vec![],
        }
    }

    #[must_use]
    pub fn parameter_constraint(mut self, name: &'a str, constraint: ParameterConstraint) -> Self {
        self.parameter_constraints
            .entry(name.as_bytes())
            .or_default()
            .push(constraint);

        self
    }

    #[must_use]
    pub fn request_constraint(mut self, constraint: RequestConstraint<R>) -> Self {
        self.request_constraints
            .push(constraint);

        self
    }

    pub fn build(self) -> Result<Route<'a, R>, RouteError> {
        Ok(Route {
            path: self.path,
            parts: Parts::new(self.path.as_bytes())?,

            parameter_constraints: self.parameter_constraints,
            request_constraints: self.request_constraints,
        })
    }
}

pub struct Route<'a, R> {
    pub path: &'a str,
    pub parts: Parts<'a>,

    pub parameter_constraints: HashMap<&'a [u8], Vec<ParameterConstraint>>,
    pub request_constraints: Vec<RequestConstraint<R>>,
}

impl<'a, R> TryFrom<&'a str> for Route<'a, R> {
    type Error = RouteError;

    fn try_from(path: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            path,
            parts: Parts::new(path.as_bytes())?,

            parameter_constraints: HashMap::new(),
            request_constraints: vec![],
        })
    }
}

pub trait IntoRoute<'a, R> {
    fn into_route(self) -> Result<Route<'a, R>, RouteError>;
}

impl<'a, R> IntoRoute<'a, R> for Route<'a, R> {
    fn into_route(self) -> Result<Self, RouteError> {
        Ok(self)
    }
}

impl<'a, R> IntoRoute<'a, R> for &'a str {
    fn into_route(self) -> Result<Route<'a, R>, RouteError> {
        Route::try_from(self)
    }
}
