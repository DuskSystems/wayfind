use crate::{decode::percent_decode, errors::RequestError};
use std::{borrow::Cow, fmt::Debug};

#[derive(Clone, Eq, PartialEq)]
pub struct Request<'r> {
    path: Cow<'r, [u8]>,
    method: Option<&'r str>,
}

impl Request<'_> {
    #[must_use]
    pub fn path(&self) -> &[u8] {
        self.path.as_ref()
    }

    #[must_use]
    pub const fn method(&self) -> Option<&str> {
        self.method
    }
}

impl Debug for Request<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("path", &String::from_utf8_lossy(&self.path))
            .field("method", &self.method)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestBuilder<'p> {
    path: Option<&'p str>,
    method: Option<&'p str>,
}

impl<'p> RequestBuilder<'p> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            path: None,
            method: None,
        }
    }

    #[must_use]
    pub const fn path(mut self, path: &'p str) -> Self {
        self.path = Some(path);
        self
    }

    #[must_use]
    pub const fn method(mut self, method: &'p str) -> Self {
        self.method = Some(method);
        self
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn build(self) -> Result<Request<'p>, RequestError> {
        let path = self.path.ok_or(RequestError::MissingPath)?;
        let path = percent_decode(path.as_bytes())?;

        Ok(Request {
            path,
            method: self.method,
        })
    }
}
