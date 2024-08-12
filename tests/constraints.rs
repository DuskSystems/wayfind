#![allow(clippy::too_many_lines)]

use std::error::Error;
use wayfind::{assert_router_matches, node::Constraint, route::RouteBuilder, router::Router};

struct LengthBetween3And10;
impl Constraint for LengthBetween3And10 {
    fn name() -> &'static str {
        "length_3_to_10"
    }

    fn check(segment: &str) -> bool {
        (3..=10).contains(&segment.len())
    }
}

struct Year1000To10000;
impl Constraint for Year1000To10000 {
    fn name() -> &'static str {
        "year_1000_to_10000"
    }

    fn check(segment: &str) -> bool {
        segment
            .parse::<u32>()
            .map(|num| (1000..=10000).contains(&num))
            .unwrap_or(false)
    }
}

struct PngOrJpg;
impl Constraint for PngOrJpg {
    fn name() -> &'static str {
        "png_or_jpg"
    }

    fn check(segment: &str) -> bool {
        segment == "png" || segment == "jpg"
    }
}

struct EvenYear;
impl Constraint for EvenYear {
    fn name() -> &'static str {
        "even_year"
    }

    fn check(segment: &str) -> bool {
        segment
            .parse::<i32>()
            .map(|year| year % 2 == 0)
            .unwrap_or(false)
    }
}

struct ValidSlug;
impl Constraint for ValidSlug {
    fn name() -> &'static str {
        "valid_slug"
    }

    fn check(segment: &str) -> bool {
        !segment.is_empty()
            && (3..=10).contains(&segment.len())
            && segment
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    }
}

#[test]
fn test_multiple_constraints() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.insert(
        RouteBuilder::new("/user/<name>/<id>")
            .constraint::<LengthBetween3And10>("name")
            .constraint::<Year1000To10000>("id")
            .build()?,
        1,
    )?;

    router.insert(
        RouteBuilder::new("/profile/<username>.<ext>")
            .constraint::<LengthBetween3And10>("username")
            .constraint::<PngOrJpg>("ext")
            .build()?,
        2,
    )?;

    router.insert(
        RouteBuilder::new("/posts/<year>/<slug>")
            .constraint::<EvenYear>("year")
            .constraint::<ValidSlug>("slug")
            .build()?,
        3,
    )?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ├─ p
       │  ├─ osts/
       │  │      ╰─ <year> (even_year)
       │  │              ╰─ /
       │  │                 ╰─ <slug> [3] (valid_slug)
       │  ╰─ rofile/
       │           ╰─ <username> (length_3_to_10)
       │                       ╰─ .
       │                          ╰─ <ext> [2] (png_or_jpg)
       ╰─ user/
              ╰─ <name> (length_3_to_10)
                      ╰─ /
                         ╰─ <id> [1] (year_1000_to_10000)
    "###);

    assert_router_matches!(router, {
        "/user/john/1234" => {
            path: "/user/<name>/<id>",
            value: 1,
            params: {
                "name" => "john",
                "id" => "1234"
            }
        }
        "/user/johndoe/10000" => {
            path: "/user/<name>/<id>",
            value: 1,
            params: {
                "name" => "johndoe",
                "id" => "10000"
            }
        }
        "/user/j/1234" => None
        "/user/johndoetolong/1234" => None
        "/user/john/123" => None
        "/user/john/10001" => None

        "/profile/alice.png" => {
            path: "/profile/<username>.<ext>",
            value: 2,
            params: {
                "username" => "alice",
                "ext" => "png"
            }
        }
        "/profile/bob.jpg" => {
            path: "/profile/<username>.<ext>",
            value: 2,
            params: {
                "username" => "bob",
                "ext" => "jpg"
            }
        }
        "/profile/a.png" => None
        "/profile/toolongusername.png" => None
        "/profile/alice.gif" => None

        "/posts/2022/hello" => {
            path: "/posts/<year>/<slug>",
            value: 3,
            params: {
                "year" => "2022",
                "slug" => "hello"
            }
        }
        "/posts/2024/test-123" => {
            path: "/posts/<year>/<slug>",
            value: 3,
            params: {
                "year" => "2024",
                "slug" => "test-123"
            }
        }
        "/posts/2023/hello" => None
        "/posts/2022/toolongslug" => None
        "/posts/2022/INVALID" => None
    });

    Ok(())
}
