#![allow(clippy::too_many_lines)]

use std::error::Error;
use wayfind::{assert_router_matches, route::RouteBuilder, router::Router};

fn is_lowercase_alpha(segment: &str) -> bool {
    segment
        .chars()
        .all(|c| c.is_ascii_lowercase())
}

fn is_length_between_3_and_10(segment: &str) -> bool {
    (3..=10).contains(&segment.len())
}

fn is_digit(segment: &str) -> bool {
    segment
        .chars()
        .all(|c| c.is_ascii_digit())
}

fn is_year_1000_to_10000(segment: &str) -> bool {
    segment
        .parse::<u32>()
        .map(|num| (1000..=10000).contains(&num))
        .unwrap_or(false)
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

fn is_year_4_digits(segment: &str) -> bool {
    segment.len() == 4
        && segment
            .chars()
            .all(|c| c.is_ascii_digit())
}

fn is_valid_slug(segment: &str) -> bool {
    !segment.is_empty()
        && segment
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

#[test]
fn test_multiple_constraints() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.insert(
        RouteBuilder::new("/user/<name>/<id>")
            .constraint("name", is_lowercase_alpha)
            .constraint("name", is_length_between_3_and_10)
            .constraint("id", is_digit)
            .constraint("id", is_year_1000_to_10000)
            .build()?,
        1,
    )?;

    router.insert(
        RouteBuilder::new("/profile/<username>.<ext>")
            .constraint("username", is_lowercase_alpha)
            .constraint("username", is_length_between_3_and_10)
            .constraint("ext", is_png_or_jpg)
            .build()?,
        2,
    )?;

    router.insert(
        RouteBuilder::new("/posts/<year>/<slug>")
            .constraint("year", is_year_4_digits)
            .constraint("year", is_even_year)
            .constraint("slug", is_valid_slug)
            .constraint("slug", is_length_between_3_and_10)
            .build()?,
        3,
    )?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ├─ user/
       │      ╰─ <name> [NodeConstraint(<function>), NodeConstraint(<function>)]
       │              ╰─ /
       │                 ╰─ <id> [1] [NodeConstraint(<function>), NodeConstraint(<function>)]
       ╰─ p
          ├─ rofile/
          │        ╰─ <username> [NodeConstraint(<function>), NodeConstraint(<function>)]
          │                    ╰─ .
          │                       ╰─ <ext> [2] [NodeConstraint(<function>)]
          ╰─ osts/
                 ╰─ <year> [NodeConstraint(<function>), NodeConstraint(<function>)]
                         ╰─ /
                            ╰─ <slug> [3] [NodeConstraint(<function>), NodeConstraint(<function>)]
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
