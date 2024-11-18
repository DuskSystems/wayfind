use crate::{decode::percent_decode, errors::RequestError};
use alloc::borrow::Cow;

/// [`Request`] stores the request data to be used to search for a matching route in a [`Router`](crate::Router).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Request<'p> {
    /// Percent-decoded path bytes.
    /// May contain invalid UTF-8 bytes.
    pub(crate) path: Cow<'p, [u8]>,
}

/// Builder pattern for creating a [`Request`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestBuilder<'p> {
    path: Option<&'p str>,
}

impl<'p> RequestBuilder<'p> {
    #[must_use]
    pub const fn new() -> Self {
        Self { path: None }
    }

    #[must_use]
    pub const fn path(mut self, path: &'p str) -> Self {
        self.path = Some(path);
        self
    }

    /// Builds a new [`Request`] instance from the builder.
    ///
    /// # Errors
    ///
    /// Return a [`RequestError`] if a required field was not populated.
    pub fn build(self) -> Result<Request<'p>, RequestError> {
        let path = self.path.ok_or(RequestError::MissingPath)?;
        let path = percent_decode(path.as_bytes())?;

        Ok(Request { path })
    }
}
