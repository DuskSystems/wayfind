use std::error::Error;

use wayfind::Router;

#[test]
fn test_display_router() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/", 1)?;
    router.insert("/users", 2)?;
    router.insert("/users/{id}", 3)?;
    router.insert("/users/{id}/profile", 4)?;
    router.insert("/posts/{year}-{month}-{day}", 5)?;
    router.insert("/files/{*path}/download", 6)?;
    router.insert("/api/v1(/)", 7)?;
    router.insert("/images/{name}(.{extension})", 8)?;
    router.insert("/{*catch_all}", 9)?;

    insta::assert_snapshot!(router, @r"
    / [*]
    ├─ api/v1 [*]
    │  ╰─ / [*]
    ├─ files/
    │  ╰─ {*path}
    │     ╰─ /download [*]
    ├─ images/
    │  ╰─ {name} [*]
    │     ╰─ .
    │        ╰─ {extension} [*]
    ├─ posts/
    │  ╰─ {year}
    │     ╰─ -
    │        ╰─ {month}
    │           ╰─ -
    │              ╰─ {day} [*]
    ├─ users [*]
    │  ╰─ /
    │     ╰─ {id} [*]
    │        ╰─ /profile [*]
    ╰─ {*catch_all} [*]
    ");

    Ok(())
}
