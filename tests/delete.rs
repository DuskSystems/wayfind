use core::error::Error;

use similar_asserts::assert_eq;
use wayfind::Router;
use wayfind::errors::DeleteError;

#[test]
fn delete_static() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/test", 1)?;

    insta::assert_snapshot!(router, @"/test");

    let error = router.delete("/tests").unwrap_err();
    assert_eq!(
        error,
        DeleteError::NotFound {
            template: "/tests".to_owned()
        }
    );

    insta::assert_snapshot!(error, @"template not found: `/tests`");
    insta::assert_debug_snapshot!(error, @r"
    error: template not found

        /tests
        ━━━━━━

    help: template does not exist in the router
    ");

    insta::assert_snapshot!(router, @"/test");

    let delete = router.delete("/test")?;
    assert_eq!(delete, 1);

    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn delete_dynamic() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users/<id>", 1)?;
    router.insert("/users/<id>/posts", 2)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
       ╰─ /posts
    ");

    let delete = router.delete("/users/<id>")?;
    assert_eq!(delete, 1);

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
       ╰─ /posts
    ");

    let delete = router.delete("/users/<id>/posts")?;
    assert_eq!(delete, 2);

    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn delete_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*path>/edit", 1)?;
    router.insert("/<*path>/delete", 2)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /
          ├─ delete
          ╰─ edit
    ");

    let delete = router.delete("/<*path>/edit")?;
    assert_eq!(delete, 1);

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /delete
    ");

    let delete = router.delete("/<*path>/delete")?;
    assert_eq!(delete, 2);

    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn delete_end_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/files/<*path>", 1)?;
    router.insert("/static", 2)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ files/
    │  ╰─ <*path>
    ╰─ static
    ");

    let delete = router.delete("/files/<*path>")?;
    assert_eq!(delete, 1);

    insta::assert_snapshot!(router, @"/static");

    Ok(())
}
