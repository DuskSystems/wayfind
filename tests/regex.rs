#![allow(clippy::too_many_lines)]

#[cfg(regex)]
use std::error::Error;
#[cfg(regex)]
use wayfind::{assert_router_matches, router::Router};

#[test]
#[cfg(regex)]
fn test_inline_regex() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/user/<name:[a-z]+>.<ext:png|jpg>", 1)?;
    router.insert("/file-<year:\\d\\d\\d\\d>-doc.<ext:pdf|docx>", 2)?;
    router.insert("/<category:[a-z-]+>-items.html", 3)?;
    router.insert("/report-<id:\\d+>", 4)?;
    router.insert("/posts/<year:\\d\\d\\d\\d>/<slug:*>", 5)?;
    router.insert("/products/<category:[a-z]+>/<id:\\d+>-<slug:[a-z-]+>", 6)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ├─ user/
       │      ╰─ <name:[a-z]+>
       │                     ╰─ .
       │                        ╰─ <ext:png|jpg> [1]
       ├─ file-
       │      ╰─ <year:\d\d\d\d>
       │                       ╰─ -doc.
       │                              ╰─ <ext:pdf|docx> [2]
       ├─ report-
       │        ╰─ <id:\d+> [4]
       ├─ p
       │  ├─ osts/
       │  │      ╰─ <year:\d\d\d\d>
       │  │                       ╰─ /
       │  │                          ╰─ <slug:*> [5]
       │  ╰─ roducts/
       │            ╰─ <category:[a-z]+>
       │                               ╰─ /
       │                                  ╰─ <id:\d+>
       │                                            ╰─ -
       │                                               ╰─ <slug:[a-z-]+> [6]
       ╰─ <category:[a-z-]+>
                           ╰─ -items.html [3]
    "###);

    assert_router_matches!(router, {
        "/user/john.png" => {
            path: "/user/<name:[a-z]+>.<ext:png|jpg>",
            value: 1,
            params: {
                "name" => "john",
                "ext" => "png"
            }
        }
        "/user/mary.jpg" => {
            path: "/user/<name:[a-z]+>.<ext:png|jpg>",
            value: 1,
            params: {
                "name" => "mary",
                "ext" => "jpg"
            }
        }
        "/user/John.png" => None
        "/user/john.gif" => None

        "/file-2023-doc.pdf" => {
            path: "/file-<year:\\d\\d\\d\\d>-doc.<ext:pdf|docx>",
            value: 2,
            params: {
                "year" => "2023",
                "ext" => "pdf"
            }
        }
        "/file-2023-doc.docx" => {
            path: "/file-<year:\\d\\d\\d\\d>-doc.<ext:pdf|docx>",
            value: 2,
            params: {
                "year" => "2023",
                "ext" => "docx"
            }
        }
        "/file-203-doc.pdf" => None
        "/file-2023-doc.txt" => None

        "/tech-items.html" => {
            path: "/<category:[a-z-]+>-items.html",
            value: 3,
            params: {
                "category" => "tech"
            }
        }
        "/home-office-items.html" => {
            path: "/<category:[a-z-]+>-items.html",
            value: 3,
            params: {
                "category" => "home-office"
            }
        }
        "/TECH-items.html" => None
        "/tech-items.htm" => None

        "/report-123" => {
            path: "/report-<id:\\d+>",
            value: 4,
            params: {
                "id" => "123"
            }
        }
        "/report-4567890" => {
            path: "/report-<id:\\d+>",
            value: 4,
            params: {
                "id" => "4567890"
            }
        }
        "/report-12a" => None
        "/report-" => None

        "/posts/2023/my-awesome-post" => {
            path: "/posts/<year:\\d\\d\\d\\d>/<slug:*>",
            value: 5,
            params: {
                "year" => "2023",
                "slug" => "my-awesome-post"
            }
        }
        "/posts/2023/nested/url/structure" => {
            path: "/posts/<year:\\d\\d\\d\\d>/<slug:*>",
            value: 5,
            params: {
                "year" => "2023",
                "slug" => "nested/url/structure"
            }
        }
        "/posts/203/short-post" => None

        "/products/electronics/12345-smart-tv" => {
            path: "/products/<category:[a-z]+>/<id:\\d+>-<slug:[a-z-]+>",
            value: 6,
            params: {
                "category" => "electronics",
                "id" => "12345",
                "slug" => "smart-tv"
            }
        }
        "/products/home/67890-cozy-sofa" => {
            path: "/products/<category:[a-z]+>/<id:\\d+>-<slug:[a-z-]+>",
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
