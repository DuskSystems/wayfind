use core::error::Error;

use similar_asserts::assert_eq;
use smallvec::smallvec;
use wayfind::{Match, Router};

#[test]
fn dynamic_simple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<id>", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
    ");

    let search = router.search("/123");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<id>",
            parameters: smallvec![("id", "123")],
        })
    );

    let search = router.search("/");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn dynamic_multiple() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<year>", 1)?;
    router.insert("/<year>/<month>", 2)?;
    router.insert("/<year>/<month>/<day>", 3)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <year>
       ╰─ /
          ╰─ <month>
             ╰─ /
                ╰─ <day>
    ");

    let search = router.search("/2024");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<year>",
            parameters: smallvec![("year", "2024")],
        })
    );

    let search = router.search("/2024/12");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<year>/<month>",
            parameters: smallvec![("year", "2024"), ("month", "12")],
        })
    );

    let search = router.search("/2024/12/01");
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            template: "/<year>/<month>/<day>",
            parameters: smallvec![("year", "2024"), ("month", "12"), ("day", "01")],
        })
    );

    Ok(())
}

#[test]
fn dynamic_priority() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/robots.txt", 1)?;
    router.insert("/<name>.txt", 2)?;
    router.insert("/<name>", 3)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ robots.txt
    ╰─ <name>
       ╰─ .txt
    ");

    let search = router.search("/robots.txt");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/robots.txt",
            parameters: smallvec![],
        })
    );

    let search = router.search("/config.txt");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<name>.txt",
            parameters: smallvec![("name", "config")],
        })
    );

    let search = router.search("/config.pdf");
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            template: "/<name>",
            parameters: smallvec![("name", "config.pdf")],
        })
    );

    let search = router.search("/.txt");
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            template: "/<name>",
            parameters: smallvec![("name", ".txt")],
        })
    );

    Ok(())
}

#[test]
fn dynamic_suffix() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<aaa>", 1)?;
    router.insert("/<zzz>.txt", 2)?;

    let search = router.search("/hello.txt");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<zzz>.txt",
            parameters: smallvec![("zzz", "hello")],
        })
    );

    let search = router.search("/hello.pdf");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<aaa>",
            parameters: smallvec![("aaa", "hello.pdf")],
        })
    );

    Ok(())
}

#[test]
fn dynamic_suffix_reversed() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<zzz>", 1)?;
    router.insert("/<aaa>.txt", 2)?;

    let search = router.search("/hello.txt");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<aaa>.txt",
            parameters: smallvec![("aaa", "hello")],
        })
    );

    let search = router.search("/hello.pdf");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<zzz>",
            parameters: smallvec![("zzz", "hello.pdf")],
        })
    );

    Ok(())
}

#[test]
fn dynamic_mixed() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<a>/one", 1)?;
    router.insert("/<b>/two", 2)?;
    router.insert("/<c>.json", 3)?;

    let search = router.search("/hello/one");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<a>/one",
            parameters: smallvec![("a", "hello")],
        })
    );

    let search = router.search("/hello/two");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<b>/two",
            parameters: smallvec![("b", "hello")],
        })
    );

    let search = router.search("/hello.json");
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            template: "/<c>.json",
            parameters: smallvec![("c", "hello")],
        })
    );

    Ok(())
}

#[test]
fn dynamic_sibling() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<a>.txt", 1)?;
    router.insert("/<a>", 2)?;
    router.insert("/<b>.json", 3)?;

    let search = router.search("/hello.txt");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<a>.txt",
            parameters: smallvec![("a", "hello")],
        })
    );

    let search = router.search("/hello.json");
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            template: "/<b>.json",
            parameters: smallvec![("b", "hello")],
        })
    );

    let search = router.search("/hello.pdf");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<a>",
            parameters: smallvec![("a", "hello.pdf")],
        })
    );

    Ok(())
}

#[test]
fn dynamic_inline_params() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<id>.<format>", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <id>
       ╰─ .
          ╰─ <format>
    ");

    let search = router.search("/archive.tar.gz");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<id>.<format>",
            parameters: smallvec![("id", "archive.tar"), ("format", "gz")],
        })
    );

    let search = router.search("/report.pdf");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<id>.<format>",
            parameters: smallvec![("id", "report"), ("format", "pdf")],
        })
    );

    let search = router.search("/nodots");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn dynamic_triple_params() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<a>.<b>.<c>", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <a>
       ╰─ .
          ╰─ <b>
             ╰─ .
                ╰─ <c>
    ");

    let search = router.search("/x.y.z");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<a>.<b>.<c>",
            parameters: smallvec![("a", "x"), ("b", "y"), ("c", "z")],
        })
    );

    let search = router.search("/a.b.c.d");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<a>.<b>.<c>",
            parameters: smallvec![("a", "a.b"), ("b", "c"), ("c", "d")],
        })
    );

    let search = router.search("/one.two.");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn dynamic_multi_separator() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<from>...<to>", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <from>
       ╰─ ...
          ╰─ <to>
    ");

    let search = router.search("/10...100");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<from>...<to>",
            parameters: smallvec![("from", "10"), ("to", "100")],
        })
    );

    let search = router.search("/abc...xyz");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<from>...<to>",
            parameters: smallvec![("from", "abc"), ("to", "xyz")],
        })
    );

    let search = router.search("/nodots");
    assert_eq!(search, None);

    Ok(())
}

#[test]
fn dynamic_inline_coexistence() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<id>.<format>", 1)?;
    router.insert("/<name>.txt", 2)?;

    let search = router.search("/report.txt");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<name>.txt",
            parameters: smallvec![("name", "report")],
        })
    );

    let search = router.search("/report.pdf");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<id>.<format>",
            parameters: smallvec![("id", "report"), ("format", "pdf")],
        })
    );

    let search = router.search("/archive.tar.gz");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<id>.<format>",
            parameters: smallvec![("id", "archive.tar"), ("format", "gz")],
        })
    );

    Ok(())
}
