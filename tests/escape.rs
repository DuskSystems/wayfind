use std::error::Error;

use similar_asserts::assert_eq;
use smallvec::smallvec;
use wayfind::{Match, Router};

#[test]
fn test_escape_parameter() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert(r"/users/\<id\>", 1)?; // "/users/<id>"

    insta::assert_snapshot!(router, @"/users/<id> [*]");

    let search = router.search("/users/<id>");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: r"/users/\<id\>",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/users/123");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_escape_group() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert(r"/\(not-optional\)", 1)?; // "/(not-optional)"

    insta::assert_snapshot!(router, @"/(not-optional) [*]");

    let search = router.search("/(not-optional)");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: r"/\(not-optional\)",
            expanded: None,
            parameters: smallvec![],
        })
    );

    let search = router.search("/optional");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_escape_nested() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert(r"(/a(/\<param\>))", 1)?; // "(/a(/<param>))"

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ a [*]
       ╰─ /<param> [*]
    ");

    let search = router.search("/a/<param>");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: r"(/a(/\<param\>))",
            expanded: Some("/a/\\<param\\>"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/a/value");
    assert_eq!(search, None);

    let search = router.search("/a");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: r"(/a(/\<param\>))",
            expanded: Some("/a"),
            parameters: smallvec![],
        })
    );

    let search = router.search("/");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: r"(/a(/\<param\>))",
            expanded: Some("/"),
            parameters: smallvec![],
        })
    );

    Ok(())
}
