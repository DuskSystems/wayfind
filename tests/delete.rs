use similar_asserts::assert_eq;
use std::error::Error;
use wayfind::{errors::DeleteError, Router};

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
          │     ╰─ / ○
          ╰─ / ○
    "#);

    assert_eq!(
        router.delete("/files/{name}/").unwrap_err(),
        DeleteError::RouteMismatch {
            route: "/files/{name}/".to_string(),
            inserted: "/files/{name}.{extension?}{/}".to_string(),
        }
    );

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /files/
       ╰─ {name} ○
          ├─ .
          │  ╰─ {extension} ○
          │     ╰─ / ○
          ╰─ / ○
    "#);

    router.delete("/files/{name}.{extension?}{/}")?;

    insta::assert_snapshot!(router, @"
    ▽
    ");

    Ok(())
}
