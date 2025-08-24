use std::error::Error;

use similar_asserts::assert_eq;
use wayfind::{
    Router,
    errors::{InsertError, TemplateError},
};

#[test]
fn test_insert_conflict_static() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/test", 1)?;

    let insert = router.insert("/test", 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            template: "/test".to_owned(),
            conflict: "/test".to_owned()
        })
    );

    insta::assert_snapshot!(router, @"/test");

    Ok(())
}

#[test]
fn test_insert_conflict_dynamic() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<id>", 1)?;

    let insert = router.insert("/<id>", 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            template: "/<id>".to_owned(),
            conflict: "/<id>".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
    ");

    Ok(())
}

#[test]
fn test_insert_conflict_dynamic_structural() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<id>", 1)?;

    let insert = router.insert("/<user>", 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            template: "/<user>".to_owned(),
            conflict: "/<id>".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
    ");

    Ok(())
}

#[test]
fn test_insert_conflict_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*catch_all>", 1)?;

    let insert = router.insert("/<*catch_all>", 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            template: "/<*catch_all>".to_owned(),
            conflict: "/<*catch_all>".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*catch_all>
    ");

    Ok(())
}

#[test]
fn test_insert_conflict_wildcard_structural() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*catch_all>", 1)?;

    let insert = router.insert("/<*files>", 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            template: "/<*files>".to_owned(),
            conflict: "/<*catch_all>".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*catch_all>
    ");

    Ok(())
}

#[test]
fn test_insert_conflict_end_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*catch_all>", 1)?;

    let insert = router.insert("/<*catch_all>", 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            template: "/<*catch_all>".to_owned(),
            conflict: "/<*catch_all>".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*catch_all>
    ");

    Ok(())
}

#[test]
fn test_insert_conflict_end_wildcard_structural() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*catch_all>", 1)?;

    let insert = router.insert("/<*files>", 2);
    assert_eq!(
        insert,
        Err(InsertError::Conflict {
            template: "/<*files>".to_owned(),
            conflict: "/<*catch_all>".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*catch_all>
    ");

    Ok(())
}

#[test]
fn test_insert_duplicate_parameter() {
    let mut router = Router::new();
    let insert = router.insert("/<*id>/users/<id>", 3);
    assert_eq!(
        insert,
        Err(InsertError::Template(TemplateError::DuplicateParameter {
            template: "/<*id>/users/<id>".to_owned(),
            name: "id".to_owned(),
            first: 1,
            first_length: 5,
            second: 13,
            second_length: 4
        }))
    );

    insta::assert_snapshot!(router, @"");
}
