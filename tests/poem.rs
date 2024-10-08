//! Tests sourced from `poem` (MIT OR Apache-2.0)
//! <https://github.com/poem-web/poem/blob/0b6ca89be9636472b25f3677dc957fe098f72fab/poem/src/route/internal/radix_tree.rs>

use std::error::Error;
use wayfind::{Constraint, Router};

#[path = "./utils.rs"]
mod utils;

struct DigitString;
impl Constraint for DigitString {
    const NAME: &'static str = "digit_string";

    fn check(segment: &str) -> bool {
        !segment.is_empty() && segment.chars().all(|c| c.is_ascii_digit())
    }
}

struct EndsWithTgz;
impl Constraint for EndsWithTgz {
    const NAME: &'static str = "ends_with_tgz";

    fn check(segment: &str) -> bool {
        #[allow(clippy::case_sensitive_file_extension_comparisons)]
        segment.ends_with(".tgz")
    }
}

#[test]
fn test_insert_static_child_1() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/abc", 1)?;
    router.insert("/abcdef", 2)?;
    router.insert("/abcdefgh", 3)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /abc ○
       ╰─ def ○
          ╰─ gh ○
    "#);

    Ok(())
}

#[test]
fn test_insert_static_child_2() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/abcd", 1)?;
    router.insert("/ab1234", 2)?;
    router.insert("/ab1256", 3)?;
    router.insert("/ab125678", 4)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /ab
       ├─ 12
       │  ├─ 34 ○
       │  ╰─ 56 ○
       │     ╰─ 78 ○
       ╰─ cd ○
    "#);

    Ok(())
}

#[test]
fn test_insert_static_child_3() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/abc", 1)?;
    router.insert("/ab", 2)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /ab ○
       ╰─ c ○
    "#);

    Ok(())
}

#[test]
fn test_insert_param_child() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/abc/{p1}", 1)?;
    router.insert("/abc/{p1}/p2", 2)?;
    router.insert("/abc/{p1}/{p3}", 3)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /abc/
       ╰─ {p1} ○
          ╰─ /
             ├─ p2 ○
             ╰─ {p3} ○
    "#);

    Ok(())
}

#[test]
fn test_catch_all_child_1() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/abc/{*p1}", 1)?;
    router.insert("/ab/de", 2)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /ab
       ├─ /de ○
       ╰─ c/
          ╰─ {*p1} ○
    "#);

    Ok(())
}

#[test]
fn test_catch_all_child_2() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("{*p1}", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ {*p1} ○
    "#);

    Ok(())
}

#[test]
fn test_insert_regex_child() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.constraint::<DigitString>()?;

    router.insert("/abc/{name:digit_string}/def", 1)?;
    router.insert("/abc/def/{name:digit_string}", 2)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /abc/
       ├─ def/
       │  ╰─ {name:digit_string} ○
       ╰─ {name:digit_string}
          ╰─ /def ○
    "#);

    Ok(())
}

#[test]
fn test_add_result() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.constraint::<DigitString>()?;

    router.insert("/a/b", 1)?;

    let error = router.insert("/a/b", 2).unwrap_err();
    insta::assert_snapshot!(error, @r#"
    duplicate route

          Route: /a/b
       Conflict: /a/b
    "#);

    router.insert("/a/b/{p}/d", 1)?;
    router.insert("/a/b/c/d", 2)?;
    router.insert("/a/b/{p2}/d", 3)?;
    router.insert("/a/{*p}", 1)?;

    let error = router.insert("/a/{*p}", 2).unwrap_err();
    insta::assert_snapshot!(error, @r#"
    duplicate route

          Route: /a/{*p}
       Conflict: /a/{*p}
    "#);

    router.insert("/k/h/{name:digit_string}", 1)?;

    // FIXME
    // assert!(router.insert("/a/b/{*p}", 1).is_ok());
    // assert!(router.insert("/a/b/{*p2}", 2).is_err());

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ a/
       │  ├─ b ○
       │  │  ╰─ /
       │  │     ├─ c/d ○
       │  │     ├─ {p}
       │  │     │  ╰─ /d ○
       │  │     ╰─ {p2}
       │  │        ╰─ /d ○
       │  ╰─ {*p} ○
       ╰─ k/h/
          ╰─ {name:digit_string} ○
    "#);

    Ok(())
}

#[test]
fn test_matches() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.constraint::<DigitString>()?;
    router.constraint::<EndsWithTgz>()?;

    router.insert("/ab/def", 1)?;
    router.insert("/abc/def", 2)?;
    router.insert("/abc/{p1}", 3)?;
    router.insert("/abc/{p1}/def", 4)?;
    router.insert("/abc/{p1}/{p2}", 5)?;
    router.insert("/abc/def/{*p1}", 6)?;
    router.insert("/a/b/c/d", 7)?;
    router.insert("/a/{p1}/{p2}/c", 8)?;
    router.insert("/{*p1}", 9)?;
    router.insert("/abc/{param:digit_string}/def", 10)?;
    router.insert("/kcd/{p1:digit_string}", 11)?;
    router.insert("/{package}/-/{package_tgz:ends_with_tgz}", 12)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ a
       │  ├─ /
       │  │  ├─ b/c/d ○
       │  │  ╰─ {p1}
       │  │     ╰─ /
       │  │        ╰─ {p2}
       │  │           ╰─ /c ○
       │  ╰─ b
       │     ├─ /def ○
       │     ╰─ c/
       │        ├─ def ○
       │        │  ╰─ /
       │        │     ╰─ {*p1} ○
       │        ├─ {param:digit_string}
       │        │  ╰─ /def ○
       │        ╰─ {p1} ○
       │           ╰─ /
       │              ├─ def ○
       │              ╰─ {p2} ○
       ├─ kcd/
       │  ╰─ {p1:digit_string} ○
       ├─ {package}
       │  ╰─ /-/
       │     ╰─ {package_tgz:ends_with_tgz} ○
       ╰─ {*p1} ○
    "#);

    assert_router_matches!(router, {
        "/ab/def" => {
            route: "/ab/def",
            data: 1
        }
        "/abc/def" => {
            route: "/abc/def",
            data: 2
        }
        "/abc/cde" => {
            route: "/abc/{p1}",
            data: 3,
            params: {
                "p1" => "cde"
            }
        }
        "/abc/cde/def" => {
            route: "/abc/{p1}/def",
            data: 4,
            params: {
                "p1" => "cde"
            }
        }
        "/abc/cde/hjk" => {
            route: "/abc/{p1}/{p2}",
            data: 5,
            params: {
                "p1" => "cde",
                "p2" => "hjk"
            }
        }
        "/abc/def/iop/123" => {
            route: "/abc/def/{*p1}",
            data: 6,
            params: {
                "p1" => "iop/123"
            }
        }
        "/a/b/k/c" => {
            route: "/a/{p1}/{p2}/c",
            data: 8,
            params: {
                "p1" => "b",
                "p2" => "k"
            }
        }
        "/kcd/uio" => {
            route: "/{*p1}",
            data: 9,
            params: {
                "p1" => "kcd/uio"
            }
        }
        // NOTE: Different behaviour: poem would match "/{*p1}"
        "/" => None
        "/abc/123/def" => {
            route: "/abc/{param:digit_string}/def",
            data: 10,
            params: {
                "param" => "123"
            }
        }
        "/kcd/567" => {
            route: "/kcd/{p1:digit_string}",
            data: 11,
            params: {
                "p1" => "567"
            }
        }
        "/is-number/-/is-number-7.0.0.tgz" => {
            route: "/{package}/-/{package_tgz:ends_with_tgz}",
            data: 12,
            params: {
                "package" => "is-number",
                "package_tgz" => "is-number-7.0.0.tgz"
            }
        }
    });

    Ok(())
}

