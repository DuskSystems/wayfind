use std::error::Error;

use similar_asserts::assert_eq;
use smallvec::smallvec;
use wayfind::{
    errors::{ConstraintError, InsertError},
    Constraint, Match, Router,
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
    router.insert("/users/{id:name}", 1)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {id:name} [*]
    ");

    let search = router.search("/users/john123");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users/{id:name}",
            expanded: None,
            parameters: smallvec![("id", "john123")],
        })
    );

    let search = router.search("/users/john@123");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_constraint_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.constraint::<NameConstraint>()?;
    router.insert("/users/{*path:name}", 1)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {*path:name} [*]
    ");

    let search = router.search("/users/john/doe123");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users/{*path:name}",
            expanded: None,
            parameters: smallvec![("path", "john/doe123")],
        })
    );

    let search = router.search("/users/john@doe/123");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn test_constraint_unknown() {
    let mut router = Router::new();

    let result = router.insert("/users/{id:unknown}", 1);
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
    router.insert("/users/{id}", 1)?;
    router.insert("/users/{id:u32}", 2)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ├─ {id:u32} [*]
    ╰─ {id} [*]
    ");

    let search = router.search("/users/abc");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/users/{id}",
            expanded: None,
            parameters: smallvec![("id", "abc")],
        })
    );

    let search = router.search("/users/123");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/users/{id:u32}",
            expanded: None,
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
    router.insert("/users/{id:u32}", 1)?;
    router.insert("/users/{id:name}", 2)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ├─ {id:name} [*]
    ╰─ {id:u32} [*]
    ");

    let search = router.search("/users/123");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/users/{id:name}",
            expanded: None,
            parameters: smallvec![("id", "123")],
        })
    );

    let search = router.search("/users/abc123");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/users/{id:name}",
            expanded: None,
            parameters: smallvec![("id", "abc123")],
        })
    );

    Ok(())
}
