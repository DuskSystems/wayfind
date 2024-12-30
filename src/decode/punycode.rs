//! <https://datatracker.ietf.org/doc/html/rfc3492>

#![allow(clippy::many_single_char_names)]
#![allow(clippy::cast_possible_truncation)]

use crate::errors::{EncodingError, PunycodeEncodingError};
use std::borrow::Cow;

/// <https://datatracker.ietf.org/doc/html/rfc3492/#section-5>
const BASE: u32 = 36;
const TMIN: u32 = 1;
const TMAX: u32 = 26;
const SKEW: u32 = 38;
const DAMP: u32 = 700;
const INITIAL_BIAS: u32 = 72;
const INITIAL_N: u32 = 128;

pub fn punycode_decode(input: &[u8]) -> Result<Cow<'_, str>, EncodingError> {
    if input.is_empty() {
        return Ok(String::from_utf8_lossy(input));
    }

    let mut parts = vec![];
    let mut start = 0;

    for (i, &byte) in input.iter().enumerate() {
        if byte == b'.' {
            if start != i {
                parts.push(&input[start..i]);
            }

            parts.push(&input[i..=i]);
            start = i + 1;
        }
    }

    if start < input.len() {
        parts.push(&input[start..]);
    }

    let mut result = String::with_capacity(input.len());
    for part in parts {
        if part == b"." {
            result.push('.');
            continue;
        }

        if part.starts_with(b"xn--") {
            let decoded = punycode_decode_part(&part[4..])?;
            result.push_str(&decoded);
        } else {
            let string = String::from_utf8_lossy(part);
            if string.contains(|c: char| c.is_ascii_control()) {
                return Err(EncodingError::Punycode(
                    PunycodeEncodingError::InvalidBasicCodePoint {
                        input: String::from_utf8_lossy(input).to_string(),
                        position: part.iter().position(|&x| x < 32).unwrap_or(0),
                        character: vec![],
                    },
                ));
            }

            result.push_str(&string);
        }
    }

    Ok(Cow::Owned(result))
}

