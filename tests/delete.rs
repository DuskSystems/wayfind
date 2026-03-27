#![expect(missing_docs, reason = "Tests")]

use core::error::Error;

use similar_asserts::assert_eq;
use wayfind::RouterBuilder;
use wayfind::errors::DeleteError;

#[test]
fn delete_static() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/test", 1)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @"/test");

    let mut builder = router.into_builder();
    let error = builder.delete("/tests").unwrap_err();
    assert_eq!(
        error,
        DeleteError::NotFound {
            template: "/tests".to_owned(),
        }
    );

    insta::assert_snapshot!(error, @"template `/tests` not found");

    let router = builder.build();
    insta::assert_snapshot!(router, @"/test");

    let mut builder = router.into_builder();
    let delete = builder.delete("/test")?;
    assert_eq!(delete, 1);

    let router = builder.build();
    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn delete_dynamic() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/users/<id>", 1)?;
    builder.insert("/users/<id>/posts", 2)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
       ╰─ /posts
    ");

    let mut builder = router.into_builder();
    let delete = builder.delete("/users/<id>")?;
    assert_eq!(delete, 1);

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
       ╰─ /posts
    ");

    let mut builder = router.into_builder();
    let delete = builder.delete("/users/<id>/posts")?;
    assert_eq!(delete, 2);

    let router = builder.build();
    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn delete_wildcard() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>/edit", 1)?;
    builder.insert("/<*path>/delete", 2)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /
          ├─ delete
          ╰─ edit
    ");

    let mut builder = router.into_builder();
    let delete = builder.delete("/<*path>/edit")?;
    assert_eq!(delete, 1);

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /delete
    ");

    let mut builder = router.into_builder();
    let delete = builder.delete("/<*path>/delete")?;
    assert_eq!(delete, 2);

    let router = builder.build();
    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn delete_end_wildcard() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/files/<*path>", 1)?;
    builder.insert("/static", 2)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /
    ├─ files/
    │  ╰─ <*path>
    ╰─ static
    ");

    let mut builder = router.into_builder();
    let delete = builder.delete("/files/<*path>")?;
    assert_eq!(delete, 1);

    let router = builder.build();
    insta::assert_snapshot!(router, @"/static");

    Ok(())
}
