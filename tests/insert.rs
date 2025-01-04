use std::error::Error;

use similar_asserts::assert_eq;
use wayfind::{
    errors::{ConstraintError, InsertError, TemplateError},
    Constraint, Router,
};

#[test]
fn test_insert_conflict() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/test", 1)?;

    let insert = router.insert("/test", 2);
    assert_eq!(insert, Err(InsertError::Conflict));

    let insert = router.insert("(/test)", 3);
    assert_eq!(insert, Err(InsertError::Conflict));

    insta::assert_snapshot!(router, @"/test [*]");

    Ok(())
}

#[test]
fn test_insert_conflict_expanded() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("(/test)", 1)?;

    let insert = router.insert("/test", 2);
    assert_eq!(insert, Err(InsertError::Conflict));

    let insert = router.insert("(/test)", 2);
    assert_eq!(insert, Err(InsertError::Conflict));

    let insert = router.insert("(/best)", 3);
    assert_eq!(insert, Err(InsertError::Conflict));

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ test [*]
    ");

    Ok(())
}

#[test]
fn test_insert_conflict_multiple_expanded() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("(/hello)", 1)?;

    let insert = router.insert("(/world)", 2);
    assert_eq!(insert, Err(InsertError::Conflict));

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ hello [*]
    ");

    Ok(())
}

// NOTE: End wildcards have some duplicate code, so worth testing seperate to rest.
#[test]
fn test_insert_conflict_end_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("(/{*catch_all})", 1)?;

    let insert = router.insert("/{*catch_all}", 2);
    assert_eq!(insert, Err(InsertError::Conflict));

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ {*catch_all} [*]
    ");

    Ok(())
}

#[test]
fn test_insert_conflict_overlapping() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/a(/b)", 1)?;
    router.insert("/x/y", 2)?;

    let insert = router.insert("(/a(/b))(/x/y)", 3);
    assert_eq!(insert, Err(InsertError::Conflict));

    insta::assert_snapshot!(router, @r"
    /
    ├─ x/y [*]
    ╰─ a [*]
       ╰─ /b [*]
    ");

    Ok(())
}

#[test]
fn test_insert_duplicate_parameter() {
    let mut router = Router::new();

    let insert = router.insert("/{*id}/users/{id}", 3);
    assert_eq!(
        insert,
        Err(InsertError::Template(TemplateError::DuplicateParameter {
            template: "/{*id}/users/{id}".to_owned(),
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
