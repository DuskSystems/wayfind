use smallvec::smallvec;
use std::error::Error;
use wayfind::{Match, PathMatch, RequestBuilder, RouteBuilder, Router};

#[test]
fn test_optional_starting() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("(/{lang})/users").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /
    ├─ users [1]
    ╰─ {lang}
       ╰─ /users [1]
    === Chains
    1
    ");

    let request = RequestBuilder::new().path("/en/users").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/{lang})/users",
                expanded: Some("/{lang}/users"),
                parameters: smallvec![("lang", "en")],
            },
        })
    );

    let request = RequestBuilder::new().path("/users").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/{lang})/users",
                expanded: Some("/users"),
                parameters: smallvec![],
            },
        })
    );

    Ok(())
}

#[test]
fn test_optional_ending() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users(/)").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users [1]
    ╰─ / [1]
    === Chains
    1
    ");

    let request = RequestBuilder::new().path("/users").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/users(/)",
                expanded: Some("/users"),
                parameters: smallvec![],
            },
        })
    );

    let request = RequestBuilder::new().path("/users/").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/users(/)",
                expanded: Some("/users/"),
                parameters: smallvec![],
            },
        })
    );

    Ok(())
}

#[test]
fn test_optional_nested() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("(/a(/b(/c)))").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    / [1]
    ╰─ a [1]
       ╰─ /b [1]
          ╰─ /c [1]
    === Chains
    1
    ");

    let request = RequestBuilder::new().path("/a/b/c").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/a(/b(/c)))",
                expanded: Some("/a/b/c"),
                parameters: smallvec![],
            },
        })
    );

    let request = RequestBuilder::new().path("/a/b").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/a(/b(/c)))",
                expanded: Some("/a/b"),
                parameters: smallvec![],
            },
        })
    );

    let request = RequestBuilder::new().path("/a").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/a(/b(/c)))",
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
                route: "(/a(/b(/c)))",
                expanded: Some("/"),
                parameters: smallvec![],
            },
        })
    );

    Ok(())
}

#[test]
fn test_optional_only() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("(/test)").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    / [1]
    ╰─ test [1]
    === Chains
    1
    ");

    let request = RequestBuilder::new().path("/test").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/test)",
                expanded: Some("/test"),
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
                route: "(/test)",
                expanded: Some("/"),
                parameters: smallvec![],
            },
        })
    );

    Ok(())
}

#[test]
fn test_optional_touching() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("(/a)(/b)(/c)").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    / [1]
    ├─ a [1]
    │  ╰─ /
    │     ├─ b [1]
    │     │  ╰─ /c [1]
    │     ╰─ c [1]
    ├─ b [1]
    │  ╰─ /c [1]
    ╰─ c [1]
    === Chains
    1
    ");

    let request = RequestBuilder::new().path("/a/b/c").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/a)(/b)(/c)",
                expanded: Some("/a/b/c"),
                parameters: smallvec![],
            },
        })
    );

    let request = RequestBuilder::new().path("/a/b").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/a)(/b)(/c)",
                expanded: Some("/a/b"),
                parameters: smallvec![],
            },
        })
    );

    let request = RequestBuilder::new().path("/a/c").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/a)(/b)(/c)",
                expanded: Some("/a/c"),
                parameters: smallvec![],
            },
        })
    );

    let request = RequestBuilder::new().path("/a").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/a)(/b)(/c)",
                expanded: Some("/a"),
                parameters: smallvec![],
            },
        })
    );

    let request = RequestBuilder::new().path("/b/c").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/a)(/b)(/c)",
                expanded: Some("/b/c"),
                parameters: smallvec![],
            },
        })
    );

    let request = RequestBuilder::new().path("/b").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/a)(/b)(/c)",
                expanded: Some("/b"),
                parameters: smallvec![],
            },
        })
    );

    let request = RequestBuilder::new().path("/c").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "(/a)(/b)(/c)",
                expanded: Some("/c"),
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
                route: "(/a)(/b)(/c)",
                expanded: Some("/"),
                parameters: smallvec![],
            },
        })
    );

    Ok(())
}