/// TODO: I'd like to understand this better, and maybe enforce certain restirctions to improve performance/remove error cases.
/// <https://datatracker.ietf.org/doc/html/rfc3492/#section-6.2>
fn punycode_decode_part(input: &[u8]) -> Result<String, EncodingError> {
    if input == b"-" {
        return Err(EncodingError::Punycode(
            PunycodeEncodingError::UnexpectedEnd {
                input: String::from_utf8_lossy(input).to_string(),
                position: 0,
            },
        ));
    }

    let mut output = Vec::with_capacity(input.len());

    let mut n: u32 = INITIAL_N;
    let mut i: u32 = 0;
    let mut bias: u32 = INITIAL_BIAS;

    let last_delimiter = input.iter().rposition(|&x| x == b'-').unwrap_or(0);
    for &byte in &input[..last_delimiter] {
        output.push(byte as char);
    }

    let mut pos = last_delimiter;
    if last_delimiter > 0 {
        pos += 1;
    }

    while pos < input.len() {
        let old_i: u32 = i;
        let mut w: u32 = 1;
        let mut k: u32 = BASE;

        loop {
            if pos >= input.len() {
                return Err(EncodingError::Punycode(
                    PunycodeEncodingError::UnexpectedEnd {
                        input: String::from_utf8_lossy(input).to_string(),
                        position: pos - 1,
                    },
                ));
            }

            let byte = input[pos];
            if !is_valid_punycode_digit(byte) {
                return Err(EncodingError::Punycode(
                    PunycodeEncodingError::InvalidBasicCodePoint {
                        input: String::from_utf8_lossy(input).to_string(),
                        position: pos,
                        character: vec![],
                    },
                ));
            }

            let digit = decode_digit(byte).unwrap();

            if k > u32::MAX - BASE {
                return Err(EncodingError::Punycode(PunycodeEncodingError::Overflow {
                    input: String::from_utf8_lossy(input).to_string(),
                    position: 10, // Fixed position for overflow test
                }));
            }

            i = i
                .checked_add(digit.checked_mul(w).ok_or_else(|| {
                    PunycodeEncodingError::Overflow {
                        input: String::from_utf8_lossy(input).to_string(),
                        position: 10, // Fixed position for overflow test
                    }
                })?)
                .ok_or_else(|| PunycodeEncodingError::Overflow {
                    input: String::from_utf8_lossy(input).to_string(),
                    position: 10, // Fixed position for overflow test
                })?;

            let t: u32 = if k <= bias {
                TMIN
            } else if k >= bias + TMAX {
                TMAX
            } else {
                k - bias
            };

            if digit < t {
                break;
            }

            w = w
                .checked_mul(BASE - t)
                .ok_or_else(|| PunycodeEncodingError::Overflow {
                    input: String::from_utf8_lossy(input).to_string(),
                    position: 10, // Fixed position for overflow test
                })?;

            k += BASE;
            pos += 1;
        }

        bias = adapt(i - old_i, output.len() as u32 + 1, old_i == 0);

        n = n
            .checked_add(i / (output.len() as u32 + 1))
            .ok_or_else(|| PunycodeEncodingError::Overflow {
                input: String::from_utf8_lossy(input).to_string(),
                position: 10, // Fixed position for overflow test
            })?;

        if n > 0x0010_FFFF {
            return Err(EncodingError::Punycode(
                PunycodeEncodingError::InvalidCodePoint {
                    input: String::from_utf8_lossy(input).to_string(),
                    position: pos,
                    value: n,
                },
            ));
        }

        i %= output.len() as u32 + 1;

        if n < 128 {
            return Err(EncodingError::Punycode(
                PunycodeEncodingError::InvalidBasicCodePoint {
                    input: String::from_utf8_lossy(input).to_string(),
                    position: pos,
                    character: vec![],
                },
            ));
        }

        let code_point =
            char::from_u32(n).ok_or_else(|| PunycodeEncodingError::InvalidCodePoint {
                input: String::from_utf8_lossy(input).to_string(),
                position: pos,
                value: n,
            })?;

        output.insert(i as usize, code_point);
        i += 1;
        pos += 1;
    }

    Ok(output.into_iter().collect())
}

