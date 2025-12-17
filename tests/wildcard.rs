use core::error::Error;

use similar_asserts::assert_eq;
use smallvec::smallvec;
use wayfind::{Match, Router};

#[test]
fn wildcard_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*path>/delete", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /delete
    ");

    let search = router.search("/docs/delete");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*path>/delete",
            parameters: smallvec![("path", "docs")],
        })
    );

    let search = router.search("/nested/docs/folder/delete");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*path>/delete",
            parameters: smallvec![("path", "nested/docs/folder")],
        })
    );

    let search = router.search("/delete");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn wildcard_multiple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*prefix>/static/<*suffix>/file", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*prefix>
       ╰─ /static/
          ╰─ <*suffix>
             ╰─ /file
    ");

    let search = router.search("/a/static/b/file");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*prefix>/static/<*suffix>/file",
            parameters: smallvec![("prefix", "a"), ("suffix", "b")],
        })
    );

    let search = router.search("/a/b/c/static/d/e/f/file");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*prefix>/static/<*suffix>/file",
            parameters: smallvec![("prefix", "a/b/c"), ("suffix", "d/e/f")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_inline() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*path>.html", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ .html
    ");

    let search = router.search("/page.html");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*path>.html",
            parameters: smallvec![("path", "page")],
        })
    );

    let search = router.search("/nested/page.html");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*path>.html",
            parameters: smallvec![("path", "nested/page")],
        })
    );

    let search = router.search("/.html");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn wildcard_greedy() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*first>-<*second>", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*first>
       ╰─ -
          ╰─ <*second>
    ");

    let search = router.search("/a-b-c");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*first>-<*second>",
            parameters: smallvec![("first", "a-b"), ("second", "c")],
        })
    );

    let search = router.search("/path/to/some-file/with-multiple-hyphens");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*first>-<*second>",
            parameters: smallvec![
                ("first", "path/to/some-file/with-multiple"),
                ("second", "hyphens")
            ],
        })
    );

    Ok(())
}

#[test]
fn wildcard_empty_segments() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*path>/end", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /end
    ");

    let search = router.search("/start/middle/end");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*path>/end",
            parameters: smallvec![("path", "start/middle")],
        })
    );

    let search = router.search("/start//middle///end");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*path>/end",
            parameters: smallvec![("path", "start//middle//")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_priority() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/static/path", 1)?;
    router.insert("/static/<*rest>", 2)?;
    router.insert("/<*path>/static", 3)?;
    router.insert("/prefix.<*suffix>", 4)?;
    router.insert("/<*prefix>.suffix", 5)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ prefix.
    │  ╰─ <*suffix>
    ├─ static/
    │  ├─ path
    │  ╰─ <*rest>
    ├─ <*path>
    │  ╰─ /static
    ╰─ <*prefix>
       ╰─ .suffix
    ");

    let search = router.search("/static/path");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/static/path",
            parameters: smallvec![],
        })
    );

    let search = router.search("/static/some/nested/path");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/static/<*rest>",
            parameters: smallvec![("rest", "some/nested/path")],
        })
    );

    let search = router.search("/some/nested/path/static");
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            template: "/<*path>/static",
            parameters: smallvec![("path", "some/nested/path")],
        })
    );

    let search = router.search("/prefix.some/nested/path");
    assert_eq!(
        search,
        Some(Match {
            data: &4,
            template: "/prefix.<*suffix>",
            parameters: smallvec![("suffix", "some/nested/path")],
        })
    );

    let search = router.search("/some/nested/path.suffix");
    assert_eq!(
        search,
        Some(Match {
            data: &5,
            template: "/<*prefix>.suffix",
            parameters: smallvec![("prefix", "some/nested/path")],
        })
    );

    Ok(())
}

#[test]
fn dynamic_wildcard_priority() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/api/<version>/<*rest>", 1)?;
    router.insert("/api/<*path>/help", 2)?;

    insta::assert_snapshot!(router, @r"
    /api/
    ├─ <version>
    │  ╰─ /
    │     ╰─ <*rest>
    ╰─ <*path>
       ╰─ /help
    ");

    // NOTE: This appears to match the 2nd template closer.
    // But since we always prefer dynamic over wildcard, it doesn't.
    // One to consider for the future.
    let search = router.search("/api/docs/reference/help");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/api/<version>/<*rest>",
            parameters: smallvec![("version", "docs"), ("rest", "reference/help")],
        })
    );

    Ok(())
}
