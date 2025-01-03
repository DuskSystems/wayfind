use std::error::Error;
use wayfind::{RouteBuilder, Router};

#[test]
fn test_display_router() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/users").build()?;
    router.insert(&route, 2)?;
    let route = RouteBuilder::new().route("/users/{id}").build()?;
    router.insert(&route, 3)?;
    let route = RouteBuilder::new().route("/users/{id}/profile").build()?;
    router.insert(&route, 4)?;
    let route = RouteBuilder::new()
        .route("/posts/{year}-{month}-{day}")
        .build()?;
    router.insert(&route, 5)?;
    let route = RouteBuilder::new()
        .route("/files/{*path}/download")
        .build()?;
    router.insert(&route, 6)?;
    let route = RouteBuilder::new().route("/api/v1(/)").build()?;
    router.insert(&route, 7)?;
    let route = RouteBuilder::new()
        .route("/images/{name}(.{extension})")
        .build()?;
    router.insert(&route, 8)?;
    let route = RouteBuilder::new().route("/{*catch_all}").build()?;
    router.insert(&route, 9)?;

    insta::assert_snapshot!(router, @r"
    === Authority
    Empty
    === Path
    / [1]
    ├─ api/v1 [7]
    │  ╰─ / [7]
    ├─ users [2]
    │  ╰─ /
    │     ╰─ {id} [3]
    │        ╰─ /profile [4]
    ├─ images/
    │  ╰─ {name} [8]
    │     ╰─ .
    │        ╰─ {extension} [8]
    ├─ files/
    │  ╰─ {*path}
    │     ╰─ /download [6]
    ├─ posts/
    │  ╰─ {year}
    │     ╰─ -
    │        ╰─ {month}
    │           ╰─ -
    │              ╰─ {day} [5]
    ╰─ {*catch_all} [9]
    === Method
    Empty
    === Chains
    *-1-*
    *-2-*
    *-3-*
    *-4-*
    *-5-*
    *-6-*
    *-7-*
    *-8-*
    *-9-*
    ");

    Ok(())
}