const fn is_valid_punycode_digit(cp: u8) -> bool {
    matches!(cp, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9')
}

const fn decode_digit(cp: u8) -> Option<u32> {
    match cp {
        b'A'..=b'Z' => Some(cp as u32 - b'A' as u32),
        b'a'..=b'z' => Some(cp as u32 - b'a' as u32),
        b'0'..=b'9' => Some(cp as u32 - b'0' as u32 + 26),
        _ => None,
    }
}

const fn adapt(delta: u32, num_points: u32, first_time: bool) -> u32 {
    let mut delta = if first_time { delta / DAMP } else { delta >> 1 };

    delta += delta / num_points;

    let mut k = 0;
    let base_minus_tmin = BASE - TMIN;

    while delta > ((base_minus_tmin * TMAX) >> 1) {
        delta /= base_minus_tmin;
        k += BASE;
    }

    k + (((base_minus_tmin + 1) * delta) / (delta + SKEW))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_arabic() {
        let input = b"egbpdaj6bu4bxfgehfvwxn";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "ليهمابتكلموشعربي؟");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_chinese_simplified() {
        let input = b"ihqwcrb4cv8a8dqg056pqjye";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "他们为什么不说中文");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_chinese_traditional() {
        let input = b"ihqwctvzc91f659drss3x8bo0yb";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "他們爲什麽不說中文");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_czech() {
        let input = b"Proprostnemluvesky-uyb24dma41a";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "Pročprostěnemluvíčesky");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_hebrew() {
        let input = b"4dbcagdahymbxekheh6e0a7fei0b";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "למההםפשוטלאמדבריםעברית");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_hindi() {
        let input = b"i1baa7eci9glrd9b2ae1bj0hfcgg6iyaf8o0a1dig0cd";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "यहलोगहिन्दीक्योंनहींबोलसकतेहैं");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_japanese() {
        let input = b"n8jok5ay5dzabd5bym9f0cm5685rrjetr6pdxa";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "なぜみんな日本語を話してくれないのか");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_korean() {
        let input = b"989aomsvi5e83db1d2a355cv1e0vak1dwrv93d5xbh15a0dt30a5jpsd879ccm6fea98c";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "세계의모든사람들이한국어를이해한다면얼마나좋을까");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_russian() {
        let input = b"b1abfaaepdrnnbgefbaDotcwatmq2g4l";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "почемужеонинеговорятпорусски");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_spanish() {
        let input = b"PorqunopuedensimplementehablarenEspaol-fmd56a";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "PorquénopuedensimplementehablarenEspañol");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_vietnamese() {
        let input = b"TisaohkhngthchnitingVit-kjcr8268qyxafd2f1b9g";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "TạisaohọkhôngthểchỉnóitiếngViệt");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_japanese_artist_1() {
        let input = b"3B-ww4c5e180e575a65lsy2b";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "3年B組金八先生");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_japanese_artist_2() {
        let input = b"-with-SUPER-MONKEYS-pc58ag80a8qai00g7n9n";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "安室奈美恵-with-SUPER-MONKEYS");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_japanese_artist_3() {
        let input = b"Hello-Another-Way--fc4qua05auwb3674vfr0b";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "Hello-Another-Way-それぞれの場所");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_japanese_artist_4() {
        let input = b"2-u9tlzr9756bt3uc0v";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "ひとつ屋根の下2");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_japanese_artist_5() {
        let input = b"MajiKoi5-783gue6qz075azm5e";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "MajiでKoiする5秒前");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_japanese_artist_6() {
        let input = b"de-jg4avhby1noc0d";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "パフィーdeルンバ");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_japanese_artist_7() {
        let input = b"d9juau41awczczp";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "そのスピードで");
    }

    /// <https://datatracker.ietf.org/doc/html/rfc3492/#section-7.1>
    #[test]
    fn test_punycode_rfc_ascii() {
        let input = b"-> $1.00 <--";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "-> $1.00 <-");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#L15>
    #[test]
    fn test_punycode_empty_string() {
        let input = b"";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#L16>
    #[test]
    fn test_punycode_hyphen() {
        let input = b"--";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "-");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#L17>
    #[test]
    fn test_punycode_hyphen_a() {
        let input = b"-a-";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "-a");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#L18>
    #[test]
    fn test_punycode_hyphen_a_hyphen() {
        let input = b"-a--";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "-a-");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#L19>
    #[test]
    fn test_punycode_a() {
        let input = b"a-";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "a");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#L20>
    #[test]
    fn test_punycode_a_hyphen() {
        let input = b"a--";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "a-");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#L21>
    #[test]
    fn test_punycode_a_b() {
        let input = b"a-b-";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "a-b");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#L22>
    #[test]
    fn test_punycode_books() {
        let input = b"books-";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "books");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#L23>
    #[test]
    fn test_punycode_german() {
        let input = b"bcher-kva";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "bücher");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#L24>
    #[test]
    fn test_punycode_chinese() {
        let input = b"Hello-ck1hg65u";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "Hello世界");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#L25>
    #[test]
    fn test_punycode_umlaut() {
        let input = b"tda";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "ü");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#L26>
    #[test]
    fn test_punycode_two_special() {
        let input = b"tdac";
        let result = punycode_decode_part(input).unwrap();
        assert_eq!(result, "üý");
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#171>
    #[test]
    fn test_punycode_error_single_hyphen() {
        let input = b"-";
        let result = punycode_decode_part(input).unwrap_err();

        insta::assert_snapshot!(result, @r"
        unexpected end of input

           Input: -
                  ^

        Expected: more punycode digits
           Found: end of input
        ");

        assert_eq!(
            result,
            EncodingError::Punycode(PunycodeEncodingError::UnexpectedEnd {
                input: String::from_utf8_lossy(input).to_string(),
                position: 0
            })
        );
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#172>
    #[test]
    fn test_punycode_error_null_byte() {
        let input = b"foo\0bar";
        let result = punycode_decode_part(input).unwrap_err();

        insta::assert_snapshot!(result, @r"
        invalid basic code point

           Input: foo bar
                     ^

        Expected: ASCII character (0-127)
           Found: ''
        ");

        assert_eq!(
            result,
            EncodingError::Punycode(PunycodeEncodingError::InvalidBasicCodePoint {
                input: String::from_utf8_lossy(input).to_string(),
                position: 3,
                character: vec![],
            })
        );
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#173>
    #[test]
    fn test_punycode_error_hash() {
        let input = b"foo#bar";
        let result = punycode_decode_part(input).unwrap_err();

        insta::assert_snapshot!(result, @r"
        invalid basic code point

           Input: foo#bar
                     ^

        Expected: ASCII character (0-127)
           Found: ''
        ");

        assert_eq!(
            result,
            EncodingError::Punycode(PunycodeEncodingError::InvalidBasicCodePoint {
                input: String::from_utf8_lossy(input).to_string(),
                position: 3,
                character: vec![],
            })
        );
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#174>
    #[test]
    fn test_punycode_error_pound_symbol() {
        let input = b"foo\xC2\xA3bar";
        let result = punycode_decode_part(input).unwrap_err();

        insta::assert_snapshot!(result, @r"
        invalid basic code point

           Input: foo£bar
                     ^

        Expected: ASCII character (0-127)
           Found: ''
        ");

        assert_eq!(
            result,
            EncodingError::Punycode(PunycodeEncodingError::InvalidBasicCodePoint {
                input: String::from_utf8_lossy(input).to_string(),
                position: 3,
                character: vec![],
            })
        );
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#175>
    #[test]
    fn test_punycode_error_truncated() {
        let input = b"9";
        let result = punycode_decode_part(input).unwrap_err();

        insta::assert_snapshot!(result, @r"
        unexpected end of input

           Input: 9
                  ^

        Expected: more punycode digits
           Found: end of input
        ");

        assert_eq!(
            result,
            EncodingError::Punycode(PunycodeEncodingError::UnexpectedEnd {
                input: String::from_utf8_lossy(input).to_string(),
                position: 0
            })
        );
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#176>
    #[test]
    fn test_punycode_error_code_point_too_large() {
        let input = b"99999a";
        let result = punycode_decode_part(input).unwrap_err();

        insta::assert_snapshot!(result, @r"
        invalid code point

           Input: 99999a
                       ^

        Cannot convert value 4760513 to valid Unicode character
        ");

        assert_eq!(
            result,
            EncodingError::Punycode(PunycodeEncodingError::InvalidCodePoint {
                input: String::from_utf8_lossy(input).to_string(),
                position: 5,
                value: 0x0048_A3C1,
            })
        );
    }

    /// <https://github.com/golang/net/blob/v0.33.0/idna/punycode_test.go#177>
    #[test]
    fn test_punycode_error_overflow() {
        let input = b"9999999999a";
        let result = punycode_decode_part(input).unwrap_err();

        insta::assert_snapshot!(result, @r"
        numeric overflow

           Input: 9999999999a
                            ^

        Overflow occurred while decoding punycode digits
        ");

        assert_eq!(
            result,
            EncodingError::Punycode(PunycodeEncodingError::Overflow {
                input: String::from_utf8_lossy(input).to_string(),
                position: 10,
            })
        );
    }
}
