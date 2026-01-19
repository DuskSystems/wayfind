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
