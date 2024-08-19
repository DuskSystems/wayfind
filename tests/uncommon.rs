#![allow(clippy::too_many_lines)]

use std::error::Error;
use wayfind::router::Router;

#[path = "./common.rs"]
mod common;

#[test]
fn uncommon() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

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

    assert_router_matches!(router, {
        // Japanese (Konnichiwa)
        "/こんにちは" => {
            path: "/こんにちは",
            value: 0
        }
        "/%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF" => {
            path: "/こんにちは",
            value: 0
        }
        "/こんにちわ" => None

        // Russian (Privet)
        "/привет" => {
            path: "/привет",
            value: 1
        }
        "/%D0%BF%D1%80%D0%B8%D0%B2%D0%B5%D1%82" => {
            path: "/привет",
            value: 1
        }
        "/привет!" => None

        // Chinese (Nǐ Hǎo)
        "/你好" => {
            path: "/你好",
            value: 2
        }
        "/%E4%BD%A0%E5%A5%BD" => {
            path: "/你好",
            value: 2
        }
        "/你们好" => None

        // Arabic Numerals (full-width)
        "/１２３" => {
            path: "/１２３",
            value: 3
        }
        "/%EF%BC%91%EF%BC%92%EF%BC%93" => {
            path: "/１２３",
            value: 3
        }
        "/123" => None

        // Null Byte
        "/null\0byte" => {
            path: "/null\0byte",
            value: 4
        }
        "/null%00byte" => {
            path: "/null\0byte",
            value: 4
        }
        "/nullbyte" => None

        // Emoji
        "/⚽️🏀🏈" => {
            path: "/⚽️🏀🏈",
            value: 5
        }
        "/%E2%9A%BD%EF%B8%8F%F0%9F%8F%80%F0%9F%8F%88" => {
            path: "/⚽️🏀🏈",
            value: 5
        }
        "/⚽️🏀" => None

        // Unicode
        "/♔♕♖♗♘♙" => {
            path: "/♔♕♖♗♘♙",
            value: 6
        }
        "/%E2%99%94%E2%99%95%E2%99%96%E2%99%97%E2%99%98%E2%99%99" => {
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
        "/cafe%CC%81" => {
            path: "/cafe\u{0301}",
            value: 7
        }
        "/caf%C3%A9" => {
            path: "/café",
            value: 8
        }
        "/cafe" => None

        // Unicode Zero Width
        "/abc\u{200B}123" => {
            path: "/abc\u{200B}123",
            value: 9
        }
        "/abc%E2%80%8B123" => {
            path: "/abc\u{200B}123",
            value: 9
        }
        "/abc123" => None

        // Unicode Right to Left
        "/hello\u{202E}dlrow" => {
            path: "/hello\u{202E}dlrow",
            value: 10
        }
        "/hello%E2%80%AEdlrow" => {
            path: "/hello\u{202E}dlrow",
            value: 10
        }
        "/helloworld" => None

        // Unicode Whitespace
        "/\u{2000}\u{2001}\u{2002}" => {
            path: "/\u{2000}\u{2001}\u{2002}",
            value: 11
        }
        "/%E2%80%80%E2%80%81%E2%80%82" => {
            path: "/\u{2000}\u{2001}\u{2002}",
            value: 11
        }
        "/   " => None

        // Unicode Control
        "/\u{0001}\u{0002}\u{0003}" => {
            path: "/\u{0001}\u{0002}\u{0003}",
            value: 12
        }
        "/%01%02%03" => {
            path: "/\u{0001}\u{0002}\u{0003}",
            value: 12
        }
        "/123" => None
    });

    Ok(())
}
