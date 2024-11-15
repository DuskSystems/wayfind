use std::error::Error;
use wayfind::{RoutableBuilder, Router};

#[test]
fn test_optimize_removal() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/users/{id}").build()?;
    router.insert(&route, 1)?;
    let route = RoutableBuilder::new()
        .route("/users/{id}/profile")
        .build()?;
    router.insert(&route, 2)?;
    let route = RoutableBuilder::new()
        .route("/users/{id}/settings")
        .build()?;
    router.insert(&route, 3)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {id} [*]
       ╰─ /
          ├─ settings [*]
          ╰─ profile [*]
    ");

    let route = RoutableBuilder::new()
        .route("/users/{id}/profile")
        .build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {id} [*]
       ╰─ /settings [*]
    ");

    let route = RoutableBuilder::new()
        .route("/users/{id}/settings")
        .build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {id} [*]
    ");

    Ok(())
}

#[test]
fn test_optimize_data() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/users/{id}").build()?;
    router.insert(&route, 1)?;
    let route = RoutableBuilder::new()
        .route("/users/{id}/profile")
        .build()?;
    router.insert(&route, 2)?;
    let route = RoutableBuilder::new()
        .route("/users/{id}/settings")
        .build()?;
    router.insert(&route, 3)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {id} [*]
       ╰─ /
          ├─ settings [*]
          ╰─ profile [*]
    ");

    let route = RoutableBuilder::new().route("/users/{id}").build()?;
    router.delete(&route)?;

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

    let route = RoutableBuilder::new().route("/abc").build()?;
    router.insert(&route, 1)?;
    let route = RoutableBuilder::new().route("/a").build()?;
    router.insert(&route, 2)?;
    let route = RoutableBuilder::new().route("/ab").build()?;
    router.insert(&route, 3)?;

    insta::assert_snapshot!(router, @r"
    /a [*]
    ╰─ b [*]
       ╰─ c [*]
    ");

    let route = RoutableBuilder::new().route("/ab").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    /a [*]
    ╰─ bc [*]
    ");

    Ok(())
}
