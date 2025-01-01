use similar_asserts::assert_eq;
use smallvec::smallvec;
use std::error::Error;
use wayfind::{
    AuthorityMatch, Match, MethodMatch, PathMatch, RequestBuilder, RouteBuilder, Router,
};

#[test]
fn test_optional_starting() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("(/{lang})/users").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /
    ├─ users [*:1]
    ╰─ {lang}
       ╰─ /users [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/en/users").build()?;
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
                route: "(/{lang})/users".into(),
                expanded: Some("/{lang}/users".into()),
                parameters: smallvec![("lang", "en")],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/users").build()?;
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
                route: "(/{lang})/users".into(),
                expanded: Some("/users".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
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
    === Authority
    Empty
    === Path
    /users [*:1]
    ╰─ / [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/users").build()?;
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
                route: "/users(/)".into(),
                expanded: Some("/users".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/users/").build()?;
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
                route: "/users(/)".into(),
                expanded: Some("/users/".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
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
    === Authority
    Empty
    === Path
    / [*:1]
    ╰─ a [*:1]
       ╰─ /b [*:1]
          ╰─ /c [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/a/b/c").build()?;
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
                route: "(/a(/b(/c)))".into(),
                expanded: Some("/a/b/c".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/a/b").build()?;
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
                route: "(/a(/b(/c)))".into(),
                expanded: Some("/a/b".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

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
                route: "(/a(/b(/c)))".into(),
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
                route: "(/a(/b(/c)))".into(),
                expanded: Some("/".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
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
    === Authority
    Empty
    === Path
    / [*:1]
    ╰─ test [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/test").build()?;
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
                route: "(/test)".into(),
                expanded: Some("/test".into()),
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
                route: "(/test)".into(),
                expanded: Some("/".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
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
    === Authority
    Empty
    === Path
    / [*:1]
    ├─ a [*:1]
    │  ╰─ /
    │     ├─ b [*:1]
    │     │  ╰─ /c [*:1]
    │     ╰─ c [*:1]
    ├─ b [*:1]
    │  ╰─ /c [*:1]
    ╰─ c [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/a/b/c").build()?;
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
                route: "(/a)(/b)(/c)".into(),
                expanded: Some("/a/b/c".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/a/b").build()?;
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
                route: "(/a)(/b)(/c)".into(),
                expanded: Some("/a/b".into(),),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/a/c").build()?;
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
                route: "(/a)(/b)(/c)".into(),
                expanded: Some("/a/c".into(),),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

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
                route: "(/a)(/b)(/c)".into(),
                expanded: Some("/a".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/b/c").build()?;
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
                route: "(/a)(/b)(/c)".into(),
                expanded: Some("/b/c".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/b").build()?;
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
                route: "(/a)(/b)(/c)".into(),
                expanded: Some("/b".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new().path("/c").build()?;
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
                route: "(/a)(/b)(/c)".into(),
                expanded: Some("/c".into()),
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
                route: "(/a)(/b)(/c)".into(),
                expanded: Some("/".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    Ok(())
}
