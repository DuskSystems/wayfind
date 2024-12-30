use crate::errors::{EncodingError, RequestError};
use std::{borrow::Cow, fmt::Debug};
use wayfind_percent::percent_decode;
use wayfind_punycode::punycode_decode;

#[derive(Clone, Eq, PartialEq)]
pub struct Request<'r> {
    authority: Option<Cow<'r, str>>,
    path: Cow<'r, [u8]>,
    method: Option<&'r str>,
}

impl Request<'_> {
    #[must_use]
    pub fn authority(&self) -> Option<&str> {
        self.authority.as_deref()
    }

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
            .field("authority", &self.authority)
            .field("path", &String::from_utf8_lossy(&self.path))
            .field("method", &self.method)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestBuilder<'p> {
    authority: Option<&'p str>,
    path: Option<&'p str>,
    method: Option<&'p str>,
}

impl<'p> RequestBuilder<'p> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            authority: None,
            path: None,
            method: None,
        }
    }

    #[must_use]
    pub const fn authority(mut self, authority: &'p str) -> Self {
        self.authority = Some(authority);
        self
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
        let authority = if let Some(authority) = self.authority {
            Some(punycode_decode(authority.as_bytes()).map_err(EncodingError::from)?)
        } else {
            None
        };

        let path = self.path.ok_or(RequestError::MissingPath)?;
        let path = percent_decode(path.as_bytes()).map_err(EncodingError::from)?;

        Ok(Request {
            authority,
            path,
            method: self.method,
        })
    }
}
