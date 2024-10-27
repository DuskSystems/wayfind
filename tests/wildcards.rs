#![allow(clippy::too_many_lines)]

use std::error::Error;
use wayfind::Router;

#[path = "./utils.rs"]
mod utils;

#[test]
fn test_inline_wildcards_extensive() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    // Basic inline wildcards
    router.insert("/{*prefix}-file.txt", 1)?;
    router.insert("/{*path}.{extension}", 2)?;

    // Greediness test
    router.insert("/greedy-{*middle}-end.txt", 3)?;

    // Multiple inline wildcards
    router.insert("/{*start}-{*middle}-{*end}.txt", 4)?;

    // Combination with dynamic parameters
    router.insert("/{category}/{*title}-{id}.html", 5)?;

    // Inline wildcard followed by dynamic parameter
    router.insert("/{*path}/{filename}", 6)?;

    // Inline wildcard into dynamic into end wildcard
    router.insert("/articles/{*title}-{year}/{*rest}", 7)?;

    // Complex combinations
    router.insert("/{*prefix}_{dynamic}_{*suffix}.{extension}", 8)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ articles/
       │  ╰─ {*title}
       │     ╰─ -
       │        ╰─ {year}
       │           ╰─ /
       │              ╰─ {*rest} ○
       ├─ greedy-
       │  ╰─ {*middle}
       │     ╰─ -end.txt ○
       ├─ {category}
       │  ╰─ /
       │     ╰─ {*title}
       │        ╰─ -
       │           ╰─ {id}
       │              ╰─ .html ○
       ├─ {*prefix}
       │  ├─ -file.txt ○
       │  ╰─ _
       │     ╰─ {dynamic}
       │        ╰─ _
       │           ╰─ {*suffix}
       │              ╰─ .
       │                 ╰─ {extension} ○
       ├─ {*start}
       │  ╰─ -
       │     ╰─ {*middle}
       │        ╰─ -
       │           ╰─ {*end}
       │              ╰─ .txt ○
       ╰─ {*path}
          ├─ .
          │  ╰─ {extension} ○
          ╰─ /
             ╰─ {filename} ○
    "#);

    assert_router_matches!(router, {
        // Basic inline wildcards
        "/my-file.txt" => {
            route: "/{*prefix}-file.txt",
            data: 1,
            params: {
                "prefix" => "my"
            }
        }
        "/path/to/document.pdf" => {
            route: "/{*path}/{filename}",
            data: 6,
            params: {
                "path" => "path/to",
                "filename" => "document.pdf"
            }
        }

        // Greediness test
        "/greedy-middle-part-end.txt" => {
            route: "/greedy-{*middle}-end.txt",
            data: 3,
            params: {
                "middle" => "middle-part"
            }
        }

        // Multiple inline wildcards
        "/start-middle-end.txt" => {
            route: "/{*start}-{*middle}-{*end}.txt",
            data: 4,
            params: {
                "start" => "start",
                "middle" => "middle",
                "end" => "end"
            }
        }
        "/complex-multi-part-wildcard.txt" => {
            route: "/{*start}-{*middle}-{*end}.txt",
            data: 4,
            params: {
                "start" => "complex-multi",
                "middle" => "part",
                "end" => "wildcard"
            }
        }

        // Combination with dynamic parameters
        "/news/breaking-news-12345.html" => {
            route: "/{category}/{*title}-{id}.html",
            data: 5,
            params: {
                "category" => "news",
                "title" => "breaking-news",
                "id" => "12345"
            }
        }

        // Inline wildcard followed by dynamic parameter
        "/very/long/path/filename.txt" => {
            route: "/{*path}/{filename}",
            data: 6,
            params: {
                "path" => "very/long/path",
                "filename" => "filename.txt"
            }
        }

        // Inline wildcard into dynamic into end wildcard
        "/articles/long-title-with-hyphens-2023/extra/path/segments" => {
            route: "/articles/{*title}-{year}/{*rest}",
            data: 7,
            params: {
                "title" => "long-title-with-hyphens",
                "year" => "2023",
                "rest" => "extra/path/segments"
            }
        }

        // Complex combinations
        "/prefix_value_suffix.ext" => {
            route: "/{*prefix}_{dynamic}_{*suffix}.{extension}",
            data: 8,
            params: {
                "prefix" => "prefix",
                "dynamic" => "value",
                "suffix" => "suffix",
                "extension" => "ext"
            }
        }
        "/long/prefix_with_underscores_dynamic_complex_suffix.html" => {
            route: "/{*prefix}_{dynamic}_{*suffix}.{extension}",
            data: 8,
            params: {
                "prefix" => "long/prefix_with_underscores_dynamic",
                "dynamic" => "complex",
                "suffix" => "suffix",
                "extension" => "html"
            }
        }

        // Test cases for non-matches
        "/not-matching-any-route" => None
    });

    Ok(())
}
