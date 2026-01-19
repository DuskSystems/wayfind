use core::error::Error;

use wayfind::Router;

#[test]
fn display_router() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/", 1)?;
    router.insert("/users", 2)?;
    router.insert("/users/<id>", 3)?;
    router.insert("/posts/<id>.json", 4)?;
    router.insert("/users/<id>/profile", 5)?;
    router.insert("/files/<*path>/download", 6)?;
    router.insert("/<*catch_all>", 7)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ files/
    │  ╰─ <*path>
    │     ╰─ /download
    ├─ posts/
    │  ╰─ <id>
    │     ╰─ .json
    ├─ users
    │  ╰─ /
    │     ╰─ <id>
    │        ╰─ /profile
    ╰─ <*catch_all>
    ");

    Ok(())
}
