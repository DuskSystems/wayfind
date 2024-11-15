use smallvec::smallvec;
use std::error::Error;
use wayfind::{
    errors::{ConstraintError, InsertError},
    Constraint, Match, Path, RoutableBuilder, Router,
};

struct NameConstraint;
impl Constraint for NameConstraint {
    const NAME: &'static str = "name";

    fn check(segment: &str) -> bool {
        segment.chars().all(|c| c.is_alphanumeric() || c == '/')
    }
}

#[test]
fn test_constraint_dynamic() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.constraint::<NameConstraint>()?;

    let route = RoutableBuilder::new().route("/users/{id:name}").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {id:name} [*]
    ");

    let path = Path::new("/users/john123")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users/{id:name}",
            expanded: None,
            data: &1,
            parameters: smallvec![("id", "john123")],
        })
    );

    let path = Path::new("/users/john@123")?;
    let search = router.search(&path)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_constraint_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.constraint::<NameConstraint>()?;

    let route = RoutableBuilder::new()
        .route("/users/{*path:name}")
        .build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {*path:name} [*]
    ");

    let path = Path::new("/users/john/doe123")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users/{*path:name}",
            expanded: None,
            data: &1,
            parameters: smallvec![("path", "john/doe123")],
        })
    );

    let path = Path::new("/users/john@doe/123")?;
    let search = router.search(&path)?;
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_constraint_unknown() {
    let mut router = Router::new();

    let route = RoutableBuilder::new()
        .route("/users/{id:unknown}")
        .build()
        .unwrap();
    let result = router.insert(&route, 1);

    assert_eq!(
        result,
        Err(InsertError::UnknownConstraint {
            constraint: "unknown".to_owned()
        })
    );
}

#[test]
fn test_constraint_conflict() -> Result<(), Box<dyn Error>> {
    struct Constraint1;
    impl Constraint for Constraint1 {
        const NAME: &'static str = "test";
        fn check(segment: &str) -> bool {
            segment == "1"
        }
    }

    struct Constraint2;
    impl Constraint for Constraint2 {
        const NAME: &'static str = "test";
        fn check(segment: &str) -> bool {
            segment == "2"
        }
    }

    let mut router: Router<'_, usize> = Router::new();
    router.constraint::<Constraint1>()?;

    let result = router.constraint::<Constraint2>();
    assert_eq!(
        result,
        Err(ConstraintError::DuplicateName {
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

    let route = RoutableBuilder::new().route("/users/{id}").build()?;
    router.insert(&route, 1)?;
    let route = RoutableBuilder::new().route("/users/{id:u32}").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ├─ {id:u32} [*]
    ╰─ {id} [*]
    ");

    let path = Path::new("/users/abc")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users/{id}",
            expanded: None,
            data: &1,
            parameters: smallvec![("id", "abc")],
        })
    );

    let path = Path::new("/users/123")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users/{id:u32}",
            expanded: None,
            data: &2,
            parameters: smallvec![("id", "123")],
        })
    );

    Ok(())
}

// NOTE: Not really happy with this. But no real way we could prevent unreachable routes at the constraint layer.
#[test]
fn test_constraint_unreachable() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.constraint::<NameConstraint>()?;

    let route = RoutableBuilder::new().route("/users/{id:u32}").build()?;
    router.insert(&route, 1)?;
    let route = RoutableBuilder::new().route("/users/{id:name}").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ├─ {id:name} [*]
    ╰─ {id:u32} [*]
    ");

    let path = Path::new("/users/123")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users/{id:name}",
            expanded: None,
            data: &2,
            parameters: smallvec![("id", "123")],
        })
    );

    let path = Path::new("/users/abc123")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users/{id:name}",
            expanded: None,
            data: &2,
            parameters: smallvec![("id", "abc123")],
        })
    );

    Ok(())
}
