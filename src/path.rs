use crate::{decode::percent_decode, errors::PathError};
use std::borrow::Cow;

/// [`Path`] stores the URI data to be used to search for a matching route in a [`Router`](crate::Router).
#[derive(Debug)]
pub struct Path<'path> {
    /// Original, unaltered path bytes.
    _raw: &'path [u8],

    /// Percent-decoded path bytes.
    decoded: Cow<'path, [u8]>,
}

impl<'path> Path<'path> {
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
    /// assert_eq!(path.decoded_bytes(), b"/hello world");
    /// ```
    ///
    /// ## Invalid
    ///
    /// ```rust
    /// use wayfind::{Path, errors::PathError};
    ///
    /// let path = Path::new("/hello%GGworld").unwrap_err();
    /// assert_eq!(path, PathError::InvalidEncoding {
    ///     input: "/hello%GGworld".to_string(),
    ///     position: 6,
    ///     character: [b'%', b'G', b'G'],
    /// });
    /// ```
    pub fn new(path: &'path str) -> Result<Self, PathError> {
        let decoded = percent_decode(path.as_bytes())?;

        Ok(Self {
            _raw: path.as_bytes(),
            decoded,
        })
    }

    /// Returns a reference to the percent-decoded path bytes.
    /// May contain invalid UTF-8 bytes.
    #[must_use]
    pub fn decoded_bytes(&'path self) -> &'path [u8] {
        &self.decoded
    }
}
