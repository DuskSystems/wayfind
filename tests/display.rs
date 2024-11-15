use std::error::Error;
use wayfind::{RoutableBuilder, Router};

#[test]
fn test_display_router() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/").build()?;
    router.insert(&route, 1)?;
    let route = RoutableBuilder::new().route("/users").build()?;
    router.insert(&route, 2)?;
    let route = RoutableBuilder::new().route("/users/{id}").build()?;
    router.insert(&route, 3)?;
    let route = RoutableBuilder::new()
        .route("/users/{id}/profile")
        .build()?;
    router.insert(&route, 4)?;
    let route = RoutableBuilder::new()
        .route("/posts/{year}-{month}-{day}")
        .build()?;
    router.insert(&route, 5)?;
    let route = RoutableBuilder::new()
        .route("/files/{*path}/download")
        .build()?;
    router.insert(&route, 6)?;
    let route = RoutableBuilder::new().route("/api/v1(/)").build()?;
    router.insert(&route, 7)?;
    let route = RoutableBuilder::new()
        .route("/images/{name}(.{extension})")
        .build()?;
    router.insert(&route, 8)?;
    let route = RoutableBuilder::new().route("/{*catch_all}").build()?;
    router.insert(&route, 9)?;

    insta::assert_snapshot!(router, @r"
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
