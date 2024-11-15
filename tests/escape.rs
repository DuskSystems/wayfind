use smallvec::smallvec;
use std::error::Error;
use wayfind::{Match, RequestBuilder, RoutableBuilder, Router};

#[test]
fn test_escape_parameter() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route(r"/users/\{id\}").build()?; // "/users/{id}"
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /users/{id} [*]
    ");

    let request = RequestBuilder::new().path("/users/{id}").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: r"/users/\{id\}",
            expanded: None,
            data: &1,
            parameters: smallvec![],
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

    let route = RoutableBuilder::new().route(r"/\(not-optional\)").build()?; // "/(not-optional)"
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /(not-optional) [*]
    ");

    let request = RequestBuilder::new().path("/(not-optional)").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: r"/\(not-optional\)",
            expanded: None,
            data: &1,
            parameters: smallvec![],
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

    let route = RoutableBuilder::new().route(r"(/a(/\{param\}))").build()?; // "(/a(/{param}))"
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ a [*]
       ╰─ /{param} [*]
    ");

    let request = RequestBuilder::new().path("/a/{param}").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: r"(/a(/\{param\}))",
            expanded: Some("/a/\\{param\\}"),
            data: &1,
            parameters: smallvec![],
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
            route: r"(/a(/\{param\}))",
            expanded: Some("/a"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: r"(/a(/\{param\}))",
            expanded: Some("/"),
            data: &1,
            parameters: smallvec![],
        })
    );

    Ok(())
}
