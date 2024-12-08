use smallvec::smallvec;
use std::error::Error;
use wayfind::{
    errors::{
        DeleteError, InsertError, MethodDeleteError, MethodInsertError, MethodSearchError,
        SearchError,
    },
    Match, MethodMatch, PathMatch, RequestBuilder, RouteBuilder, Router,
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
    === Path
    /users [0]
    === Method
    /users
    ╰─ GET [0]
    ");

    let request = RequestBuilder::new().path("/users").method("GET").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/users",
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
    === Path
    /users [0]
    === Method
    /users
    ╰─ GET [0]
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
    === Path
    /users [0]
    === Method
    /users
    ├─ GET [0]
    ╰─ POST [0]
    ");

    let request = RequestBuilder::new().path("/users").method("GET").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/users",
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
            path: PathMatch {
                route: "/users",
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
    === Path
    /users [0]
    === Method
    ");

    Ok(())
}

#[test]
fn test_method_none() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users [0]
    === Method
    ");

    let request = RequestBuilder::new().path("/users").method("GET").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/users",
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
            path: PathMatch {
                route: "/users",
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
            path: PathMatch {
                route: "/users",
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
    === Path
    /users [0]
    === Method
    /users
    ╰─ GET [0]
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["PUT"])
        .build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users [0]
    === Method
    /users
    ├─ GET [0]
    ╰─ PUT [1]
    ");

    let request = RequestBuilder::new().path("/users").method("GET").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/users",
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
            path: PathMatch {
                route: "/users",
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
    === Path
    /users [0]
    === Method
    /users
    ╰─ GET [0]
    ");

    let route = RouteBuilder::new().route("/users").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users [0]
    === Method
    /users
    ╰─ GET [0]
    ");

    let request = RequestBuilder::new().path("/users").method("GET").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/users",
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
            path: PathMatch {
                route: "/users",
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
    === Path
    /users [0]
    === Method
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users [0]
    === Method
    /users
    ╰─ GET [0]
    ");

    let request = RequestBuilder::new().path("/users").method("GET").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/users",
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
            path: PathMatch {
                route: "/users",
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
    === Path
    /users [0]
    === Method
    /users
    ╰─ GET [0]
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    let insert = router.insert(&route, 1);
    assert_eq!(
        insert,
        Err(InsertError::Method(MethodInsertError::Conflict {
            route: "/users".to_owned(),
            method: "GET".to_owned()
        }))
    );

    insta::assert_snapshot!(router, @r"
    === Path
    /users [0]
    === Method
    /users
    ╰─ GET [0]
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
    === Path
    /users [0]
    === Method
    /users
    ├─ GET [0]
    ╰─ PUT [0]
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["PUT"])
        .build()?;
    let insert = router.insert(&route, 1);
    assert_eq!(
        insert,
        Err(InsertError::Method(MethodInsertError::Conflict {
            route: "/users".to_owned(),
            method: "PUT".to_owned()
        }))
    );

    insta::assert_snapshot!(router, @r"
    === Path
    /users [0]
    === Method
    /users
    ├─ GET [0]
    ╰─ PUT [0]
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
    === Path
    /users [0]
    === Method
    /users
    ╰─ GET [0]
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["PUT", "GET"])
        .build()?;
    let search = router.insert(&route, 1);
    assert_eq!(
        search,
        Err(InsertError::Method(MethodInsertError::Conflict {
            route: "/users".to_owned(),
            method: "GET".to_owned()
        }))
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
    === Path
    /users [0]
    === Method
    /users
    ╰─ GET [0]
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET"])
        .build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    === Path
    === Method
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
    === Path
    /users [0]
    === Method
    /users
    ├─ GET [0]
    ╰─ POST [0]
    ");

    let route = RouteBuilder::new()
        .route("/users")
        .methods(vec!["GET", "POST"])
        .build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    === Path
    === Method
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
    === Path
    /users [0]
    === Method
    /users
    ├─ GET [0]
    ╰─ PUT [0]
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
    === Path
    === Method
    /users
    ├─ GET [0]
    ╰─ PUT [0]
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
    === Path
    /users [0]
    === Method
    /users
    ╰─ GET [0]
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
    === Path
    === Method
    /users
    ╰─ GET [0]
    ");

    Ok(())
}
