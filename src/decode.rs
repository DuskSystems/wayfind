use crate::errors::decode::DecodeError;
use std::borrow::Cow;

pub(crate) fn percent_decode(input: &[u8]) -> Result<Cow<[u8]>, DecodeError> {
    if !input.contains(&b'%') {
        return Ok(Cow::Borrowed(input));
    }

    let mut output = Vec::with_capacity(input.len());
    let mut position = 0;
    let len = input.len();

    while position < len {
        if input[position] == b'%' && position + 2 < len {
            let a = input[position + 1];
            let b = input[position + 2];

            if let Some(decoded) = decode_hex(a, b) {
                output.push(decoded);
            } else {
                return Err(DecodeError::InvalidEncoding {
                    input: String::from_utf8_lossy(input).to_string(),
                    position,
                    character: [b'%', a, b],
                });
            }

            position += 3;
        } else {
            output.push(input[position]);
            position += 1;
        }
    }

    Ok(Cow::Owned(output))
}

#[inline]
#[allow(clippy::cast_possible_truncation)]
fn decode_hex(a: u8, b: u8) -> Option<u8> {
    let a = (a as char).to_digit(16)?;
    let b = (b as char).to_digit(16)?;
    Some((a as u8) << 4 | (b as u8))
}
