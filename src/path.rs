use crate::{decode::percent_decode, errors::decode::DecodeError};
use std::borrow::Cow;

pub struct Path<'a> {
    original: Cow<'a, [u8]>,
    decoded: Option<Vec<u8>>,
}

impl<'a> Path<'a> {
    #[must_use]
    pub const fn new(path: &'a str) -> Self {
        Self {
            original: Cow::Borrowed(path.as_bytes()),
            decoded: None,
        }
    }

    pub fn percent_decode(&mut self) -> Result<(), DecodeError> {
        if self.decoded.is_none() {
            self.decoded = Some(percent_decode(&self.original)?);
        }

        Ok(())
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.decoded.as_deref().unwrap_or(&self.original)
    }
}
