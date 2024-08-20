use crate::{decode::percent_decode, errors::decode::DecodeError};
use std::borrow::Cow;

pub struct Path<'path> {
    decoded: Cow<'path, [u8]>,
}

impl<'path> Path<'path> {
    pub fn new(path: &'path str) -> Result<Self, DecodeError> {
        Ok(Self {
            decoded: percent_decode(path.as_bytes())?,
        })
    }

    #[must_use]
    pub fn decoded_bytes(&'path self) -> &'path [u8] {
        &self.decoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_invalid_encoding() {
        let error = Path::new("/hello%20world%GG").err().unwrap();
        assert_eq!(
            error,
            DecodeError::InvalidEncoding {
                input: "/hello%20world%GG".to_string(),
                position: 14,
                character: [b'%', b'G', b'G']
            }
        );

        insta::assert_snapshot!(error, @r###"
        invalid percent-encoding

           Input: /hello%20world%GG
                                ^^^

        Expected: '%' followed by two hexadecimal digits (a-F, 0-9)
           Found: '%GG'
        "###);
    }
}
