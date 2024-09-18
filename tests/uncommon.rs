use std::error::Error;
use wayfind::Router;

#[path = "./utils.rs"]
mod utils;

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
            route: "/こんにちは",
            data: 0
        }
        "/%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF" => {
            route: "/こんにちは",
            data: 0
        }
        "/こんにちわ" => None

        // Russian (Privet)
        "/привет" => {
            route: "/привет",
            data: 1
        }
        "/%D0%BF%D1%80%D0%B8%D0%B2%D0%B5%D1%82" => {
            route: "/привет",
            data: 1
        }
        "/привет!" => None

        // Chinese (Nǐ Hǎo)
        "/你好" => {
            route: "/你好",
            data: 2
        }
        "/%E4%BD%A0%E5%A5%BD" => {
            route: "/你好",
            data: 2
        }
        "/你们好" => None

        // Arabic Numerals (full-width)
        "/１２３" => {
            route: "/１２３",
            data: 3
        }
        "/%EF%BC%91%EF%BC%92%EF%BC%93" => {
            route: "/１２３",
            data: 3
        }
        "/123" => None

        // Null Byte
        "/null\0byte" => {
            route: "/null\0byte",
            data: 4
        }
        "/null%00byte" => {
            route: "/null\0byte",
            data: 4
        }
        "/nullbyte" => None

        // Emoji
        "/⚽️🏀🏈" => {
            route: "/⚽️🏀🏈",
            data: 5
        }
        "/%E2%9A%BD%EF%B8%8F%F0%9F%8F%80%F0%9F%8F%88" => {
            route: "/⚽️🏀🏈",
            data: 5
        }
        "/⚽️🏀" => None

        // Unicode
        "/♔♕♖♗♘♙" => {
            route: "/♔♕♖♗♘♙",
            data: 6
        }
        "/%E2%99%94%E2%99%95%E2%99%96%E2%99%97%E2%99%98%E2%99%99" => {
            route: "/♔♕♖♗♘♙",
            data: 6
        }
        "/♔♕♖♗♘♟" => None

        // Unicode Normalization
        "/cafe\u{0301}" => {
            route: "/cafe\u{0301}",
            data: 7
        }
        "/café" => {
            route: "/café",
            data: 8
        }
        "/cafe%CC%81" => {
            route: "/cafe\u{0301}",
            data: 7
        }
        "/caf%C3%A9" => {
            route: "/café",
            data: 8
        }
        "/cafe" => None

        // Unicode Zero Width
        "/abc\u{200B}123" => {
            route: "/abc\u{200B}123",
            data: 9
        }
        "/abc%E2%80%8B123" => {
            route: "/abc\u{200B}123",
            data: 9
        }
        "/abc123" => None

        // Unicode Right to Left
        "/hello\u{202E}dlrow" => {
            route: "/hello\u{202E}dlrow",
            data: 10
        }
        "/hello%E2%80%AEdlrow" => {
            route: "/hello\u{202E}dlrow",
            data: 10
        }
        "/helloworld" => None

        // Unicode Whitespace
        "/\u{2000}\u{2001}\u{2002}" => {
            route: "/\u{2000}\u{2001}\u{2002}",
            data: 11
        }
        "/%E2%80%80%E2%80%81%E2%80%82" => {
            route: "/\u{2000}\u{2001}\u{2002}",
            data: 11
        }
        "/   " => None

        // Unicode Control
        "/\u{0001}\u{0002}\u{0003}" => {
            route: "/\u{0001}\u{0002}\u{0003}",
            data: 12
        }
        "/%01%02%03" => {
            route: "/\u{0001}\u{0002}\u{0003}",
            data: 12
        }
        "/123" => None
    });

    Ok(())
}
