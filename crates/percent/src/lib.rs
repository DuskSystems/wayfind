//! <https://datatracker.ietf.org/doc/html/rfc3986#section-2.1>

#![allow(clippy::missing_errors_doc)]

use errors::PercentDecodingError;
use std::borrow::Cow;

pub mod errors;

/// Try and percent-decode input bytes.
/// Does not do any sort of normalization, simply decodes hex characters.
pub fn percent_decode(input: &[u8]) -> Result<Cow<'_, [u8]>, PercentDecodingError> {
    if !input.contains(&b'%') {
        return Ok(Cow::Borrowed(input));
    }

    let mut output = Vec::with_capacity(input.len());
    let mut i = 0;
    let len = input.len();

    while i < len {
        match input[i] {
            b'%' if i + 2 >= len => {
                return Err(PercentDecodingError::InvalidCharacter {
                    input: String::from_utf8_lossy(input).to_string(),
                    position: i,
                    character: input[i..].to_vec(),
                });
            }
            b'%' => {
                let a = input[i + 1];
                let b = input[i + 2];

                if let Some(decoded) = decode_hex(a, b) {
                    output.push(decoded);
                } else {
                    return Err(PercentDecodingError::InvalidCharacter {
                        input: String::from_utf8_lossy(input).to_string(),
                        position: i,
                        character: vec![b'%', a, b],
                    });
                }

                i += 3;
            }
            byte => {
                output.push(byte);
                i += 1;
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L867>
    #[test]
    fn test_percent_empty_input() {
        let input = b"";
        let result = percent_decode(input).unwrap();
        assert_eq!(result, Cow::Borrowed(b""));
    }

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L872>
    #[test]
    fn test_percent_no_decode_needed() {
        let input = b"abc";
        let result = percent_decode(input).unwrap();
        assert_eq!(result, Cow::Borrowed(b"abc"));
    }

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L877>
    #[test]
    fn test_percent_simple_hex_decode() {
        let input = b"1%41";
        let result = percent_decode(input).unwrap();
        assert_eq!(result, Cow::Borrowed(b"1A"));
    }

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L882>
    #[test]
    fn test_percent_multiple_hex_decode() {
        let input = b"1%41%42%43";
        let result = percent_decode(input).unwrap();
        assert_eq!(result, Cow::Borrowed(b"1ABC"));
    }

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L887>
    #[test]
    fn test_percent_hex_decode_lowercase() {
        let input = b"%4a";
        let result = percent_decode(input).unwrap();
        assert_eq!(result, Cow::Borrowed(b"J"));
    }

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L892>
    #[test]
    fn test_percent_hex_decode_uppercase() {
        let input = b"%6F";
        let result = percent_decode(input).unwrap();
        assert_eq!(result, Cow::Borrowed(b"o"));
    }

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L897>
    #[test]
    fn test_percent_incomplete_percent() {
        let input = b"%";
        let result = percent_decode(input).unwrap_err();

        insta::assert_snapshot!(result, @r"
        invalid character

           Input: %
                  ^

        Expected: '%' followed by two hexadecimal digits (a-F, 0-9)
           Found: '%'
        ");

        assert_eq!(
            result,
            PercentDecodingError::InvalidCharacter {
                input: String::from_utf8_lossy(input).to_string(),
                position: 0,
                character: vec![b'%'],
            }
        );
    }

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L902>
    #[test]
    fn test_percent_incomplete_hex_digit() {
        let input = b"%a";
        let result = percent_decode(input).unwrap_err();

        insta::assert_snapshot!(result, @r"
        invalid character

           Input: %a
                  ^^

        Expected: '%' followed by two hexadecimal digits (a-F, 0-9)
           Found: '%a'
        ");

        assert_eq!(
            result,
            PercentDecodingError::InvalidCharacter {
                input: String::from_utf8_lossy(input).to_string(),
                position: 0,
                character: vec![b'%', b'a'],
            }
        );
    }

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L907>
    #[test]
    fn test_percent_incomplete_second_hex_digit() {
        let input = b"%1";
        let result = percent_decode(input).unwrap_err();

        insta::assert_snapshot!(result, @r"
        invalid character

           Input: %1
                  ^^

        Expected: '%' followed by two hexadecimal digits (a-F, 0-9)
           Found: '%1'
        ");

        assert_eq!(
            result,
            PercentDecodingError::InvalidCharacter {
                input: String::from_utf8_lossy(input).to_string(),
                position: 0,
                character: vec![b'%', b'1'],
            }
        );
    }

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L912>
    #[test]
    fn test_percent_trailing_incomplete_percent() {
        let input = b"123%45%6";
        let result = percent_decode(input).unwrap_err();

        insta::assert_snapshot!(result, @r"
        invalid character

           Input: 123%45%6
                        ^^

        Expected: '%' followed by two hexadecimal digits (a-F, 0-9)
           Found: '%6'
        ");

        assert_eq!(
            result,
            PercentDecodingError::InvalidCharacter {
                input: String::from_utf8_lossy(input).to_string(),
                position: 6,
                character: vec![b'%', b'6'],
            }
        );
    }

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L917>
    #[test]
    fn test_percent_invalid_hex_digits() {
        let input = b"%zzzzz";
        let result = percent_decode(input).unwrap_err();

        insta::assert_snapshot!(result, @r"
        invalid character

           Input: %zzzzz
                  ^^^

        Expected: '%' followed by two hexadecimal digits (a-F, 0-9)
           Found: '%zz'
        ");

        assert_eq!(
            result,
            PercentDecodingError::InvalidCharacter {
                input: String::from_utf8_lossy(input).to_string(),
                position: 0,
                character: vec![b'%', b'z', b'z'],
            }
        );
    }

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L922>
    #[test]
    fn test_percent_space_encoding_1() {
        let input = b"a%20b";
        let result = percent_decode(input).unwrap();
        assert_eq!(result, Cow::Borrowed(b"a b"));
    }

    /// <https://github.com/golang/go/blob/release-branch.go1.24/src/net/url/url_test.go#L927>
    #[test]
    #[ignore = "we don't support '+' in paths"]
    fn test_percent_space_encoding_2() {
        let input = b"a+b";
        let result = percent_decode(input).unwrap();
        assert_eq!(result, Cow::Borrowed(b"a b"));
    }
}
