use smallvec::smallvec;
use std::error::Error;
use wayfind::{Match, PathMatch, RequestBuilder, RouteBuilder, Router};

#[test]
fn test_escape_parameter() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route(r"/users/\{id\}").build()?; // "/users/{id}"
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users/{id} [1]
    === Chains
    1
    ");

    let request = RequestBuilder::new().path("/users/{id}").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: r"/users/\{id\}",
                expanded: None,
                parameters: smallvec![],
            },
        })
    );
    let request = RequestBuilder::new().path("/users/123").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_escape_group() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route(r"/\(not-optional\)").build()?; // "/(not-optional)"
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /(not-optional) [1]
    === Chains
    1
    ");

    let request = RequestBuilder::new().path("/(not-optional)").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: r"/\(not-optional\)",
                expanded: None,
                parameters: smallvec![],
            },
        })
    );

    let request = RequestBuilder::new().path("/optional").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

// FIXME: We're missing parser tests for this.
#[test]
fn test_escape_nested() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route(r"(/a(/\{param\}))").build()?; // "(/a(/{param}))"
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    / [1]
    ╰─ a [1]
       ╰─ /{param} [1]
    === Chains
    1
    ");

    let request = RequestBuilder::new().path("/a/{param}").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: r"(/a(/\{param\}))",
                expanded: Some("/a/\\{param\\}"),
                parameters: smallvec![],
            },
        })
    );

    let request = RequestBuilder::new().path("/a/value").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new().path("/a").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: r"(/a(/\{param\}))",
                expanded: Some("/a"),
                parameters: smallvec![],
            },
        })
    );

    let request = RequestBuilder::new().path("/").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: r"(/a(/\{param\}))",
                expanded: Some("/"),
                parameters: smallvec![],
            },
        })
    );

    Ok(())
}
