use http::{HeaderMap, HeaderValue, Method, Uri, Version};
use std::fmt::Debug;

#[derive(Clone)]
pub enum RequestConstraint {
    MethodFunction(fn(&Method) -> bool),
    UriFunction(fn(&Uri) -> bool),
    VersionFunction(fn(&Version) -> bool),
    HeadersFunction(fn(&HeaderMap<HeaderValue>) -> bool),
}

impl Debug for RequestConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MethodFunction(_) => write!(f, "RequestConstraint::MethodFunction"),
            Self::UriFunction(_) => write!(f, "RequestConstraint::UriFunction"),
            Self::VersionFunction(_) => write!(f, "RequestConstraint::VersionFunction"),
            Self::HeadersFunction(_) => write!(f, "RequestConstraint::HeadersFunction"),
        }
    }
}

impl PartialEq for RequestConstraint {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::MethodFunction(left), Self::MethodFunction(right)) => std::ptr::eq(left, right),
            (Self::UriFunction(left), Self::UriFunction(right)) => std::ptr::eq(left, right),
            (Self::VersionFunction(left), Self::VersionFunction(right)) => std::ptr::eq(left, right),
            (Self::HeadersFunction(left), Self::HeadersFunction(right)) => std::ptr::eq(left, right),
            _ => false,
        }
    }
}

impl Eq for RequestConstraint {}
