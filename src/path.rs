use crate::errors::decode::DecodeError;
use std::borrow::Cow;

pub struct Path<'path> {
    decoded: Cow<'path, str>,
}

impl<'path> Path<'path> {
    pub fn new(path: &'path str) -> Result<Self, DecodeError> {
        Ok(Self {
            decoded: percent_encoding::percent_decode_str(path).decode_utf8()?,
        })
    }

    #[must_use]
    pub fn decoded_bytes(&'path self) -> &'path [u8] {
        self.decoded.as_bytes()
    }
}
