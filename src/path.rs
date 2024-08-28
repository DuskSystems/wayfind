use crate::{decode::percent_decode, errors::PathError};
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
    /// Will error if the path can't be created, due to invalid percent-encoding, or invalid UTF-8 post-decoding.
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
    /// ## Invalid Encoding
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
    ///
    /// ## Invalid UTF-8
    ///
    /// ```rust
    /// use wayfind::{Path, errors::PathError};
    ///
    /// let path = Path::new("/hello%FFworld").unwrap_err();
    /// assert_eq!(path, PathError::Utf8Error {
    ///     input: "/hello%FFworld".to_string(),
    ///     decoded: "/hello�world".to_string(),
    ///     position: 6,
    ///     length: 1,
    /// });
    /// ```
    pub fn new(path: &'path str) -> Result<Self, PathError> {
        let decoded = percent_decode(path.as_bytes())?;
        std::str::from_utf8(&decoded).map_err(|err| PathError::Utf8Error {
            input: path.to_string(),
            decoded: String::from_utf8_lossy(&decoded).to_string(),
            position: err.valid_up_to(),
            length: err.error_len().unwrap_or(1),
        })?;

        Ok(Self {
            raw: path.as_bytes(),
            decoded,
        })
    }

    /// Returns a reference to the original path bytes.
    #[must_use]
    pub const fn raw_bytes(&'path self) -> &'path [u8] {
        self.raw
    }

    /// Returns a reference to the percent-decoded path bytes.
    /// Guaranteed to be valid UTF-8, via a check in [`Path::new`].
    #[must_use]
    pub fn decoded_bytes(&'path self) -> &'path [u8] {
        &self.decoded
    }
}
