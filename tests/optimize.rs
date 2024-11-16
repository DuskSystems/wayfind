use std::error::Error;
use wayfind::{RouteBuilder, Router};

#[test]
fn test_optimize_removal() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users/{id}").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/users/{id}/profile").build()?;
    router.insert(&route, 2)?;
    let route = RouteBuilder::new().route("/users/{id}/settings").build()?;
    router.insert(&route, 3)?;

    insta::assert_snapshot!(router.path, @r"
    /users/
    ╰─ {id} [*]
       ╰─ /
          ├─ settings [*]
          ╰─ profile [*]
    ");

    let route = RouteBuilder::new().route("/users/{id}/profile").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router.path, @r"
    /users/
    ╰─ {id} [*]
       ╰─ /settings [*]
    ");

    let route = RouteBuilder::new().route("/users/{id}/settings").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router.path, @r"
    /users/
    ╰─ {id} [*]
    ");

    Ok(())
}

#[test]
fn test_optimize_data() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users/{id}").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/users/{id}/profile").build()?;
    router.insert(&route, 2)?;
    let route = RouteBuilder::new().route("/users/{id}/settings").build()?;
    router.insert(&route, 3)?;

    insta::assert_snapshot!(router.path, @r"
    /users/
    ╰─ {id} [*]
       ╰─ /
          ├─ settings [*]
          ╰─ profile [*]
    ");

    let route = RouteBuilder::new().route("/users/{id}").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router.path, @r"
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

    let route = RouteBuilder::new().route("/abc").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/a").build()?;
    router.insert(&route, 2)?;
    let route = RouteBuilder::new().route("/ab").build()?;
    router.insert(&route, 3)?;

    insta::assert_snapshot!(router.path, @r"
    /a [*]
    ╰─ b [*]
       ╰─ c [*]
    ");

    let route = RouteBuilder::new().route("/ab").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router.path, @r"
    /a [*]
    ╰─ bc [*]
    ");

    Ok(())
}
