use smallvec::smallvec;
use std::error::Error;
use wayfind::{Match, RequestBuilder, RoutableBuilder, Router};

#[test]
fn test_optional_starting() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("(/{lang})/users").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ users [*]
    ╰─ {lang}
       ╰─ /users [*]
    ");

    let request = RequestBuilder::new().path("/en/users").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/{lang})/users",
            expanded: Some("/{lang}/users"),
            data: &1,
            parameters: smallvec![("lang", "en")],
        })
    );

    let request = RequestBuilder::new().path("/users").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/{lang})/users",
            expanded: Some("/users"),
            data: &1,
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn test_optional_ending() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/users(/)").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /users [*]
    ╰─ / [*]
    ");

    let request = RequestBuilder::new().path("/users").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users(/)",
            expanded: Some("/users"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/users/").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users(/)",
            expanded: Some("/users/"),
            data: &1,
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn test_optional_nested() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("(/a(/b(/c)))").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ a [*]
       ╰─ /b [*]
          ╰─ /c [*]
    ");

    let request = RequestBuilder::new().path("/a/b/c").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a(/b(/c)))",
            expanded: Some("/a/b/c"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/a/b").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a(/b(/c)))",
            expanded: Some("/a/b"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/a").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a(/b(/c)))",
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
            route: "(/a(/b(/c)))",
            expanded: Some("/"),
            data: &1,
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn test_optional_only() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("(/test)").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ test [*]
    ");

    let request = RequestBuilder::new().path("/test").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/test)",
            expanded: Some("/test"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/test)",
            expanded: Some("/"),
            data: &1,
            parameters: smallvec![],
        })
    );

    Ok(())
}

#[test]
fn test_optional_touching() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("(/a)(/b)(/c)").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    / [*]
    ├─ a [*]
    │  ╰─ /
    │     ├─ b [*]
    │     │  ╰─ /c [*]
    │     ╰─ c [*]
    ├─ b [*]
    │  ╰─ /c [*]
    ╰─ c [*]
    ");

    let request = RequestBuilder::new().path("/a/b/c").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/a/b/c"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/a/b").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/a/b"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/a/c").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/a/c"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/a").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/a"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/b/c").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/b/c"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/b").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/b"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/c").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/c"),
            data: &1,
            parameters: smallvec![],
        })
    );

    let request = RequestBuilder::new().path("/").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            route: "(/a)(/b)(/c)",
            expanded: Some("/"),
            data: &1,
            parameters: smallvec![],
        })
    );

    Ok(())
}
