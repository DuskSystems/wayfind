use std::error::Error;
use wayfind::{errors::DeleteError, Router};

#[test]
fn test_delete() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/test", 1)?;

    insta::assert_snapshot!(router, @"/test [*]");

    let delete = router.delete("/tests");
    assert_eq!(
        delete,
        Err(DeleteError::NotFound {
            route: "/tests".to_string()
        })
    );

    insta::assert_snapshot!(router, @"/test [*]");

    let delete = router.delete("(/test)");
    assert_eq!(
        delete,
        Err(DeleteError::NotFound {
            route: "(/test)".to_string(),
        })
    );

    insta::assert_snapshot!(router, @"/test [*]");

    router.delete("/test")?;
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
        Err(DeleteError::RouteMismatch {
            route: "/test".to_string(),
            inserted: "(/test)".to_string()
        })
    );

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ test [*]
    ");

    let delete = router.delete("/");
    assert_eq!(
        delete,
        Err(DeleteError::RouteMismatch {
            route: "/".to_string(),
            inserted: "(/test)".to_string()
        })
    );

    insta::assert_snapshot!(router, @r"
    / [*]
    ╰─ test [*]
    ");

    router.delete("(/test)")?;
    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn test_delete_empty() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{id}data", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {id}
       ╰─ data [*]
    ");

    let delete = router.delete("/{id}");
    assert_eq!(
        delete,
        Err(DeleteError::NotFound {
            route: "/{id}".to_string()
        })
    );

    insta::assert_snapshot!(router, @r"
    /
    ╰─ {id}
       ╰─ data [*]
    ");

    Ok(())
}
