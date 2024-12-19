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
    ╰─ {id} [1]
       ╰─ /
          ├─ settings [3]
          ╰─ profile [2]
    === Chains
    1
    2
    3
    ");

    let route = RouteBuilder::new().route("/users/{id}/profile").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ╰─ {id} [1]
       ╰─ /settings [3]
    === Chains
    1
    3
    ");

    let route = RouteBuilder::new().route("/users/{id}/settings").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ╰─ {id} [1]
    === Chains
    1
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
    ╰─ {id} [1]
       ╰─ /
          ├─ settings [3]
          ╰─ profile [2]
    === Chains
    1
    2
    3
    ");

    let route = RouteBuilder::new().route("/users/{id}").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ╰─ {id}
       ╰─ /
          ├─ settings [3]
          ╰─ profile [2]
    === Chains
    2
    3
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
    /a [2]
    ╰─ b [3]
       ╰─ c [1]
    === Chains
    1
    2
    3
    ");

    let route = RouteBuilder::new().route("/ab").build()?;
    router.delete(&route)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /a [2]
    ╰─ bc [1]
    === Chains
    1
    2
    ");

    Ok(())
}
