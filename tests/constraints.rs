#![allow(clippy::too_many_lines)]

use regex::bytes::Regex;
use std::error::Error;
use wayfind::{assert_router_matches, node::NodeConstraint, route::RouteBuilder, router::Router};

fn is_lowercase_alpha(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .all(|&b| b.is_ascii_lowercase())
}

fn is_length_between_3_and_10(bytes: &[u8]) -> bool {
    (3..=10).contains(&bytes.len())
}

fn is_png_or_jpg(bytes: &[u8]) -> bool {
    let s = std::str::from_utf8(bytes).unwrap_or("");
    s == "png" || s == "jpg"
}

fn is_even_year(bytes: &[u8]) -> bool {
    let s = std::str::from_utf8(bytes).unwrap_or("");
    s.parse::<i32>()
        .map(|year| year % 2 == 0)
        .unwrap_or(false)
}

#[test]
fn test_multiple_constraints() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.insert(
        RouteBuilder::new("/user/<name>/<id>")
            .constraint("name", NodeConstraint::Regex(Regex::new(r"^[a-z]+$")?))
            .constraint("name", NodeConstraint::Regex(Regex::new(r"^.{3,10}$")?))
            .constraint("id", NodeConstraint::Regex(Regex::new(r"^\d+$")?))
            .constraint("id", NodeConstraint::Regex(Regex::new(r"^(?:[1-9]\d{3}|10000)$")?))
            .build()?,
        1,
    )?;

    router.insert(
        RouteBuilder::new("/profile/<username>.<ext>")
            .constraint("username", NodeConstraint::Function(is_lowercase_alpha))
            .constraint("username", NodeConstraint::Function(is_length_between_3_and_10))
            .constraint("ext", NodeConstraint::Function(is_png_or_jpg))
            .build()?,
        2,
    )?;

    router.insert(
        RouteBuilder::new("/posts/<year>/<slug>")
            .constraint("year", NodeConstraint::Regex(Regex::new(r"^\d{4}$")?))
            .constraint("year", NodeConstraint::Function(is_even_year))
            .constraint("slug", NodeConstraint::Regex(Regex::new(r"^[a-z0-9-]+$")?))
            .constraint("slug", NodeConstraint::Function(is_length_between_3_and_10))
            .build()?,
        3,
    )?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ├─ user/
       │      ╰─ <name> [Constraint::Regex(^[a-z]+$), Constraint::Regex(^.{3,10}$)]
       │              ╰─ /
       │                 ╰─ <id> [1] [Constraint::Regex(^\d+$), Constraint::Regex(^(?:[1-9]\d{3}|10000)$)]
       ╰─ p
          ├─ rofile/
          │        ╰─ <username> [Constraint::Function, Constraint::Function]
          │                    ╰─ .
          │                       ╰─ <ext> [2] [Constraint::Function]
          ╰─ osts/
                 ╰─ <year> [Constraint::Regex(^\d{4}$), Constraint::Function]
                         ╰─ /
                            ╰─ <slug> [3] [Constraint::Regex(^[a-z0-9-]+$), Constraint::Function]
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
