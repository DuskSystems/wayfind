use crate::{decode::percent_decode, errors::RequestError};
use std::borrow::Cow;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Request<'p> {
    path: Cow<'p, [u8]>,
    method: Option<&'p str>,
}

impl<'p> Request<'p> {
    #[must_use]
    pub fn path(&'p self) -> &'p [u8] {
        self.path.as_ref()
    }

    #[must_use]
    pub const fn method(&'p self) -> Option<&'p str> {
        self.method
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
