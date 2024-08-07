#![allow(clippy::too_many_lines)]

use regex::bytes::Regex;
use std::error::Error;
use wayfind::{assert_router_matches, node::NodeConstraint, route::RouteBuilder, router::Router};

#[test]
fn test_inline_regex() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.insert(
        RouteBuilder::new("/user/<name>.<ext>")
            .constraint("name", NodeConstraint::Regex(Regex::new(r"[a-z]+")?))
            .constraint("ext", NodeConstraint::Regex(Regex::new(r"png|jpg")?))
            .build()?,
        1,
    )?;

    router.insert(
        RouteBuilder::new("/file-<year>-doc.<ext>")
            .constraint("year", NodeConstraint::Regex(Regex::new(r"\d{4}")?))
            .constraint("ext", NodeConstraint::Regex(Regex::new(r"pdf|docx")?))
            .build()?,
        2,
    )?;

    router.insert(
        RouteBuilder::new("/<category>-items.html")
            .constraint("category", NodeConstraint::Regex(Regex::new(r"[a-z-]+")?))
            .build()?,
        3,
    )?;

    router.insert(
        RouteBuilder::new("/report-<id>")
            .constraint("id", NodeConstraint::Regex(Regex::new(r"\d+")?))
            .build()?,
        4,
    )?;

    router.insert(
        RouteBuilder::new("/posts/<year>/<slug:*>")
            .constraint("year", NodeConstraint::Regex(Regex::new(r"\d{4}")?))
            .build()?,
        5,
    )?;

    router.insert(
        RouteBuilder::new("/products/<category>/<id>-<slug>")
            .constraint("category", NodeConstraint::Regex(Regex::new(r"[a-z]+")?))
            .constraint("id", NodeConstraint::Regex(Regex::new(r"\d+")?))
            .constraint("slug", NodeConstraint::Regex(Regex::new(r"[a-z-]+")?))
            .build()?,
        6,
    )?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ├─ user/
       │      ╰─ <name> Constraint::Regex: [a-z]+
       │              ╰─ .
       │                 ╰─ <ext> [1] Constraint::Regex: png|jpg
       ├─ file-
       │      ╰─ <year> Constraint::Regex: \d{4}
       │              ╰─ -doc.
       │                     ╰─ <ext> [2] Constraint::Regex: pdf|docx
       ├─ report-
       │        ╰─ <id> [4] Constraint::Regex: \d+
       ├─ p
       │  ├─ osts/
       │  │      ╰─ <year> Constraint::Regex: \d{4}
       │  │              ╰─ /
       │  │                 ╰─ <slug:*> [5]
       │  ╰─ roducts/
       │            ╰─ <category> Constraint::Regex: [a-z]+
       │                        ╰─ /
       │                           ╰─ <id> Constraint::Regex: \d+
       │                                 ╰─ -
       │                                    ╰─ <slug> [6] Constraint::Regex: [a-z-]+
       ╰─ <category> Constraint::Regex: [a-z-]+
                   ╰─ -items.html [3]
    "###);

    assert_router_matches!(router, {
        "/user/john.png" => {
            path: "/user/<name>.<ext>",
            value: 1,
            params: {
                "name" => "john",
                "ext" => "png"
            }
        }
        "/user/mary.jpg" => {
            path: "/user/<name>.<ext>",
            value: 1,
            params: {
                "name" => "mary",
                "ext" => "jpg"
            }
        }
        "/user/John.png" => None
        "/user/john.gif" => None

        "/file-2023-doc.pdf" => {
            path: "/file-<year>-doc.<ext>",
            value: 2,
            params: {
                "year" => "2023",
                "ext" => "pdf"
            }
        }
        "/file-2023-doc.docx" => {
            path: "/file-<year>-doc.<ext>",
            value: 2,
            params: {
                "year" => "2023",
                "ext" => "docx"
            }
        }
        "/file-203-doc.pdf" => None
        "/file-2023-doc.txt" => None

        "/tech-items.html" => {
            path: "/<category>-items.html",
            value: 3,
            params: {
                "category" => "tech"
            }
        }
        "/home-office-items.html" => {
            path: "/<category>-items.html",
            value: 3,
            params: {
                "category" => "home-office"
            }
        }
        "/TECH-items.html" => None
        "/tech-items.htm" => None

        "/report-123" => {
            path: "/report-<id>",
            value: 4,
            params: {
                "id" => "123"
            }
        }
        "/report-4567890" => {
            path: "/report-<id>",
            value: 4,
            params: {
                "id" => "4567890"
            }
        }
        "/report-12a" => None
        "/report-" => None

        "/posts/2023/my-awesome-post" => {
            path: "/posts/<year>/<slug:*>",
            value: 5,
            params: {
                "year" => "2023",
                "slug" => "my-awesome-post"
            }
        }
        "/posts/2023/nested/url/structure" => {
            path: "/posts/<year>/<slug:*>",
            value: 5,
            params: {
                "year" => "2023",
                "slug" => "nested/url/structure"
            }
        }
        "/posts/203/short-post" => None

        "/products/electronics/12345-smart-tv" => {
            path: "/products/<category>/<id>-<slug>",
            value: 6,
            params: {
                "category" => "electronics",
                "id" => "12345",
                "slug" => "smart-tv"
            }
        }
        "/products/home/67890-cozy-sofa" => {
            path: "/products/<category>/<id>-<slug>",
            value: 6,
            params: {
                "category" => "home",
                "id" => "67890",
                "slug" => "cozy-sofa"
            }
        }
        "/products/INVALID/12345-valid-slug" => None
        "/products/valid/INVALID-valid-slug" => None
        "/products/valid/12345-INVALID_SLUG" => None
    });

    Ok(())
}
