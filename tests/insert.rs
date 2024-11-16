use std::error::Error;
use wayfind::{
    errors::{InsertError, PathConstraintError, PathInsertError, PathRouteError},
    DataChain, MethodId, PathConstraint, PathId, RouteBuilder, Router,
};

#[test]
fn test_insert_conflict() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/test").build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new().route("/test").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            chain: DataChain {
                path: PathId(1),
                method: MethodId(None),
            }
        })
    );

    let route = RouteBuilder::new().route("(/test)").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::Path(PathInsertError::DuplicateRoute {
            id: PathId(1),
        }))
    );

    insta::assert_snapshot!(router, @r"
    === Path
    / [3]
    ╰─ test [1]
    === Method
    Empty
    === Chains
    1-*
    ");

    Ok(())
}

#[test]
fn test_insert_conflict_expanded() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("(/test)").build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new().route("/test").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            chain: DataChain {
                path: PathId(1),
                method: MethodId(None),
            }
        })
    );

    let route = RouteBuilder::new().route("(/test)").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            chain: DataChain {
                path: PathId(1),
                method: MethodId(None),
            }
        })
    );

    insta::assert_snapshot!(router, @r"
    === Path
    / [1]
    ╰─ test [1]
    === Method
    Empty
    === Chains
    1-*
    ");

    Ok(())
}

// NOTE: End wildcards have some duplicate code, so worth testing seperate to rest.
#[test]
fn test_insert_conflict_end_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("(/{*catch_all})").build()?;
    router.insert(&route, 1)?;

    let route = RouteBuilder::new().route("/{*catch_all}").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            chain: DataChain {
                path: PathId(1),
                method: MethodId(None),
            }
        })
    );

    insta::assert_snapshot!(router, @r"
    === Path
    / [1]
    ╰─ {*catch_all} [1]
    === Method
    Empty
    === Chains
    1-*
    ");

    Ok(())
}

#[test]
fn test_insert_duplicate_parameter() {
    let mut router = Router::new();

    let route = RouteBuilder::new()
        .route("/{*id}/users/{id}")
        .build()
        .unwrap();
    let insert = router.insert(&route, 3);
    assert_eq!(
        insert,
        Err(InsertError::Path(PathInsertError::PathRouteError(
            PathRouteError::DuplicateParameter {
                route: "/{*id}/users/{id}".to_owned(),
                name: "id".to_owned(),
                first: 1,
                first_length: 5,
                second: 13,
                second_length: 4
            }
        )))
    );

    insta::assert_snapshot!(router, @r"
    === Path
    Empty
    === Method
    Empty
    === Chains
    Empty
    ");
}

#[test]
fn test_insert_constraint_conflict() {
    struct MyConstraint;
    impl PathConstraint for MyConstraint {
        const NAME: &'static str = "u32";
        fn check(segment: &str) -> bool {
            segment.parse::<u32>().is_ok()
        }
    }

    let mut router: Router<'_, usize> = Router::new();
    let constraint = router.path.constraint::<MyConstraint>();
    assert_eq!(
        constraint,
        Err(PathConstraintError::DuplicateName {
            name: "u32",
            existing_type: "u32",
            new_type: "insert::test_insert_constraint_conflict::MyConstraint"
        })
    );
}
