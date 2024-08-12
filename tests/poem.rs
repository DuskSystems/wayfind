//! Tests sourced from `poem` (MIT OR Apache-2.0)
//! <https://github.com/poem-web/poem/blob/0b6ca89be9636472b25f3677dc957fe098f72fab/poem/src/route/internal/radix_tree.rs>

#![allow(clippy::too_many_lines)]

use std::error::Error;
use wayfind::{assert_router_matches, node::Constraint, router::Router};

struct DigitString;
impl Constraint for DigitString {
    fn name() -> &'static str {
        "digit_string"
    }

    fn check(segment: &str) -> bool {
        !segment.is_empty()
            && segment
                .chars()
                .all(|c| c.is_ascii_digit())
    }
}

struct EndsWithTgz;
impl Constraint for EndsWithTgz {
    fn name() -> &'static str {
        "ends_with_tgz"
    }

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

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /abc [1]
          ╰─ def [2]
               ╰─ gh [3]
    "###);

    Ok(())
}

#[test]
fn test_insert_static_child_2() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/abcd", 1)?;
    router.insert("/ab1234", 2)?;
    router.insert("/ab1256", 3)?;
    router.insert("/ab125678", 4)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /ab
         ├─ 12
         │   ├─ 34 [2]
         │   ╰─ 56 [3]
         │       ╰─ 78 [4]
         ╰─ cd [1]
    "###);

    Ok(())
}

#[test]
fn test_insert_static_child_3() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/abc", 1)?;
    router.insert("/ab", 2)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /ab [2]
         ╰─ c [1]
    "###);

    Ok(())
}

#[test]
fn test_insert_param_child() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/abc/{p1}", 1)?;
    router.insert("/abc/{p1}/p2", 2)?;
    router.insert("/abc/{p1}/{p3}", 3)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /abc/
           ╰─ {p1} [1]
                 ╰─ /
                    ├─ p2 [2]
                    ╰─ {p3} [3]
    "###);

    Ok(())
}

#[test]
fn test_catch_all_child_1() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/abc/{*p1}", 1)?;
    router.insert("/ab/de", 2)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /ab
         ├─ /de [2]
         ╰─ c/
             ╰─ {*p1} [1]
    "###);

    Ok(())
}

#[test]
fn test_catch_all_child_2() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("{*p1}", 1)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ {*p1} [1]
    "###);

    Ok(())
}

#[test]
fn test_insert_regex_child() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.constraint::<DigitString>()?;

    router.insert("/abc/{name:digit_string}/def", 1)?;
    router.insert("/abc/def/{name:digit_string}", 2)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /abc/
           ├─ def/
           │     ╰─ {name:digit_string} [2]
           ╰─ {name:digit_string}
                                ╰─ /def [1]
    "###);

    Ok(())
}

