use core::error::Error;

use wayfind::Router;

#[test]
fn optimize_removal() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users/<id>", 1)?;
    router.insert("/users/<id>/profile", 2)?;
    router.insert("/users/<id>/settings", 3)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
       ╰─ /
          ├─ profile
          ╰─ settings
    ");

    router.delete("/users/<id>/profile")?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
       ╰─ /settings
    ");

    router.delete("/users/<id>/settings")?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
    ");

    Ok(())
}

#[test]
fn optimize_data() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users/<id>", 1)?;
    router.insert("/users/<id>/profile", 2)?;
    router.insert("/users/<id>/settings", 3)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ <id>
       ╰─ /
          ├─ profile
          ╰─ settings
    ");

    router.delete("/users/<id>")?;

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
    let mut router = Router::new();
    router.insert("/abc", 1)?;
    router.insert("/a", 2)?;
    router.insert("/ab", 3)?;

    insta::assert_snapshot!(router, @r"
    /a
    ╰─ b
       ╰─ c
    ");

    router.delete("/ab")?;

    insta::assert_snapshot!(router, @r"
    /a
    ╰─ bc
    ");

    Ok(())
}
