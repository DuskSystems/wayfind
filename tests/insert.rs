use std::error::Error;
use wayfind::{
    errors::{ConstraintError, InsertError, RouteError},
    Constraint, RoutableBuilder, Router,
};

#[test]
fn test_insert_conflict() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/test").build()?;
    router.insert(&route, 1)?;

    let route = RoutableBuilder::new().route("/test").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::DuplicateRoute {
            route: "/test".to_owned(),
            conflict: "/test".to_owned()
        })
    );

    let route = RoutableBuilder::new().route("(/test)").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::DuplicateRoute {
            route: "(/test)".to_owned(),
            conflict: "/test".to_owned()
        })
    );

    insta::assert_snapshot!(router, @"/test [*]");

    Ok(())
}

#[test]
fn test_insert_conflict_expanded() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("(/test)").build()?;
    router.insert(&route, 1)?;

    let route = RoutableBuilder::new().route("/test").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::DuplicateRoute {
            route: "/test".to_owned(),
            conflict: "(/test)".to_owned()
        })
    );

    let route = RoutableBuilder::new().route("(/test)").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::DuplicateRoute {
            route: "(/test)".to_owned(),
            conflict: "(/test)".to_owned()
        })
    );

    insta::assert_snapshot!(router, @"");

    Ok(())
}

// NOTE: End wildcards have some duplicate code, so worth testing seperate to rest.
#[test]
fn test_insert_conflict_end_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("(/{*catch_all})").build()?;
    router.insert(&route, 1)?;

    let route = RoutableBuilder::new().route("/{*catch_all}").build()?;
    let insert = router.insert(&route, 2);
    assert_eq!(
        insert,
        Err(InsertError::DuplicateRoute {
            route: "/{*catch_all}".to_owned(),
            conflict: "(/{*catch_all})".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ {*catch_all} [*]
    ");

    Ok(())
}

#[test]
fn test_insert_duplicate_parameter() {
    let mut router = Router::new();

    let route = RoutableBuilder::new()
        .route("/{*id}/users/{id}")
        .build()
        .unwrap();
    let insert = router.insert(&route, 3);
    assert_eq!(
        insert,
        Err(InsertError::RouteError(RouteError::DuplicateParameter {
            route: "/{*id}/users/{id}".to_owned(),
            name: "id".to_owned(),
            first: 1,
            first_length: 5,
            second: 13,
            second_length: 4
        }))
    );

    insta::assert_snapshot!(router, @"");
}

#[test]
fn test_insert_constraint_conflict() {
    struct MyConstraint;
    impl Constraint for MyConstraint {
        const NAME: &'static str = "u32";
        fn check(segment: &str) -> bool {
            segment.parse::<u32>().is_ok()
        }
    }

    let mut router: Router<'_, usize> = Router::new();
    let constraint = router.constraint::<MyConstraint>();
    assert_eq!(
        constraint,
        Err(ConstraintError::DuplicateName {
            name: "u32",
            existing_type: "u32",
            new_type: "insert::test_insert_constraint_conflict::MyConstraint"
        })
    );
}
