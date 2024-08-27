use crate::{decode::percent_decode, errors::decode::DecodeError};
use std::borrow::Cow;

/// [`Path`] stores the URI data to be used to search for a matching route in a [`Router`](crate::Router).
#[derive(Debug)]
pub struct Path<'path> {
    /// Original, unaltered path bytes.
    raw: &'path [u8],

    /// Percent-decoded path bytes.
    decoded: Cow<'path, [u8]>,
}

impl<'path> Path<'path> {
    /// Creates a new [`Path`] instance from a URI path string.
    ///
    /// # Errors
    ///
    /// Will error if the path fails percent-decoding.
    ///
    /// # Examples
    ///
    /// ## Valid
    ///
    /// ```rust
    /// use wayfind::Path;
    ///
    /// let path = Path::new("/hello%20world").unwrap();
    /// assert_eq!(path.raw_bytes(), b"/hello%20world");
    /// assert_eq!(path.decoded_bytes(), b"/hello world");
    /// ```
    ///
    /// ## Invalid
    ///
    /// ```rust
    /// use wayfind::{Path, errors::DecodeError};
    ///
    /// let path = Path::new("/hello%GGworld").unwrap_err();
    /// assert_eq!(path, DecodeError::InvalidEncoding {
    ///     input: "/hello%GGworld".to_string(),
    ///     position: 6,
    ///     character: [b'%', b'G', b'G'],
    /// });
    /// ```
    pub fn new(path: &'path str) -> Result<Self, DecodeError> {
        Ok(Self {
            raw: path.as_bytes(),
            decoded: percent_decode(path.as_bytes())?,
        })
    }

    /// Returns a reference to the original path bytes.
    #[must_use]
    pub const fn raw_bytes(&'path self) -> &'path [u8] {
        self.raw
    }

    /// Returns a reference to the percent-decoded path bytes.
    #[must_use]
    pub fn decoded_bytes(&'path self) -> &'path [u8] {
        &self.decoded
    }
}
