use core::error::Error;

use similar_asserts::assert_eq;
use wayfind::Router;
use wayfind::errors::{InsertError, TemplateError};

#[test]
fn insert_conflict_static() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/test", 1)?;

    let error = router.insert("/test", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            existing: "/test".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict with `/test`");
    insta::assert_snapshot!(router, @"/test");

    Ok(())
}

#[test]
fn insert_conflict_dynamic() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<id>", 1)?;

    let error = router.insert("/<id>", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            existing: "/<id>".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict with `/<id>`");
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
    ");

    Ok(())
}

#[test]
fn insert_conflict_dynamic_structural() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<id>", 1)?;

    let error = router.insert("/<user>", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            existing: "/<id>".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict with `/<id>`");
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
    ");

    Ok(())
}

#[test]
fn insert_conflict_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*catch_all>", 1)?;

    let error = router.insert("/<*catch_all>", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            existing: "/<*catch_all>".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict with `/<*catch_all>`");
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*catch_all>
    ");

    Ok(())
}

#[test]
fn insert_conflict_wildcard_structural() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*catch_all>", 1)?;

    let error = router.insert("/<*files>", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            existing: "/<*catch_all>".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict with `/<*catch_all>`");
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*catch_all>
    ");

    Ok(())
}

#[test]
fn insert_conflict_end_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*catch_all>", 1)?;

    let error = router.insert("/<*catch_all>", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            existing: "/<*catch_all>".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict with `/<*catch_all>`");
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*catch_all>
    ");

    Ok(())
}

#[test]
fn insert_conflict_end_wildcard_structural() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*catch_all>", 1)?;

    let error = router.insert("/<*files>", 2).unwrap_err();
    assert_eq!(
        error,
        InsertError::Conflict {
            existing: "/<*catch_all>".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict with `/<*catch_all>`");
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*catch_all>
    ");

    Ok(())
}

#[test]
fn insert_duplicate_parameter() {
    let mut router = Router::new();
    let error = router.insert("/<*id>/users/<id>", 3).unwrap_err();
    assert_eq!(
        error,
        InsertError::Template(TemplateError::DuplicateParameter {
            name: "id".to_owned(),
        })
    );

    insta::assert_snapshot!(error, @"duplicate parameter name `id`");
    insta::assert_snapshot!(router, @"");
}
