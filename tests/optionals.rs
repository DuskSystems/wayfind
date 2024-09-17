use std::error::Error;
use wayfind::Router;

#[path = "./utils.rs"]
mod utils;

#[test_log::test]
fn test_optional_wildcards() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("(/{*name})/abc", 1)?;
    router.insert("/def(/{*rest})", 2)?;
    router.insert("(/{*prefix})/ghi(/{*suffix})", 3)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ abc ○
       ├─ def ○
       │  ╰─ /
       │     ╰─ {*rest} ○
       ├─ ghi ○
       │  ╰─ /
       │     ╰─ {*suffix} ○
       ├─ {*name}
       │  ╰─ /abc ○
       ╰─ {*prefix}
          ╰─ /ghi ○
             ╰─ /
                ╰─ {*suffix} ○
    "#);

    assert_router_matches!(router, {
        "/abc" => {
            route: "(/{*name})/abc",
            expanded: "/abc",
            data: 1
        }
        "/xyz/abc" => {
            route: "(/{*name})/abc",
            expanded: "/{*name}/abc",
            data: 1,
            params: {
                "name" => "xyz"
            }
        }
        "/def" => {
            route: "/def(/{*rest})",
            expanded: "/def",
            data: 2
        }
        "/def/some/path" => {
            route: "/def(/{*rest})",
            expanded: "/def/{*rest}",
            data: 2,
            params: {
                "rest" => "some/path"
            }
        }
        "/ghi" => {
            route: "(/{*prefix})/ghi(/{*suffix})",
            expanded: "/ghi",
            data: 3
        }
        "/prefix/ghi" => {
            route: "(/{*prefix})/ghi(/{*suffix})",
            expanded: "/{*prefix}/ghi",
            data: 3,
            params: {
                "prefix" => "prefix"
            }
        }
        "/ghi/suffix" => {
            route: "(/{*prefix})/ghi(/{*suffix})",
            expanded: "/ghi/{*suffix}",
            data: 3,
            params: {
                "suffix" => "suffix"
            }
        }
        "/prefix/ghi/suffix" => {
            route: "(/{*prefix})/ghi(/{*suffix})",
            expanded: "/{*prefix}/ghi/{*suffix}",
            data: 3,
            params: {
                "prefix" => "prefix",
                "suffix" => "suffix"
            }
        }
    });

    Ok(())
}
