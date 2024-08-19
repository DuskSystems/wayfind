use crate::{decode::percent_decode, errors::decode::DecodeError};

pub struct Path {
    pub inner: Vec<u8>,
}

impl Path {
    #[must_use]
    pub fn new(path: &str) -> Self {
        Self {
            inner: path.as_bytes().to_vec(),
        }
    }

    pub fn percent_decode(&mut self) -> Result<(), DecodeError> {
        self.inner = percent_decode(&self.inner)?;
        Ok(())
    }
}
