#![expect(missing_docs, reason = "Tests")]

use core::error::Error;

use wayfind::RouterBuilder;

#[test]
fn optimize_removal() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/users/<id>", 1)?;
    builder.insert("/users/<id>/profile", 2)?;
    builder.insert("/users/<id>/settings", 3)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
       ╰─ /
          ├─ profile
          ╰─ settings
    ");

    let mut builder = router.into_builder();
    builder.delete("/users/<id>/profile")?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
       ╰─ /settings
    ");

    let mut builder = router.into_builder();
    builder.delete("/users/<id>/settings")?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
    ");

    Ok(())
}

#[test]
fn optimize_data() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/users/<id>", 1)?;
    builder.insert("/users/<id>/profile", 2)?;
    builder.insert("/users/<id>/settings", 3)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
       ╰─ /
          ├─ profile
          ╰─ settings
    ");

    let mut builder = router.into_builder();
    builder.delete("/users/<id>")?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
       ╰─ /
          ├─ profile
          ╰─ settings
    ");

    Ok(())
}

#[test]
fn optimize_compression() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/abc", 1)?;
    builder.insert("/a", 2)?;
    builder.insert("/ab", 3)?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /a
    ╰─ b
       ╰─ c
    ");

    let mut builder = router.into_builder();
    builder.delete("/ab")?;

    let router = builder.build();
    insta::assert_snapshot!(router, @r"
    /a
    ╰─ bc
    ");

    Ok(())
}
