use crate::{decode::percent_decode, errors::decode::DecodeError};
use std::borrow::Cow;

pub struct Path<'path> {
    _raw: &'path [u8],
    decoded: Cow<'path, [u8]>,
}

impl<'path> Path<'path> {
    pub fn new(path: &'path str) -> Result<Self, DecodeError> {
        Ok(Self {
            _raw: path.as_bytes(),
            decoded: percent_decode(path.as_bytes())?,
        })
    }

    #[must_use]
    pub fn decoded_bytes(&'path self) -> &'path [u8] {
        &self.decoded
    }
}
