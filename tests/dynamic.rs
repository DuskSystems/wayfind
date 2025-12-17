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
fn dynamic_inline() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<year>", 1)?;
    router.insert("/<year>-<month>", 2)?;
    router.insert("/<year>-<month>-<day>", 3)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <year>
       ╰─ -
          ╰─ <month>
             ╰─ -
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

    let search = router.search("/2024-12");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/<year>-<month>",
            parameters: smallvec![("year", "2024"), ("month", "12")],
        })
    );

    let search = router.search("/2024-12-01");
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            template: "/<year>-<month>-<day>",
            parameters: smallvec![("year", "2024"), ("month", "12"), ("day", "01")],
        })
    );

    Ok(())
}

#[test]
fn dynamic_greedy() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<file>.<extension>", 1)?;

    insta::assert_snapshot!(router, @r"
    /
    ╰─ <file>
       ╰─ .
          ╰─ <extension>
    ");

    let search = router.search("/report");
    assert_eq!(search, None);

    let search = router.search("/report.pdf");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<file>.<extension>",
            parameters: smallvec![("file", "report"), ("extension", "pdf")],
        })
    );

    let search = router.search("/report.final.pdf");
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            template: "/<file>.<extension>",
            parameters: smallvec![("file", "report.final"), ("extension", "pdf")],
        })
    );

    Ok(())
}

#[test]
fn dynamic_priority() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/robots.txt", 1)?;
    router.insert("/robots.<extension>", 2)?;
    router.insert("/<name>.txt", 3)?;
    router.insert("/<name>.<extension>", 4)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ robots.
    │  ├─ txt
    │  ╰─ <extension>
    ╰─ <name>
       ╰─ .
          ├─ txt
          ╰─ <extension>
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

    let search = router.search("/robots.pdf");
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            template: "/robots.<extension>",
            parameters: smallvec![("extension", "pdf")],
        })
    );

    let search = router.search("/config.txt");
    assert_eq!(
        search,
        Some(Match {
            data: &3,
            template: "/<name>.txt",
            parameters: smallvec![("name", "config")],
        })
    );

    let search = router.search("/config.pdf");
    assert_eq!(
        search,
        Some(Match {
            data: &4,
            template: "/<name>.<extension>",
            parameters: smallvec![("name", "config"), ("extension", "pdf")],
        })
    );

    Ok(())
}
