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

    insta::assert_snapshot!(router.path, @r"
    / [*]
    ├─ api/v1 [*]
    │  ╰─ / [*]
    ├─ users [*]
    │  ╰─ /
    │     ╰─ {id} [*]
    │        ╰─ /profile [*]
    ├─ images/
    │  ╰─ {name} [*]
    │     ╰─ .
    │        ╰─ {extension} [*]
    ├─ files/
    │  ╰─ {*path}
    │     ╰─ /download [*]
    ├─ posts/
    │  ╰─ {year}
    │     ╰─ -
    │        ╰─ {month}
    │           ╰─ -
    │              ╰─ {day} [*]
    ╰─ {*catch_all} [*]
    ");

    Ok(())
}
