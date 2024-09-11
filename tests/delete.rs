use std::error::Error;
use wayfind::Router;

#[test]
fn expanded() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/files/{name}.{extension?}{/}", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /files/
             ╰─ {name} ○
                     ├─ .
                     │  ╰─ {extension} ○
                     │               ╰─ / ○
                     ╰─ / ○
    "#);

    // Should not be able to delete via expande routes.
    router.delete("/files/{name}/")?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /files/
             ╰─ {name} ○
                     ╰─ .
                        ╰─ {extension} ○
                                     ╰─ / ○
    "#);

    // Should be able to delete via pre-expanded route.
    router.delete("/files/{name}.{extension?}{/}")?;

    insta::assert_snapshot!(router, @"");

    Ok(())
}
