use crate::errors::decode::DecodeError;
use std::borrow::Cow;

pub(crate) fn percent_decode(input: &[u8]) -> Result<Cow<[u8]>, DecodeError> {
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
            output.push(decode_hex(a, b)?);
            i += 3;
        } else {
            output.push(input[i]);
            i += 1;
        }
    }

    Ok(Cow::Owned(output))
}

#[allow(clippy::cast_possible_truncation)]
fn decode_hex(a: u8, b: u8) -> Result<u8, DecodeError> {
    let a = (a as char)
        .to_digit(16)
        .ok_or(DecodeError::InvalidEncoding)?;

    let b = (b as char)
        .to_digit(16)
        .ok_or(DecodeError::InvalidEncoding)?;

    Ok((a as u8) << 4 | (b as u8))
}
