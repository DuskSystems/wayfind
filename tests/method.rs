use similar_asserts::assert_eq;
use smallvec::smallvec;
use std::error::Error;
use wayfind::{
    errors::{
        DeleteError, InsertError, MethodDeleteError, MethodInsertError, MethodSearchError,
        SearchError,
    },
    AuthorityMatch, Match, MethodMatch, PathMatch, RequestBuilder, RouteBuilder, Router,
};

#[test]
fn test_method_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ╰─ GET [1]
    === Chains
    *-1-1
    ");

    let request = RequestBuilder::new().path("/users").method("GET").build()?;
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
            method: MethodMatch {
                method: Some("GET")
            }
        })
    );

    Ok(())
}

#[test]
fn test_method_not_allowed() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ╰─ GET [1]
    === Chains
    *-1-1
    ");

    let request = RequestBuilder::new()
        .path("/users")
        .method("POST")
        .build()?;
    let search = router.search(&request);
    assert_eq!(
        search,
        Err(SearchError::Method(MethodSearchError::NotAllowed))
    );

    Ok(())
}

#[test]
fn test_method_multiple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET", "POST"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ├─ GET [1]
    ╰─ POST [1]
    === Chains
    *-1-1
    ");

    let request = RequestBuilder::new().path("/users").method("GET").build()?;
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
            method: MethodMatch {
                method: Some("GET")
            }
        })
    );

    let request = RequestBuilder::new()
        .path("/users")
        .method("POST")
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
            method: MethodMatch {
                method: Some("POST")
            }
        })
    );

    let request = RequestBuilder::new()
        .path("/users")
        .method("DELETE")
        .build()?;
    let search = router.search(&request);
    assert_eq!(
        search,
        Err(SearchError::Method(MethodSearchError::NotAllowed))
    );

    Ok(())
}

#[test]
fn test_method_empty() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec![])
        .build()?;

    let insert = router.insert(&route, 1);
    assert_eq!(insert, Err(InsertError::Method(MethodInsertError::Empty)));

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    Empty
    === Chains
    Empty
    ");

    Ok(())
}

#[test]
fn test_method_none() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let request = RequestBuilder::new().path("/users").method("GET").build()?;
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
        .path("/users")
        .method("POST")
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

    Ok(())
}

#[test]
fn test_method_same_path() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ╰─ GET [1]
    === Chains
    *-1-1
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["PUT"])
        .build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ├─ GET [1]
    ╰─ PUT [2]
    === Chains
    *-1-1
    *-1-2
    ");

    let request = RequestBuilder::new().path("/users").method("GET").build()?;
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
            method: MethodMatch {
                method: Some("GET")
            }
        })
    );

    let request = RequestBuilder::new().path("/users").method("PUT").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            authority: AuthorityMatch {
                authority: None,
                parameters: smallvec![]
            },
            path: PathMatch {
                route: "/users".into(),
                expanded: None,
                parameters: smallvec![],
            },
            method: MethodMatch {
                method: Some("PUT")
            }
        })
    );

    Ok(())
}

#[test]
fn test_method_same_route_catch() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ╰─ GET [1]
    === Chains
    *-1-1
    ");

    let route = RouteBuilder::new().route("/users").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ╰─ GET [1]
    === Chains
    *-1-*
    *-1-1
    ");

    let request = RequestBuilder::new().path("/users").method("GET").build()?;
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
            method: MethodMatch {
                method: Some("GET")
            }
        })
    );

    let request = RequestBuilder::new().path("/users").method("PUT").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
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
fn test_method_same_route_catch_backwards() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    Empty
    === Chains
    *-1-*
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ╰─ GET [1]
    === Chains
    *-1-*
    *-1-1
    ");

    let request = RequestBuilder::new().path("/users").method("GET").build()?;
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
            method: MethodMatch {
                method: Some("GET")
            }
        })
    );

    let request = RequestBuilder::new().path("/users").method("PUT").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
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
fn test_method_conflict_direct() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ╰─ GET [1]
    === Chains
    *-1-1
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    let insert = router.insert(&route, 1);
    assert_eq!(
        insert,
        Err(InsertError::Method(MethodInsertError::Conflict))
    );

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ╰─ GET [1]
    === Chains
    *-1-1
    ");

    Ok(())
}

#[test]
fn test_method_conflict_list_inner() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET", "PUT"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ├─ GET [1]
    ╰─ PUT [1]
    === Chains
    *-1-1
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["PUT"])
        .build()?;
    let insert = router.insert(&route, 1);
    assert_eq!(
        insert,
        Err(InsertError::Method(MethodInsertError::Conflict))
    );

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ├─ GET [1]
    ╰─ PUT [1]
    === Chains
    *-1-1
    ");

    Ok(())
}

#[test]
fn test_method_conflict_list_outer() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ╰─ GET [1]
    === Chains
    *-1-1
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["PUT", "GET"])
        .build()?;
    let search = router.insert(&route, 1);
    assert_eq!(
        search,
        Err(InsertError::Method(MethodInsertError::Conflict))
    );

    Ok(())
}

#[test]
fn test_method_delete() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ╰─ GET [1]
    === Chains
    *-1-1
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    Empty
    === Method
    Empty
    === Chains
    Empty
    ");

    Ok(())
}

#[test]
fn test_method_delete_list() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET", "POST"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ├─ GET [1]
    ╰─ POST [1]
    === Chains
    *-1-1
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET", "POST"])
        .build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    Empty
    === Method
    Empty
    === Chains
    Empty
    ");

    Ok(())
}

#[test]
fn test_method_delete_mismatch_list_inner() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET", "PUT"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ├─ GET [1]
    ╰─ PUT [1]
    === Chains
    *-1-1
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["PUT"])
        .build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::Method(MethodDeleteError::Mismatch))
    );

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ├─ GET [1]
    ╰─ PUT [1]
    === Chains
    *-1-1
    ");

    Ok(())
}

#[test]
fn test_method_delete_mismatch_list_outer() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ╰─ GET [1]
    === Chains
    *-1-1
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["PUT", "GET"])
        .build()?;
    let delete = router.delete(&route);
    assert_eq!(
        delete,
        Err(DeleteError::Method(MethodDeleteError::Mismatch))
    );

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    /users [*:1]
    === Method
    [1]
    ╰─ GET [1]
    === Chains
    *-1-1
    ");

    Ok(())
}