#[test]
fn test_match_priority() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/a/bc", 1)?;
    router.insert("/a/{*path}", 2)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /a/
       ├─ bc ○
       ╰─ {*path} ○
    "#);

    assert_router_matches!(router, {
        "/a/123" => {
            route: "/a/{*path}",
            data: 2,
            params: {
                "path" => "123"
            }
        }
    });

    router.insert("/a/{id}", 3)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /a/
       ├─ bc ○
       ├─ {id} ○
       ╰─ {*path} ○
    "#);

    assert_router_matches!(router, {
        "/a/123" => {
            route: "/a/{id}",
            data: 3,
            params: {
                "id" => "123"
            }
        }
    });

    router.constraint::<DigitString>()?;
    router.insert("/a/{id:digit_string}", 4)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /a/
       ├─ bc ○
       ├─ {id:digit_string} ○
       ├─ {id} ○
       ╰─ {*path} ○
    "#);

    assert_router_matches!(router, {
        "/a/123" => {
            route: "/a/{id:digit_string}",
            data: 4,
            params: {
                "id" => "123"
            }
        }
    });

    router.insert("/a/123", 5)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /a/
       ├─ 123 ○
       ├─ bc ○
       ├─ {id:digit_string} ○
       ├─ {id} ○
       ╰─ {*path} ○
    "#);

    assert_router_matches!(router, {
        "/a/123" => {
            route: "/a/123",
            data: 5
        }
    });

    Ok(())
}

#[test]
fn test_catch_all_priority_in_sub_path() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/a/{*path}", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /a/
       ╰─ {*path} ○
    "#);

    assert_router_matches!(router, {
        "/a/b/c/123" => {
            route: "/a/{*path}",
            data: 1,
            params: {
                "path" => "b/c/123"
            }
        }
    });

    router.insert("/a/b/{*path}", 2)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /a/
       ├─ b/
       │  ╰─ {*path} ○
       ╰─ {*path} ○
    "#);

    assert_router_matches!(router, {
        "/a/b/c/123" => {
            route: "/a/b/{*path}",
            data: 2,
            params: {
                "path" => "c/123"
            }
        }
    });

    router.insert("/a/b/c/{*path}", 3)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /a/
       ├─ b/
       │  ├─ c/
       │  │  ╰─ {*path} ○
       │  ╰─ {*path} ○
       ╰─ {*path} ○
    "#);

    assert_router_matches!(router, {
        "/a/b/c/123" => {
            route: "/a/b/c/{*path}",
            data: 3,
            params: {
                "path" => "123"
            }
        }
    });

    Ok(())
}

#[test]
fn test_issue_275() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{id1}/a", 1)?;
    router.insert("/{id2}/b", 2)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ {id1}
       │  ╰─ /a ○
       ╰─ {id2}
          ╰─ /b ○
    "#);

    assert_router_matches!(router, {
        "/abc/a" => {
            route: "/{id1}/a",
            data: 1,
            params: {
                "id1" => "abc"
            }
        }
        "/def/b" => {
            route: "/{id2}/b",
            data: 2,
            params: {
                "id2" => "def"
            }
        }
    });

    Ok(())
}

#[test]
fn test_percent_decoded() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/a/{id}", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /a/
       ╰─ {id} ○
    "#);

    assert_router_matches!(router, {
        "/a/abc" => {
            route: "/a/{id}",
            data: 1,
            params: {
                "id" => "abc"
            }
        }
        "/a/%E4%BD%A0%E5%A5%BD" => {
            route: "/a/{id}",
            data: 1,
            params: {
                "id" => "你好"
            }
        }
    });

    Ok(())
}
