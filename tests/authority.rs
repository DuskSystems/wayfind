use smallvec::smallvec;
use std::error::Error;
use wayfind::{
    AuthorityMatch, Match, MethodMatch, PathMatch, RequestBuilder, RouteBuilder, Router,
};

#[test]
fn test_authority_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .authority("api.example.com")
        .route("/users")
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    api.example.com [*:1]
    === Path
    /users [1:1]
    === Method
    Empty
    === Chains
    1-1-*
    ");

    let request = RequestBuilder::new()
        .authority("api.example.com")
        .path("/users")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: Some("api.example.com".into()),
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/users".into(),
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    Ok(())
}

#[test]
fn test_authority_parameter() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .authority("{tenant}.example.com")
        .route("/users")
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    {tenant}
    ╰─ .example.com [*:1]
    === Path
    /users [1:1]
    === Method
    Empty
    === Chains
    1-1-*
    ");

    let request = RequestBuilder::new()
        .authority("acme.example.com")
        .path("/users")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: Some("{tenant}.example.com".into()),
                parameters: smallvec![("tenant", "acme")],
            },
            path: PathMatch {
                route: "/users".into(),
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    Ok(())
}

#[test]
fn test_authority_multiple_parameters() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .authority("{tenant}.{region}.example.com")
        .route("/users")
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    {tenant}
    ╰─ .
       ╰─ {region}
          ╰─ .example.com [*:1]
    === Path
    /users [1:1]
    === Method
    Empty
    === Chains
    1-1-*
    ");

    let request = RequestBuilder::new()
        .authority("acme.us-east.example.com")
        .path("/users")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: Some("{tenant}.{region}.example.com".into()),
                parameters: smallvec![("tenant", "acme"), ("region", "us-east")],
            },
            path: PathMatch {
                route: "/users".into(),
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    Ok(())
}

#[test]
fn test_authority_priority() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .authority("api.example.com")
        .route("/users")
        .build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new()
        .authority("{tenant}.example.com")
        .route("/users")
        .build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    api.example.com [*:1]
    {tenant}
    ╰─ .example.com [*:2]
    === Path
    /users [1:1, 2:2]
    === Method
    Empty
    === Chains
    1-1-*
    2-2-*
    ");

    let request = RequestBuilder::new()
        .authority("api.example.com")
        .path("/users")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: Some("api.example.com".into()),
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/users".into(),
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new()
        .authority("acme.example.com")
        .path("/users")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            authority: AuthorityMatch {
                authority: Some("{tenant}.example.com".into()),
                parameters: smallvec![("tenant", "acme")],
            },
            path: PathMatch {
                route: "/users".into(),
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    Ok(())
}

#[test]
fn test_authority_fallback() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users").build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new()
        .authority("api.example.com")
        .route("/users")
        .build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    api.example.com [*:1]
    === Path
    /users [*:1, 1:2]
    === Method
    Empty
    === Chains
    *-1-*
    1-2-*
    ");

    let request = RequestBuilder::new()
        .authority("api.example.com")
        .path("/users")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            authority: AuthorityMatch {
                authority: Some("api.example.com".into()),
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/users".into(),
                expanded: None,
                parameters: smallvec![],
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
                route: "/users".into(),
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new()
        .authority("other.example.com")
        .path("/users")
        .build()?;
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
                route: "/users".into(),
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    Ok(())
}

#[test]
fn test_authority_path_expansion() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .authority("api.example.com")
        .route("(/hello)")
        .build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new()
        .authority("{tenant}.example.com")
        .route("(/world)")
        .build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    api.example.com [*:1]
    {tenant}
    ╰─ .example.com [*:2]
    === Path
    / [1:1, 2:2]
    ├─ hello [1:1]
    ╰─ world [2:2]
    === Method
    Empty
    === Chains
    1-1-*
    2-2-*
    ");

    let request = RequestBuilder::new()
        .authority("api.example.com")
        .path("/world")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new()
        .authority("acme.example.com")
        .path("/hello")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new()
        .authority("api.example.com")
        .path("/hello")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: Some("api.example.com".into()),
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "(/hello)".into(),
                expanded: Some("/hello".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new()
        .authority("acme.example.com")
        .path("/world")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            authority: AuthorityMatch {
                authority: Some("{tenant}.example.com".into()),
                parameters: smallvec![("tenant", "acme")]
            },
            path: PathMatch {
                route: "(/world)".into(),
                expanded: Some("/world".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new()
        .authority("unknown.com")
        .path("/")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    let request = RequestBuilder::new()
        .authority("api.example.com")
        .path("/")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            authority: AuthorityMatch {
                authority: Some("api.example.com".into()),
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "(/hello)".into(),
                expanded: Some("/".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    let request = RequestBuilder::new()
        .authority("acme.example.com")
        .path("/")
        .build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            authority: AuthorityMatch {
                authority: Some("{tenant}.example.com".into()),
                parameters: smallvec![("tenant", "acme")]
            },
            path: PathMatch {
                route: "(/world)".into(),
                expanded: Some("/".into()),
                parameters: smallvec![],
            },
            method: MethodMatch { method: None }
        })
    );

    Ok(())
}
