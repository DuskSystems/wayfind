use crate::errors::EncodingError;
use alloc::{
    borrow::Cow,
    string::{String, ToString},
    vec::Vec,
};

/// Try and percent-decode input bytes.
/// Does not do any sort of normalization, simply decodes hex characters.
pub fn percent_decode(input: &[u8]) -> Result<Cow<'_, [u8]>, EncodingError> {
    if !input.contains(&b'%') {
        return Ok(Cow::Borrowed(input));
    }

    let mut output = Vec::with_capacity(input.len());
    let mut i = 0;
    let len = input.len();

    while i < len {
        if input[i] == b'%' && i + 2 < len {
            let a = input[i + 1];
            let b = input[i + 2];

            if let Some(decoded) = decode_hex(a, b) {
                output.push(decoded);
            } else {
                return Err(EncodingError::InvalidEncoding {
                    input: String::from_utf8_lossy(input).to_string(),
                    position: i,
                    character: [b'%', a, b],
                });
            }

            i += 3;
        } else {
            output.push(input[i]);
            i += 1;
        }
    }

    Ok(Cow::Owned(output))
}

#[inline]
const fn decode_hex(a: u8, b: u8) -> Option<u8> {
    let high = match a {
        b'0'..=b'9' => a - b'0',
        b'A'..=b'F' => a - b'A' + 10,
        b'a'..=b'f' => a - b'a' + 10,
        _ => return None,
    };

    let low = match b {
        b'0'..=b'9' => b - b'0',
        b'A'..=b'F' => b - b'A' + 10,
        b'a'..=b'f' => b - b'a' + 10,
        _ => return None,
    };

    Some((high << 4) | low)
}
