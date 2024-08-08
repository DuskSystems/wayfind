#![allow(clippy::too_many_lines)]

use regex::Regex;
use std::error::Error;
use wayfind::{
    assert_router_matches, constraints::parameter::ParameterConstraint, route::RouteBuilder, router::Router,
};

fn is_lowercase_alpha(segment: &str) -> bool {
    segment
        .chars()
        .all(|c| c.is_ascii_lowercase())
}

fn is_length_between_3_and_10(segment: &str) -> bool {
    (3..=10).contains(&segment.len())
}

fn is_png_or_jpg(segment: &str) -> bool {
    segment == "png" || segment == "jpg"
}

fn is_even_year(segment: &str) -> bool {
    segment
        .parse::<i32>()
        .map(|year| year % 2 == 0)
        .unwrap_or(false)
}

#[test]
fn test_parameter_constraints() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.insert(
        RouteBuilder::new("/user/<name>/<id>")
            .parameter_constraint("name", ParameterConstraint::Regex(Regex::new(r"^[a-z]+$")?))
            .parameter_constraint("name", ParameterConstraint::Regex(Regex::new(r"^.{3,10}$")?))
            .parameter_constraint("id", ParameterConstraint::Regex(Regex::new(r"^\d+$")?))
            .parameter_constraint("id", ParameterConstraint::Regex(Regex::new(r"^(?:[1-9]\d{3}|10000)$")?))
            .build()?,
        1,
    )?;

    router.insert(
        RouteBuilder::new("/profile/<username>.<ext>")
            .parameter_constraint("username", ParameterConstraint::Function(is_lowercase_alpha))
            .parameter_constraint("username", ParameterConstraint::Function(is_length_between_3_and_10))
            .parameter_constraint("ext", ParameterConstraint::Function(is_png_or_jpg))
            .build()?,
        2,
    )?;

    router.insert(
        RouteBuilder::new("/posts/<year>/<slug>")
            .parameter_constraint("year", ParameterConstraint::Regex(Regex::new(r"^\d{4}$")?))
            .parameter_constraint("year", ParameterConstraint::Function(is_even_year))
            .parameter_constraint("slug", ParameterConstraint::Regex(Regex::new(r"^[a-z0-9-]+$")?))
            .parameter_constraint("slug", ParameterConstraint::Function(is_length_between_3_and_10))
            .build()?,
        3,
    )?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ├─ p
       │  ├─ osts/
       │  │      ╰─ <year> [ParameterConstraint::Function, ParameterConstraint::Regex(^\d{4}$)]
       │  │              ╰─ /
       │  │                 ╰─ <slug> [3] [ParameterConstraint::Function, ParameterConstraint::Regex(^[a-z0-9-]+$)]
       │  ╰─ rofile/
       │           ╰─ <username> [ParameterConstraint::Function, ParameterConstraint::Function]
       │                       ╰─ .
       │                          ╰─ <ext> [2] [ParameterConstraint::Function]
       ╰─ user/
              ╰─ <name> [ParameterConstraint::Regex(^[a-z]+$), ParameterConstraint::Regex(^.{3,10}$)]
                      ╰─ /
                         ╰─ <id> [1] [ParameterConstraint::Regex(^\d+$), ParameterConstraint::Regex(^(?:[1-9]\d{3}|10000)$)]
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
