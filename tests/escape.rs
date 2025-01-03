use similar_asserts::assert_eq;
use smallvec::smallvec;
use std::error::Error;
use wayfind::{
    AuthorityMatch, Match, MethodMatch, PathMatch, RequestBuilder, RouteBuilder, Router,
};

#[test]
fn test_escape_parameter() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route(r"/users/\{id\}").build()?; // "/users/{id}"
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users/{id} [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/users/{id}").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: r"/users/\{id\}".into(),
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
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
    === Authority
    Empty
    === Path
    /(not-optional) [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/(not-optional)").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: r"/\(not-optional\)".into(),
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/optional").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_escape_nested() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route(r"(/a(/\{param\}))").build()?; // "(/a(/{param}))"
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    / [*:1]
    ╰─ a [*:1]
       ╰─ /{param} [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/a/{param}").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: r"(/a(/\{param\}))".into(),
                expanded: Some("/a/\\{param\\}".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
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
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: r"(/a(/\{param\}))".into(),
                expanded: Some("/a".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: r"(/a(/\{param\}))".into(),
                expanded: Some("/".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    Ok(())
}
