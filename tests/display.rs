#![expect(missing_docs, reason = "Tests")]

use core::error::Error;

use wayfind::RouterBuilder;

#[test]
fn display_router() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/", 1)?;
    builder.insert("/users", 2)?;
    builder.insert("/users/<id>", 3)?;
    builder.insert("/posts/<id>.json", 4)?;
    builder.insert("/users/<id>/profile", 5)?;
    builder.insert("/files/<*path>/download", 6)?;
    builder.insert("/<*catch_all>", 7)?;

    let router = builder.build();
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
