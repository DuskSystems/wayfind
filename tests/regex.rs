use std::error::Error;
use wayfind::{assert_router_matches, router::Router};

#[test]
fn test_inline_regex() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/user/{name:[a-z]+}.{ext:png|jpg}", 1)?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /user/
            ╰─ {name:[a-z]+}
                           ╰─ .
                              ╰─ {ext:png|jpg} [1]
    "###);

    assert_router_matches!(router, {
        "/user/john.png" => {
            path: "/user/{name:[a-z]+}.{ext:png|jpg}",
            value: 1,
            params: {
                "name" => "john",
                "ext" => "png"
            }
        }
        "/user/mary.jpg" => {
            path: "/user/{name:[a-z]+}.{ext:png|jpg}",
            value: 1,
            params: {
                "name" => "mary",
                "ext" => "jpg"
            }
        }
        "/user/John.png" => None
        "/user/john.gif" => None
    });

    Ok(())
}
