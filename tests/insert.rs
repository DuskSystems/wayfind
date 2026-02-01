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
            template: "/test".to_owned(),
            existing: "/test".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict: `/test` conflicts with `/test`");
    insta::assert_debug_snapshot!(error, @r"
    error: conflict detected

        /test
        ━━━━━ conflicts with `/test`

    help: templates cannot overlap with existing routes
    ");

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
            template: "/<id>".to_owned(),
            existing: "/<id>".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict: `/<id>` conflicts with `/<id>`");
    insta::assert_debug_snapshot!(error, @r"
    error: conflict detected

        /<id>
        ━━━━━ conflicts with `/<id>`

    help: templates cannot overlap with existing routes
    ");

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
            template: "/<user>".to_owned(),
            existing: "/<id>".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict: `/<user>` conflicts with `/<id>`");
    insta::assert_debug_snapshot!(error, @r"
    error: conflict detected

        /<user>
        ━━━━━━━ conflicts with `/<id>`

    help: templates cannot overlap with existing routes
    ");

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
            template: "/<*catch_all>".to_owned(),
            existing: "/<*catch_all>".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict: `/<*catch_all>` conflicts with `/<*catch_all>`");
    insta::assert_debug_snapshot!(error, @r"
    error: conflict detected

        /<*catch_all>
        ━━━━━━━━━━━━━ conflicts with `/<*catch_all>`

    help: templates cannot overlap with existing routes
    ");

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
            template: "/<*files>".to_owned(),
            existing: "/<*catch_all>".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict: `/<*files>` conflicts with `/<*catch_all>`");
    insta::assert_debug_snapshot!(error, @r"
    error: conflict detected

        /<*files>
        ━━━━━━━━━ conflicts with `/<*catch_all>`

    help: templates cannot overlap with existing routes
    ");

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
            template: "/<*catch_all>".to_owned(),
            existing: "/<*catch_all>".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict: `/<*catch_all>` conflicts with `/<*catch_all>`");
    insta::assert_debug_snapshot!(error, @r"
    error: conflict detected

        /<*catch_all>
        ━━━━━━━━━━━━━ conflicts with `/<*catch_all>`

    help: templates cannot overlap with existing routes
    ");

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
            template: "/<*files>".to_owned(),
            existing: "/<*catch_all>".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"conflict: `/<*files>` conflicts with `/<*catch_all>`");
    insta::assert_debug_snapshot!(error, @r"
    error: conflict detected

        /<*files>
        ━━━━━━━━━ conflicts with `/<*catch_all>`

    help: templates cannot overlap with existing routes
    ");

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
            template: "/<*id>/users/<id>".to_owned(),
            name: "id".to_owned(),
            original: 1..6,
            duplicate: 13..17,
        })
    );

    insta::assert_snapshot!(error, @"duplicate parameter name `id` in `/<*id>/users/<id>`");
    insta::assert_debug_snapshot!(error, @r"
    error: duplicate parameter name: `id`

        /<*id>/users/<id>
         ━━━━━       ━━━━

    help: rename one of the parameters
    ");

    insta::assert_snapshot!(router, @"");
}
