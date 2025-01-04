use std::error::Error;

use wayfind::Router;

#[test]
fn test_optimize_removal() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users/{id}", 1)?;
    router.insert("/users/{id}/profile", 2)?;
    router.insert("/users/{id}/settings", 3)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {id} [*]
       ╰─ /
          ├─ settings [*]
          ╰─ profile [*]
    ");

    router.delete("/users/{id}/profile")?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {id} [*]
       ╰─ /settings [*]
    ");

    router.delete("/users/{id}/settings")?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {id} [*]
    ");

    Ok(())
}

#[test]
fn test_optimize_data() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users/{id}", 1)?;
    router.insert("/users/{id}/profile", 2)?;
    router.insert("/users/{id}/settings", 3)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {id} [*]
       ╰─ /
          ├─ settings [*]
          ╰─ profile [*]
    ");

    router.delete("/users/{id}")?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {id}
       ╰─ /
          ├─ settings [*]
          ╰─ profile [*]
    ");

    Ok(())
}

#[test]
fn test_optimize_compression() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/abc", 1)?;
    router.insert("/a", 2)?;
    router.insert("/ab", 3)?;

    insta::assert_snapshot!(router, @r"
    /a [*]
    ╰─ b [*]
       ╰─ c [*]
    ");

    router.delete("/ab")?;

    insta::assert_snapshot!(router, @r"
    /a [*]
    ╰─ bc [*]
    ");

    Ok(())
}
