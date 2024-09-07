use std::error::Error;
use wayfind::Router;

#[path = "./common.rs"]
mod common;

#[test]
fn percent_encoding() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/hello@world", 1)?;
    router.insert("/hello-world.com", 2)?;
    router.insert("/hello world", 3)?;
    router.insert("/こんにちは", 4)?;
    router.insert("/50%", 5)?;
    router.insert("/hello world@example.com", 6)?;
    router.insert("/path/to/resource with spaces", 7)?;
    router.insert("/encoded/slash", 8)?;

    assert_router_matches!(router, {
        "/hello%40world" => {
            route: "/hello@world",
            data: 1
        }
        "/hello@world" => {
            route: "/hello@world",
            data: 1
        }
        "/hello-world.com" => {
            route: "/hello-world.com",
            data: 2
        }
        "/hello%20world" => {
            route: "/hello world",
            data: 3
        }
        "/hello world" => {
            route: "/hello world",
            data: 3
        }
        "/%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF" => {
            route: "/こんにちは",
            data: 4
        }
        "/こんにちは" => {
            route: "/こんにちは",
            data: 4
        }
        "/50%25" => {
            route: "/50%",
            data: 5
        }
        "/50%" => {
            route: "/50%",
            data: 5
        }
        "/hello%20world%40example.com" => {
            route: "/hello world@example.com",
            data: 6
        }
        "/hello world@example.com" => {
            route: "/hello world@example.com",
            data: 6
        }
        "/path/to/resource%20with%20spaces" => {
            route: "/path/to/resource with spaces",
            data: 7
        }
        "/path/to/resource with spaces" => {
            route: "/path/to/resource with spaces",
            data: 7
        }
        "/encoded%2Fslash" => {
            route: "/encoded/slash",
            data: 8
        }
        "/encoded/slash" => {
            route: "/encoded/slash",
            data: 8
        }
    });

    Ok(())
}

#[test]
fn percent_encoding_insert() {
    let mut router = Router::new();

    let error = router.insert("/hello%20world", 0).unwrap_err();
    insta::assert_snapshot!(error, @r###"
    encoded path

         Input: /hello%20world
       Decoded: /hello world

    The router expects paths to be in their decoded form
    "###);
}
