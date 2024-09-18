use std::error::Error;
use wayfind::Router;

#[path = "./utils.rs"]
mod utils;

#[test]
fn test_trailing_slashes() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users(/)", 1)?;
    router.insert("/posts/{id}(/)", 2)?;
    router.insert("/articles(/{category})(/)", 3)?;
    router.insert("/files/{name}(.{extension})(/)", 4)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ articles ○
       │  ╰─ / ○
       │     ╰─ {category} ○
       │        ╰─ / ○
       ├─ files/
       │  ╰─ {name} ○
       │     ├─ .
       │     │  ╰─ {extension} ○
       │     │     ╰─ / ○
       │     ╰─ / ○
       ├─ posts/
       │  ╰─ {id} ○
       │     ╰─ / ○
       ╰─ users ○
          ╰─ / ○
    "#);

    assert_router_matches!(router, {
        "/users" => {
            route: "/users(/)",
            expanded: "/users",
            data: 1
        }
        "/users/" => {
            route: "/users(/)",
            expanded: "/users/",
            data: 1
        }
        "/posts/123" => {
            route: "/posts/{id}(/)",
            expanded: "/posts/{id}",
            data: 2,
            params: {
                "id" => "123"
            }
        }
        "/posts/123/" => {
            route: "/posts/{id}(/)",
            expanded: "/posts/{id}/",
            data: 2,
            params: {
                "id" => "123"
            }
        }
        "/articles" => {
            route: "/articles(/{category})(/)",
            expanded: "/articles",
            data: 3
        }
        "/articles/" => {
            route: "/articles(/{category})(/)",
            expanded: "/articles/",
            data: 3
        }
        "/articles/tech" => {
            route: "/articles(/{category})(/)",
            expanded: "/articles/{category}",
            data: 3,
            params: {
                "category" => "tech"
            }
        }
        "/articles/tech/" => {
            route: "/articles(/{category})(/)",
            expanded: "/articles/{category}/",
            data: 3,
            params: {
                "category" => "tech"
            }
        }
        "/files/document" => {
            route: "/files/{name}(.{extension})(/)",
            expanded: "/files/{name}",
            data: 4,
            params: {
                "name" => "document"
            }
        }
        "/files/document/" => {
            route: "/files/{name}(.{extension})(/)",
            expanded: "/files/{name}/",
            data: 4,
            params: {
                "name" => "document"
            }
        }
        "/files/document.pdf" => {
            route: "/files/{name}(.{extension})(/)",
            expanded: "/files/{name}.{extension}",
            data: 4,
            params: {
                "name" => "document",
                "extension" => "pdf"
            }
        }
        "/files/document.pdf/" => {
            route: "/files/{name}(.{extension})(/)",
            expanded: "/files/{name}.{extension}/",
            data: 4,
            params: {
                "name" => "document",
                "extension" => "pdf"
            }
        }
    });

    Ok(())
}
