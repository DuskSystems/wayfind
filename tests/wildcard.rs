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
fn wildcard_empty() -> Result<(), Box<dyn Error>> {
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
fn wildcard_greedy() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*path>/edit/<*rest>", 1)?;
    router.insert("/<*path>/delete", 2)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*path>
       ╰─ /
          ├─ delete
          ╰─ edit/
             ╰─ <*rest>
    ");

    let search = router.search("/documents/delete");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<*path>/delete",
            parameters: smallvec![("path", "documents")],
        })
    );

    let search = router.search("/documents/edit/summary");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*path>/edit/<*rest>",
            parameters: smallvec![("path", "documents"), ("rest", "summary")],
        })
    );

    let search = router.search("/docs/edit/readme/delete");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<*path>/delete",
            parameters: smallvec![("path", "docs/edit/readme")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_dynamic() -> Result<(), Box<dyn Error>> {
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

    // NOTE: Should arguably match `/<*path>/help`.
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

#[test]
fn wildcard_suffix() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*aaa>", 1)?;
    router.insert("/<*zzz>.txt", 2)?;

    let search = router.search("/hello.txt");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<*zzz>.txt",
            parameters: smallvec![("zzz", "hello")],
        })
    );

    let search = router.search("/hello.pdf");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*aaa>",
            parameters: smallvec![("aaa", "hello.pdf")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_fallthrough() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<username>", 1)?;
    router.insert("/<*namespace_id>/<project_id>/-/issues", 2)?;

    let search = router.search("/johndoe");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<username>",
            parameters: smallvec![("username", "johndoe")],
        })
    );

    let search = router.search("/gitlab-org/gitlab/-/issues");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<*namespace_id>/<project_id>/-/issues",
            parameters: smallvec![("namespace_id", "gitlab-org"), ("project_id", "gitlab")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_namespace() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*namespace_id>/<project_id>/-/merge_requests", 1)?;

    let search = router.search("/gitlab-org/frontend/gitlab-ui/-/merge_requests");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*namespace_id>/<project_id>/-/merge_requests",
            parameters: smallvec![
                ("namespace_id", "gitlab-org/frontend"),
                ("project_id", "gitlab-ui"),
            ],
        })
    );

    Ok(())
}

#[test]
fn wildcard_catchall() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<username>", 1)?;
    router.insert("/<username>/<*rest>", 2)?;
    router.insert("/<*namespace_id>/<project_id>/-/issues", 3)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ <username>
    │  ╰─ /
    │     ╰─ <*rest>
    ╰─ <*namespace_id>
       ╰─ /
          ╰─ <project_id>
             ╰─ /-/issues
    ");

    // NOTE: Should arguably match `/<*namespace_id>/<project_id>/-/issues`.
    let search = router.search("/gitlab-org/gitlab/-/issues");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<username>/<*rest>",
            parameters: smallvec![("username", "gitlab-org"), ("rest", "gitlab/-/issues")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_miss_fallback() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/pipelines/<id>", 1)?;
    router.insert("/pipelines/<id>/security", 2)?;
    router.insert("/pipelines/<*ref>/latest", 3)?;

    let search = router.search("/pipelines/main/latest");
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            template: "/pipelines/<*ref>/latest",
            parameters: smallvec![("ref", "main")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_multi_segment() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/pipelines/<*ref>/latest", 1)?;

    let search = router.search("/pipelines/feature/auth/latest");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/pipelines/<*ref>/latest",
            parameters: smallvec![("ref", "feature/auth")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_nested() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*a>/<b>/-/issues", 1)?;

    let search = router.search("/a/b/c/my-project/-/issues");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*a>/<b>/-/issues",
            parameters: smallvec![("a", "a/b/c"), ("b", "my-project")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_branches() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*a>/<b>/-/wikis/<c>", 1)?;
    router.insert("/<*a>/-/settings", 2)?;

    let search = router.search("/gitlab-org/gitlab/-/wikis/home");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*a>/<b>/-/wikis/<c>",
            parameters: smallvec![("a", "gitlab-org"), ("b", "gitlab"), ("c", "home")],
        })
    );

    let search = router.search("/gitlab-org/-/settings");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<*a>/-/settings",
            parameters: smallvec![("a", "gitlab-org")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_ambiguous() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*a>/<b>/-/issues", 1)?;

    let search = router.search("/a/-/issues/real-project/-/issues");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*a>/<b>/-/issues",
            parameters: smallvec![("a", "a/-/issues"), ("b", "real-project")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_chain() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*a>/middle/<*b>/end", 1)?;

    let search = router.search("/x/y/middle/w/v/end");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*a>/middle/<*b>/end",
            parameters: smallvec![("a", "x/y"), ("b", "w/v")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_multibyte() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*path>.html", 1)?;

    let search = router.search("/docs/café.html");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*path>.html",
            parameters: smallvec![("path", "docs/café")],
        })
    );

    let search = router.search("/日本語/ページ.html");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*path>.html",
            parameters: smallvec![("path", "日本語/ページ")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_repeated_anchor() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*x>/-/<*y>/end", 1)?;

    let search = router.search("/a/-/b/-/c/-/d/-/end");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*x>/-/<*y>/end",
            parameters: smallvec![("x", "a/-/b/-/c"), ("y", "d/-")],
        })
    );

    let search = router.search("/a/-/b/-/c/-/d/-/miss");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn wildcard_endings() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*a>/x", 1)?;
    router.insert("/<*a>/y", 2)?;
    router.insert("/<*a>/z", 3)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <*a>
       ╰─ /
          ├─ x
          ├─ y
          ╰─ z
    ");

    let search = router.search("/one/x");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*a>/x",
            parameters: smallvec![("a", "one")],
        })
    );

    let search = router.search("/one/two/three/y");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<*a>/y",
            parameters: smallvec![("a", "one/two/three")],
        })
    );

    let search = router.search("/one/z");
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            template: "/<*a>/z",
            parameters: smallvec![("a", "one")],
        })
    );

    let search = router.search("/one/miss");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn wildcard_anchored() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*a>/-/<*b>/x", 1)?;
    router.insert("/<*a>/-/<*b>/y", 2)?;
    router.insert("/<*a>/-/<*b>/z", 3)?;

    let search = router.search("/one/two/-/three/x");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*a>/-/<*b>/x",
            parameters: smallvec![("a", "one/two"), ("b", "three")],
        })
    );

    let search = router.search("/one/-/two/y");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<*a>/-/<*b>/y",
            parameters: smallvec![("a", "one"), ("b", "two")],
        })
    );

    let search = router.search("/one/-/two/miss");
    assert_eq!(search, None);

    let search = router.search("/-/x/-/x/-/x/miss");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn wildcard_repeated_suffix() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*path>.txt", 1)?;
    router.insert("/<*path>", 2)?;

    let search = router.search("/a.txt.txt.txt.txt");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<*path>.txt",
            parameters: smallvec![("path", "a.txt.txt.txt")],
        })
    );

    let search = router.search("/.txt");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<*path>",
            parameters: smallvec![("path", ".txt")],
        })
    );

    Ok(())
}

#[test]
fn wildcard_segment_empty() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*a>/<b>", 1)?;

    let search = router.search("//foo");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn wildcard_inline_empty() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*path>.x/<*rest>", 1)?;

    let search = router.search("/.x/y");
    assert_eq!(search, None);

    Ok(())
}
