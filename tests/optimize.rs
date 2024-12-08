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

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ╰─ {id} [0]
       ╰─ /
          ├─ settings [2]
          ╰─ profile [1]
    === Method
    ");

    let route = RouteBuilder::new().route("/users/{id}/profile").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ╰─ {id} [0]
       ╰─ /settings [2]
    === Method
    ");

    let route = RouteBuilder::new().route("/users/{id}/settings").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ╰─ {id} [0]
    === Method
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

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ╰─ {id} [0]
       ╰─ /
          ├─ settings [2]
          ╰─ profile [1]
    === Method
    ");

    let route = RouteBuilder::new().route("/users/{id}").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ╰─ {id}
       ╰─ /
          ├─ settings [2]
          ╰─ profile [1]
    === Method
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

    insta::assert_snapshot!(router, @r"
    === Path
    /a [1]
    ╰─ b [2]
       ╰─ c [0]
    === Method
    ");

    let route = RouteBuilder::new().route("/ab").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /a [1]
    ╰─ bc [0]
    === Method
    ");

    Ok(())
}
