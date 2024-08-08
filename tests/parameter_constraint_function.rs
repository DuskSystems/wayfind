#![allow(clippy::too_many_lines)]

use std::error::Error;
use wayfind::{
    assert_router_matches, constraints::parameter::ParameterConstraint, route::RouteBuilder, router::Router,
};

fn is_lowercase_alpha(segment: &str) -> bool {
    segment
        .chars()
        .all(|c| c.is_ascii_lowercase())
}

fn is_png_or_jpg(segment: &str) -> bool {
    segment == "png" || segment == "jpg"
}

fn is_four_digit_year(segment: &str) -> bool {
    segment.len() == 4
        && segment
            .chars()
            .all(|c| c.is_ascii_digit())
}

fn is_pdf_or_docx(segment: &str) -> bool {
    segment == "pdf" || segment == "docx"
}

fn is_lowercase_alpha_or_dash(segment: &str) -> bool {
    segment
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '-')
}

fn is_numeric(segment: &str) -> bool {
    segment
        .chars()
        .all(|c| c.is_ascii_digit())
}

#[test]
fn test_inline_functions() -> Result<(), Box<dyn Error>> {
    let mut router = Router::<_, ()>::new();

    router.insert(
        RouteBuilder::new("/user/<name>.<ext>")
            .parameter_constraint("name", ParameterConstraint::Function(is_lowercase_alpha))
            .parameter_constraint("ext", ParameterConstraint::Function(is_png_or_jpg))
            .build()?,
        1,
    )?;

    router.insert(
        RouteBuilder::new("/file-<year>-doc.<ext>")
            .parameter_constraint("year", ParameterConstraint::Function(is_four_digit_year))
            .parameter_constraint("ext", ParameterConstraint::Function(is_pdf_or_docx))
            .build()?,
        2,
    )?;

    router.insert(
        RouteBuilder::new("/<category>-items.html")
            .parameter_constraint("category", ParameterConstraint::Function(is_lowercase_alpha_or_dash))
            .build()?,
        3,
    )?;

    router.insert(
        RouteBuilder::new("/report-<id>")
            .parameter_constraint("id", ParameterConstraint::Function(is_numeric))
            .build()?,
        4,
    )?;

    router.insert(
        RouteBuilder::new("/posts/<year>/<slug:*>")
            .parameter_constraint("year", ParameterConstraint::Function(is_four_digit_year))
            .build()?,
        5,
    )?;

    router.insert(
        RouteBuilder::new("/products/<category>/<id>-<slug>")
            .parameter_constraint("category", ParameterConstraint::Function(is_lowercase_alpha))
            .parameter_constraint("id", ParameterConstraint::Function(is_numeric))
            .parameter_constraint("slug", ParameterConstraint::Function(is_lowercase_alpha_or_dash))
            .build()?,
        6,
    )?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ├─ user/
       │      ╰─ <name> [ParameterConstraint::Function]
       │              ╰─ .
       │                 ╰─ <ext> [1] [ParameterConstraint::Function]
       ├─ file-
       │      ╰─ <year> [ParameterConstraint::Function]
       │              ╰─ -doc.
       │                     ╰─ <ext> [2] [ParameterConstraint::Function]
       ├─ report-
       │        ╰─ <id> [4] [ParameterConstraint::Function]
       ├─ p
       │  ├─ osts/
       │  │      ╰─ <year> [ParameterConstraint::Function]
       │  │              ╰─ /
       │  │                 ╰─ <slug:*> [5]
       │  ╰─ roducts/
       │            ╰─ <category> [ParameterConstraint::Function]
       │                        ╰─ /
       │                           ╰─ <id> [ParameterConstraint::Function]
       │                                 ╰─ -
       │                                    ╰─ <slug> [6] [ParameterConstraint::Function]
       ╰─ <category> [ParameterConstraint::Function]
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
