use std::error::Error;

use wayfind::Router;

#[test]
fn test_display_router() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/", 1)?;
    router.insert("/users", 2)?;
    router.insert("/users/<id>", 3)?;
    router.insert("/users/<id>/profile", 4)?;
    router.insert("/posts/<year>-<month>-<day>", 5)?;
    router.insert("/files/<*path>/download", 6)?;
    router.insert("/<*catch_all>", 7)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ files/
    │  ╰─ <*path>
    │     ╰─ /download
    ├─ posts/
    │  ╰─ <year>
    │     ╰─ -
    │        ╰─ <month>
    │           ╰─ -
    │              ╰─ <day>
    ├─ users
    │  ╰─ /
    │     ╰─ <id>
    │        ╰─ /profile
    ╰─ <*catch_all>
    ");

    Ok(())
}
