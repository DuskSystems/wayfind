use crate::{decode::percent_decode, errors::PathError};
use alloc::borrow::Cow;

/// [`Path`] stores the URI data to be used to search for a matching route in a [`Router`](crate::Router).
#[derive(Debug, Eq, PartialEq)]
pub struct Path<'p> {
    /// Percent-decoded path bytes.
    decoded: Cow<'p, [u8]>,
}

impl<'p> Path<'p> {
    /// Creates a new [`Path`] instance from a URI path string.
    ///
    /// # Errors
    ///
    /// Will error if the path can't be created, due to invalid percent-encoding.
    ///
    /// # Examples
    ///
    /// ## Valid
    ///
    /// ```rust
    /// use wayfind::Path;
    ///
    /// let path = Path::new("/hello%20world").unwrap();
    /// assert_eq!(path.as_bytes(), b"/hello world");
    /// ```
    ///
    /// ## Invalid
    ///
    /// ```rust
    /// use wayfind::{Path, errors::{EncodingError, PathError}};
    ///
    /// let path = Path::new("/hello%GGworld").unwrap_err();
    /// assert_eq!(path, PathError::EncodingError(EncodingError::InvalidEncoding {
    ///     input: "/hello%GGworld".to_string(),
    ///     position: 6,
    ///     character: [b'%', b'G', b'G'],
    /// }));
    /// ```
    pub fn new(path: &'p str) -> Result<Self, PathError> {
        Ok(Self {
            decoded: percent_decode(path.as_bytes())?,
        })
    }

    /// Returns a reference to the percent-decoded path bytes.
    /// May contain invalid UTF-8 bytes.
    #[must_use]
    pub fn as_bytes(&'p self) -> &'p [u8] {
        &self.decoded
    }
}
