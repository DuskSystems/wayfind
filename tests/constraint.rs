use smallvec::smallvec;
use std::error::Error;
use wayfind::{
    errors::{InsertError, PathConstraintError, PathInsertError},
    Match, PathConstraint, PathMatch, RequestBuilder, RouteBuilder, Router,
};

struct NameConstraint;
impl PathConstraint for NameConstraint {
    const NAME: &'static str = "name";

    fn check(segment: &str) -> bool {
        segment.chars().all(|c| c.is_alphanumeric() || c == '/')
    }
}

#[test]
fn test_constraint_dynamic() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.path.constraint::<NameConstraint>()?;

    let route = RouteBuilder::new().route("/users/{id:name}").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ╰─ {id:name} [*]
    ");

    let request = RequestBuilder::new().path("/users/john123").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/users/{id:name}",
                expanded: None,
                parameters: smallvec![("id", "john123")],
            },
        })
    );

    let request = RequestBuilder::new().path("/users/john@123").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_constraint_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.path.constraint::<NameConstraint>()?;

    let route = RouteBuilder::new().route("/users/{*path:name}").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ╰─ {*path:name} [*]
    ");

    let request = RequestBuilder::new().path("/users/john/doe123").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/users/{*path:name}",
                expanded: None,
                parameters: smallvec![("path", "john/doe123")],
            },
        })
    );

    let request = RequestBuilder::new().path("/users/john@doe/123").build()?;
    let search = router.search(&request)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_constraint_unknown() {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/users/{id:unknown}")
        .build()
        .unwrap();
    let result = router.insert(&route, 1);

    assert_eq!(
        result,
        Err(InsertError::Path(PathInsertError::UnknownConstraint {
            constraint: "unknown".to_owned()
        }))
    );
}

#[test]
fn test_constraint_conflict() -> Result<(), Box<dyn Error>> {
    struct Constraint1;
    impl PathConstraint for Constraint1 {
        const NAME: &'static str = "test";
        fn check(segment: &str) -> bool {
            segment == "1"
        }
    }

    struct Constraint2;
    impl PathConstraint for Constraint2 {
        const NAME: &'static str = "test";
        fn check(segment: &str) -> bool {
            segment == "2"
        }
    }

    let mut router: Router<'_, usize> = Router::new();
    router.path.constraint::<Constraint1>()?;

    let result = router.path.constraint::<Constraint2>();
    assert_eq!(
        result,
        Err(PathConstraintError::DuplicateName {
            name: "test",
            existing_type: "constraint::test_constraint_conflict::Constraint1",
            new_type: "constraint::test_constraint_conflict::Constraint2"
        })
    );

    Ok(())
}

#[test]
fn test_constraint_builtin() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users/{id}").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/users/{id:u32}").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ├─ {id:u32} [*]
    ╰─ {id} [*]
    ");

    let request = RequestBuilder::new().path("/users/abc").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/users/{id}",
                expanded: None,
                parameters: smallvec![("id", "abc")],
            },
        })
    );

    let request = RequestBuilder::new().path("/users/123").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            path: PathMatch {
                route: "/users/{id:u32}",
                expanded: None,
                parameters: smallvec![("id", "123")],
            },
        })
    );

    Ok(())
}

// NOTE: Not really happy with this. But no real way we could prevent unreachable routes at the constraint layer.
#[test]
fn test_constraint_unreachable() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.path.constraint::<NameConstraint>()?;

    let route = RouteBuilder::new().route("/users/{id:u32}").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/users/{id:name}").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ├─ {id:name} [*]
    ╰─ {id:u32} [*]
    ");

    let request = RequestBuilder::new().path("/users/123").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            path: PathMatch {
                route: "/users/{id:name}",
                expanded: None,
                parameters: smallvec![("id", "123")],
            },
        })
    );

    let request = RequestBuilder::new().path("/users/abc123").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            path: PathMatch {
                route: "/users/{id:name}",
                expanded: None,
                parameters: smallvec![("id", "abc123")],
            },
        })
    );

    Ok(())
}
