#![allow(clippy::too_many_lines)]

use std::error::Error;
use wayfind::{assert_router_matches, router::Router};

#[test]
fn uncommon() -> Result<(), Box<dyn Error>> {
    let mut router = Router::<_, ()>::new();

    // Japanese (Konnichiwa)
    router.insert("/こんにちは", 0)?;

    // Russian (Privet)
    router.insert("/привет", 1)?;

    // Chinese (Nǐ Hǎo)
    router.insert("/你好", 2)?;

    // Arabic Numerals (full-width)
    router.insert("/１２３", 3)?;

    // Null Byte
    router.insert("/null\0byte", 4)?;

    // Emoji
    router.insert("/⚽️🏀🏈", 5)?;

    // Unicode
    router.insert("/♔♕♖♗♘♙", 6)?;

    // Unicode Normalization
    router.insert("/cafe\u{0301}", 7)?;
    router.insert("/café", 8)?;

    // Unicode Zero Width
    router.insert("/abc\u{200B}123", 9)?;

    // Unicode Right to Left
    router.insert("/hello\u{202E}dlrow", 10)?;

    // Unicode Whitespace
    router.insert("/\u{2000}\u{2001}\u{2002}", 11)?;

    // Unicode Control
    router.insert("/\u{0001}\u{0002}\u{0003}", 12)?;

    // Punycode (müller.de)
    router.insert("/xn--mller-kva.de", 13)?;

    // URL Encoded (😊)
    router.insert("/%F0%9F%98%8A", 14)?;

    // Double URL Encoded (💀)
    router.insert("/%25F0%259F%2592%2580", 15)?;

    assert_router_matches!(router, {
        // Japanese (Konnichiwa)
        "/こんにちは" => {
            path: "/こんにちは",
            value: 0
        }
        "/こんにちわ" => None

        // Russian (Privet)
        "/привет" => {
            path: "/привет",
            value: 1
        }
        "/привет!" => None

        // Chinese (Nǐ Hǎo)
        "/你好" => {
            path: "/你好",
            value: 2
        }
        "/你们好" => None

        // Arabic Numerals (full-width)
        "/１２３" => {
            path: "/１２３",
            value: 3
        }
        "/123" => None

        // Null Byte
        "/null\0byte" => {
            path: "/null\0byte",
            value: 4
        }
        "/nullbyte" => None

        // Emoji
        "/⚽️🏀🏈" => {
            path: "/⚽️🏀🏈",
            value: 5
        }
        "/⚽️🏀" => None

        // Unicode
        "/♔♕♖♗♘♙" => {
            path: "/♔♕♖♗♘♙",
            value: 6
        }
        "/♔♕♖♗♘♟" => None

        // Unicode Normalization
        "/cafe\u{0301}" => {
            path: "/cafe\u{0301}",
            value: 7
        }
        "/café" => {
            path: "/café",
            value: 8
        }
        "/cafe" => None

        // Unicode Zero Width
        "/abc\u{200B}123" => {
            path: "/abc\u{200B}123",
            value: 9
        }
        "/abc123" => None

        // Unicode Right to Left
        "/hello\u{202E}dlrow" => {
            path: "/hello\u{202E}dlrow",
            value: 10
        }
        "/helloworld" => None

        // Unicode Whitespace
        "/\u{2000}\u{2001}\u{2002}" => {
            path: "/\u{2000}\u{2001}\u{2002}",
            value: 11
        }
        "/   " => None

        // Unicode Control
        "/\u{0001}\u{0002}\u{0003}" => {
            path: "/\u{0001}\u{0002}\u{0003}",
            value: 12
        }
        "/123" => None

        // Punycode (müller.de)
        "/xn--mller-kva.de" => {
            path: "/xn--mller-kva.de",
            value: 13
        }
        "/muller.de" => None

        // URL Encoded (😊)
        "/%F0%9F%98%8A" => {
            path: "/%F0%9F%98%8A",
            value: 14
        }
        "/😊" => None

        // Double URL Encoded (💀)
        "/%25F0%259F%2592%2580" => {
            path: "/%25F0%259F%2592%2580",
            value: 15
        }
        "/%F0%9F%92%80" => None
        "/💀" => None
    });

    Ok(())
}