#[test]
#[ignore = "todo"]
fn test_add_result() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.constraint::<DigitString>()?;

    assert!(router.insert("/a/b", 1).is_ok());
    assert!(router.insert("/a/b", 2).is_err());
    assert!(router.insert("/a/b/{p}/d", 1).is_ok());
    assert!(router.insert("/a/b/c/d", 2).is_ok());
    assert!(router.insert("/a/b/{p2}/d", 3).is_ok());
    assert!(router.insert("/a/{*p}", 1).is_ok());
    assert!(router.insert("/a/{*p}", 2).is_err());
    assert!(router.insert("/a/b/{*p}", 1).is_ok());
    assert!(router.insert("/a/b/{*p2}", 2).is_err());
    assert!(router
        .insert("/k/h/{name:digit_string}", 1)
        .is_ok());

    insta::assert_snapshot!(router, @"");

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

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ├─ a
       │  ├─ /
       │  │  ├─ b/c/d [7]
       │  │  ╰─ {p1}
       │  │        ╰─ /
       │  │           ╰─ {p2}
       │  │                 ╰─ /c [8]
       │  ╰─ b
       │     ├─ /def [1]
       │     ╰─ c/
       │         ├─ def [2]
       │         │    ╰─ /
       │         │       ╰─ {*p1} [6]
       │         ├─ {param:digit_string}
       │         │                     ╰─ /def [10]
       │         ╰─ {p1} [3]
       │               ╰─ /
       │                  ├─ def [4]
       │                  ╰─ {p2} [5]
       ├─ kcd/
       │     ╰─ {p1:digit_string} [11]
       ├─ {package}
       │          ╰─ /-/
       │               ╰─ {package_tgz:ends_with_tgz} [12]
       ╰─ {*p1} [9]
    "###);

    assert_router_matches!(router, {
        "/ab/def" => {
            path: "/ab/def",
            value: 1
        }
        "/abc/def" => {
            path: "/abc/def",
            value: 2
        }
        "/abc/cde" => {
            path: "/abc/{p1}",
            value: 3,
            params: {
                "p1" => "cde"
            }
        }
        "/abc/cde/def" => {
            path: "/abc/{p1}/def",
            value: 4,
            params: {
                "p1" => "cde"
            }
        }
        "/abc/cde/hjk" => {
            path: "/abc/{p1}/{p2}",
            value: 5,
            params: {
                "p1" => "cde",
                "p2" => "hjk"
            }
        }
        "/abc/def/iop/123" => {
            path: "/abc/def/{*p1}",
            value: 6,
            params: {
                "p1" => "iop/123"
            }
        }
        "/a/b/k/c" => {
            path: "/a/{p1}/{p2}/c",
            value: 8,
            params: {
                "p1" => "b",
                "p2" => "k"
            }
        }
        "/kcd/uio" => {
            path: "/{*p1}",
            value: 9,
            params: {
                "p1" => "kcd/uio"
            }
        }
        // NOTE: Different behaviour: poem would match "/{*p1}"
        "/" => None
        "/abc/123/def" => {
            path: "/abc/{param:digit_string}/def",
            value: 10,
            params: {
                "param" => "123"
            }
        }
        "/kcd/567" => {
            path: "/kcd/{p1:digit_string}",
            value: 11,
            params: {
                "p1" => "567"
            }
        }
        "/is-number/-/is-number-7.0.0.tgz" => {
            path: "/{package}/-/{package_tgz:ends_with_tgz}",
            value: 12,
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

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /a/
         ├─ bc [1]
         ╰─ {*path} [2]
    "###);

    assert_router_matches!(router, {
        "/a/123" => {
            path: "/a/{*path}",
            value: 2,
            params: {
                "path" => "123"
            }
        }
    });

    router.insert("/a/{id}", 3)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /a/
         ├─ bc [1]
         ├─ {id} [3]
         ╰─ {*path} [2]
    "###);

    assert_router_matches!(router, {
        "/a/123" => {
            path: "/a/{id}",
            value: 3,
            params: {
                "id" => "123"
            }
        }
    });

    router.constraint::<DigitString>()?;
    router.insert("/a/{id:digit_string}", 4)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /a/
         ├─ bc [1]
         ├─ {id:digit_string} [4]
         ├─ {id} [3]
         ╰─ {*path} [2]
    "###);

    assert_router_matches!(router, {
        "/a/123" => {
            path: "/a/{id:digit_string}",
            value: 4,
            params: {
                "id" => "123"
            }
        }
    });

    router.insert("/a/123", 5)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /a/
         ├─ 123 [5]
         ├─ bc [1]
         ├─ {id:digit_string} [4]
         ├─ {id} [3]
         ╰─ {*path} [2]
    "###);

    assert_router_matches!(router, {
        "/a/123" => {
            path: "/a/123",
            value: 5
        }
    });

    Ok(())
}

#[test]
fn test_catch_all_priority_in_sub_path() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/a/{*path}", 1)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /a/
         ╰─ {*path} [1]
    "###);

    assert_router_matches!(router, {
        "/a/b/c/123" => {
            path: "/a/{*path}",
            value: 1,
            params: {
                "path" => "b/c/123"
            }
        }
    });

    router.insert("/a/b/{*path}", 2)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /a/
         ├─ b/
         │   ╰─ {*path} [2]
         ╰─ {*path} [1]
    "###);

    assert_router_matches!(router, {
        "/a/b/c/123" => {
            path: "/a/b/{*path}",
            value: 2,
            params: {
                "path" => "c/123"
            }
        }
    });

    router.insert("/a/b/c/{*path}", 3)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /a/
         ├─ b/
         │   ├─ c/
         │   │   ╰─ {*path} [3]
         │   ╰─ {*path} [2]
         ╰─ {*path} [1]
    "###);

    assert_router_matches!(router, {
        "/a/b/c/123" => {
            path: "/a/b/c/{*path}",
            value: 3,
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

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ├─ {id1}
       │      ╰─ /a [1]
       ╰─ {id2}
              ╰─ /b [2]
    "###);

    assert_router_matches!(router, {
        "/abc/a" => {
            path: "/{id1}/a",
            value: 1,
            params: {
                "id1" => "abc"
            }
        }
        "/def/b" => {
            path: "/{id2}/b",
            value: 2,
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

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /a/
         ╰─ {id} [1]
    "###);

    assert_router_matches!(router, {
        "/a/abc" => {
            path: "/a/{id}",
            value: 1,
            params: {
                "id" => "abc"
            }
        }
        // NOTE: Different behaviour: poem would decode to `你好`
        "/a/%E4%BD%A0%E5%A5%BD" => {
            path: "/a/{id}",
            value: 1,
            params: {
                "id" => "%E4%BD%A0%E5%A5%BD"
            }
        }
    });

    Ok(())
}
