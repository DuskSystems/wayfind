use http::Request;
use std::fmt::Debug;

#[derive(Clone)]
pub enum RequestConstraint<R> {
    Function(fn(&Request<R>) -> bool),
}

impl<R> Debug for RequestConstraint<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(_) => write!(f, "RequestConstraint::Function"),
        }
    }
}

impl<R> PartialEq for RequestConstraint<R> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Function(left), Self::Function(right)) => std::ptr::eq(left, right),
        }
    }
}

impl<R> Eq for RequestConstraint<R> {}
