use std::error::Error;

use similar_asserts::assert_eq;
use wayfind::{Router, errors::DeleteError};

#[test]
fn test_delete() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/test", 1)?;

    insta::assert_snapshot!(router, @"/test [*]");

    let delete = router.delete("/tests");
    assert_eq!(
        delete,
        Err(DeleteError::NotFound {
            template: "/tests".to_owned()
        })
    );

    insta::assert_snapshot!(router, @"/test [*]");

    let delete = router.delete("(/test)");
    assert_eq!(
        delete,
        Err(DeleteError::Mismatch {
            template: "(/test)".to_owned(),
            inserted: "/test".to_owned(),
        })
    );

    insta::assert_snapshot!(router, @"/test [*]");

    let delete = router.delete("/test")?;
    assert_eq!(delete, 1);

    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn test_delete_mismatch() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("(/test)", 1)?;

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ test [*]
    ");

    let delete = router.delete("/test");
    assert_eq!(
        delete,
        Err(DeleteError::Mismatch {
            template: "/test".to_owned(),
            inserted: "(/test)".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ test [*]
    ");

    let delete = router.delete("/");
    assert_eq!(
        delete,
        Err(DeleteError::Mismatch {
            template: "/".to_owned(),
            inserted: "(/test)".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ test [*]
    ");

    let delete = router.delete("(/test)")?;
    assert_eq!(delete, 1);

    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn test_delete_overlap() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/a(/b)", 1)?;

    insta::assert_snapshot!(router, @r"
    /a [*]
    ╰─ /b [*]
    ");

    let delete = router.delete("/a");
    assert_eq!(
        delete,
        Err(DeleteError::Mismatch {
            template: "/a".to_owned(),
            inserted: "/a(/b)".to_owned(),
        })
    );

    let delete = router.delete("/a(/b(/c))");
    assert_eq!(
        delete,
        Err(DeleteError::Mismatch {
            template: "/a(/b(/c))".to_owned(),
            inserted: "/a(/b)".to_owned(),
        })
    );

    let delete = router.delete("/a(/b)")?;
    assert_eq!(delete, 1);

    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn test_delete_empty() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<id>data", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
       ╰─ data [*]
    ");

    let delete = router.delete("/<id>");
    assert_eq!(
        delete,
        Err(DeleteError::NotFound {
            template: "/<id>".to_owned()
        })
    );

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
       ╰─ data [*]
    ");

    Ok(())
}
