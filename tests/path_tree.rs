//! Tests sourced from `path-tree` (MIT OR Apache-2.0)
//! <https://github.com/viz-rs/path-tree/blob/v0.8.1/tests/tree.rs>

use similar_asserts::assert_eq;
use std::error::Error;
use wayfind::Router;

#[path = "./utils.rs"]
mod utils;

#[test]
fn statics() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/", 0)?;
    router.insert("/hi", 1)?;
    router.insert("/contact", 2)?;
    router.insert("/co", 3)?;
    router.insert("/c", 4)?;
    router.insert("/a", 5)?;
    router.insert("/ab", 6)?;
    router.insert("/doc/", 7)?;
    router.insert("/doc/go_faq.html", 8)?;
    router.insert("/doc/go1.html", 9)?;
    router.insert("/α", 10)?;
    router.insert("/β", 11)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ / ○
       ├─ doc/ ○
       │  ╰─ go
       │     ├─ _faq.html ○
       │     ╰─ 1.html ○
       ├─ hi ○
       ├─ �
       │  ├─ � ○
       │  ╰─ � ○
       ├─ a ○
       │  ╰─ b ○
       ╰─ c ○
          ╰─ o ○
             ╰─ ntact ○
    "#);

    assert_router_matches!(router, {
        "/" => {
            route: "/",
            data: 0
        }
        "/hi" => {
            route: "/hi",
            data: 1
        }
        "/contact" => {
            route: "/contact",
            data: 2
        }
        "/co" => {
            route: "/co",
            data: 3
        }
        "/c" => {
            route: "/c",
            data: 4
        }
        "/a" => {
            route: "/a",
            data: 5
        }
        "/ab" => {
            route: "/ab",
            data: 6
        }
        "/doc/" => {
            route: "/doc/",
            data: 7
        }
        "/doc/go_faq.html" => {
            route: "/doc/go_faq.html",
            data: 8
        }
        "/doc/go1.html" => {
            route: "/doc/go1.html",
            data: 9
        }
        "/α" => {
            route: "/α",
            data: 10
        }
        "/β" => {
            route: "/β",
            data: 11
        }
    });

    Ok(())
}

#[test]
fn wildcards() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/", 0)?;
    router.insert("/cmd/{tool}/{sub}", 1)?;
    router.insert("/cmd/{tool}/", 2)?;
    router.insert("/cmd/vet", 3)?;
    router.insert("/src/{*filepath}", 4)?;
    router.insert("/src1/", 5)?;
    router.insert("/src1/{*filepath}", 6)?;
    // NOTE: We don't support 'inline wildcard' logic.
    // router.insert("/src2{*filepath}", 7)?;
    router.insert("/search/", 8)?;
    router.insert("/search/{query}", 9)?;
    router.insert("/search/invalid", 10)?;
    router.insert("/user_{name}", 11)?;
    router.insert("/user_{name}/about", 12)?;
    router.insert("/user_x", 13)?;
    router.insert("/files/{dir}/{*filepath}", 14)?;
    router.insert("/doc/", 15)?;
    router.insert("/doc/rust_faq.html", 16)?;
    router.insert("/doc/rust1.html", 17)?;
    router.insert("/info/{user}/public", 18)?;
    router.insert("/info/{user}/project/{project}", 19)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ / ○
       ├─ doc/ ○
       │  ╰─ rust
       │     ├─ _faq.html ○
       │     ╰─ 1.html ○
       ├─ files/
       │  ╰─ {dir}
       │     ╰─ /
       │        ╰─ {*filepath} ○
       ├─ info/
       │  ╰─ {user}
       │     ╰─ /p
       │        ├─ ublic ○
       │        ╰─ roject/
       │           ╰─ {project} ○
       ├─ user_
       │  ├─ x ○
       │  ╰─ {name} ○
       │     ╰─ /about ○
       ├─ cmd/
       │  ├─ vet ○
       │  ╰─ {tool}
       │     ╰─ / ○
       │        ╰─ {sub} ○
       ╰─ s
          ├─ earch/ ○
          │  ├─ invalid ○
          │  ╰─ {query} ○
          ╰─ rc
             ├─ 1/ ○
             │  ╰─ {*filepath} ○
             ╰─ /
                ╰─ {*filepath} ○
    "#);

    assert_router_matches!(router, {
        "/" => {
            route: "/",
            data: 0
        }
        "/cmd/test/" => {
            route: "/cmd/{tool}/",
            data: 2,
            params: {
                "tool" => "test"
            }
        }
        "/cmd/test/3" => {
            route: "/cmd/{tool}/{sub}",
            data: 1,
            params: {
                "tool" => "test",
                "sub" => "3"
            }
        }
        // NOTE: Different behaviour: path-tree would match "/src/{*filepath}"
        "/src/" => None
        "/src/some/file.png" => {
            route: "/src/{*filepath}",
            data: 4,
            params: {
                "filepath" => "some/file.png"
            }
        }
        "/search/someth!ng+in+ünìcodé" => {
            route: "/search/{query}",
            data: 9,
            params: {
                "query" => "someth!ng+in+ünìcodé"
            }
        }
        "/user_rust" => {
            route: "/user_{name}",
            data: 11,
            params: {
                "name" => "rust"
            }
        }
        "/user_rust/about" => {
            route: "/user_{name}/about",
            data: 12,
            params: {
                "name" => "rust"
            }
        }
        "/files/js/inc/framework.js" => {
            route: "/files/{dir}/{*filepath}",
            data: 14,
            params: {
                "dir" => "js",
                "filepath" => "inc/framework.js"
            }
        }
        "/info/gordon/public" => {
            route: "/info/{user}/public",
            data: 18,
            params: {
                "user" => "gordon"
            }
        }
        "/info/gordon/project/rust" => {
            route: "/info/{user}/project/{project}",
            data: 19,
            params: {
                "user" => "gordon",
                "project" => "rust"
            }
        }
    });

    Ok(())
}

#[test]
fn single_named_parameter() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users/{id}", 0)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /users/
       ╰─ {id} ○
    "#);

    assert_router_matches!(router, {
        "/" => None
        "/users/gordon" => {
            route: "/users/{id}",
            data: 0,
            params: {
                "id" => "gordon"
            }
        }
        "/users/you" => {
            route: "/users/{id}",
            data: 0,
            params: {
                "id" => "you"
            }
        }
        "/users/gordon/profile" => None
        "/users/" => None
        "/users" => None
    });

    Ok(())
}

#[test]
#[ignore = "undecided on behaviour"]
fn repeated_single_named_param() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users/{id}", 0)?;
    router.insert("/users/{user_id}", 1)?;

    insta::assert_snapshot!(router, @"");

    // FIXME: Currently we match the first route, since it was inserted first.
    // Possibly we'd be better off erroring here, since it's ambiguous.
    assert_router_matches!(router, {
        "/users/gordon" => {
            route: "/users/{user_id}",
            data: 1,
            params: {
                "user_id" => "gordon"
            }
        }
    });

    Ok(())
}

#[test]
fn static_and_named_parameter() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/a/b/c", "/a/b/c")?;
    router.insert("/a/c/d", "/a/c/d")?;
    router.insert("/a/c/a", "/a/c/a")?;
    router.insert("/{id}/c/e", "/{id}/c/e")?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ a/
       │  ├─ b/c ○
       │  ╰─ c/
       │     ├─ a ○
       │     ╰─ d ○
       ╰─ {id}
          ╰─ /c/e ○
    "#);

    assert_router_matches!(router, {
        "/" => None
        "/a/b/c" => {
            route: "/a/b/c",
            data: "/a/b/c"
        }
        "/a/c/d" => {
            route: "/a/c/d",
            data: "/a/c/d"
        }
        "/a/c/a" => {
            route: "/a/c/a",
            data: "/a/c/a"
        }
        "/a/c/e" => {
            route: "/{id}/c/e",
            data: "/{id}/c/e",
            params: {
                "id" => "a"
            }
        }
    });

    Ok(())
}

#[test]
fn multi_named_parameters() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{lang}/{keyword}", true)?;
    router.insert("/{id}", true)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ {id} ○
       ╰─ {lang}
          ╰─ /
             ╰─ {keyword} ○
    "#);

    assert_router_matches!(router, {
        "/" => None
        "/rust/" => None
        "/rust/let/" => None
        "/rust/let/const" => None
        "/rust/let" => {
            route: "/{lang}/{keyword}",
            data: true,
            params: {
                "lang" => "rust",
                "keyword" => "let"
            }
        }
        "/rust" => {
            route: "/{id}",
            data: true,
            params: {
                "id" => "rust"
            }
        }
    });

    Ok(())
}

#[test]
fn catch_all_parameter() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/src/{*filepath}", "* files")?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /src/
       ╰─ {*filepath} ○
    "#);

    assert_router_matches!(router, {
        "/src" => None
        // NOTE: Different behaviour: path-tree would match "/src/{*filepath}"
        "/src/" => None
        "/src/somefile.rs" => {
            route: "/src/{*filepath}",
            data: "* files",
            params: {
                "filepath" => "somefile.rs"
            }
        }
        "/src/subdir/somefile.rs" => {
            route: "/src/{*filepath}",
            data: "* files",
            params: {
                "filepath" => "subdir/somefile.rs"
            }
        }
        "/src.rs" => None
        "/rust" => None
    });

    router.insert("/src/", "dir")?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /src/ ○
       ╰─ {*filepath} ○
    "#);

    assert_router_matches!(router, {
        "/src/" => {
            route: "/src/",
            data: "dir"
        }
    });

    Ok(())
}

#[test]
#[ignore = "wildcards not yet implemented"]
fn catch_all_parameter_with_prefix() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/commit_{*sha}", "* sha")?;
    router.insert("/commit/{sha}", "hex")?;
    router.insert("/commit/{sha0}/compare/{sha1}", "compare")?;
    router.insert("/src/", "dir")?;

    insta::assert_snapshot!(router, @"");

    assert_router_matches!(router, {
        "/src/" => {
            route: "/src/",
            data: "dir"
        }
        "/commit/123" => {
            route: "/commit/{sha}",
            data: "hex",
            params: {
                "sha" => "123"
            }
        }
        "/commit/123/compare/321" => {
            route: "/commit/{sha0}/compare/{sha1}",
            data: "compare",
            params: {
                "sha0" => "123",
                "sha1" => "321"
            }
        }
        "/commit" => None
        "/commit_" => {
            route: "/commit_{*sha}",
            data: "* sha",
            params: {
                "sha" => ""
            }
        }
        "/commit_/" => {
            route: "/commit_{*sha}",
            data: "* sha",
            params: {
                "sha" => "/"
            }
        }
        "/commit_/foo" => {
            route: "/commit_{*sha}",
            data: "* sha",
            params: {
                "sha" => "/foo"
            }
        }
        "/commit123" => None
        "/commit_123" => {
            route: "/commit_{*sha}",
            data: "* sha",
            params: {
                "sha" => "123"
            }
        }
        "/commit_123/" => {
            route: "/commit_{*sha}",
            data: "* sha",
            params: {
                "sha" => "123/"
            }
        }
        "/commit_123/foo" => {
            route: "/commit_{*sha}",
            data: "* sha",
            params: {
                "sha" => "123/foo"
            }
        }
    });

    Ok(())
}

#[test]
fn static_and_catch_all_parameter() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/a/b/c", "/a/b/c")?;
    router.insert("/a/c/d", "/a/c/d")?;
    router.insert("/a/c/a", "/a/c/a")?;
    router.insert("/a/{*c}", "/a/*c")?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /a/
       ├─ b/c ○
       ├─ c/
       │  ├─ a ○
       │  ╰─ d ○
       ╰─ {*c} ○
    "#);

    assert_router_matches!(router, {
        "/" => None
        "/a/b/c" => {
            route: "/a/b/c",
            data: "/a/b/c"
        }
        "/a/c/d" => {
            route: "/a/c/d",
            data: "/a/c/d"
        }
        "/a/c/a" => {
            route: "/a/c/a",
            data: "/a/c/a"
        }
        "/a/c/e" => {
            route: "/a/{*c}",
            data: "/a/*c",
            params: {
                "c" => "c/e"
            }
        }
    });

    Ok(())
}

#[test]
fn root_catch_all_parameter() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/", 1)?;
    router.insert("/{*wildcard}", 2)?;
    router.insert("/users/{*wildcard}", 3)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ / ○
       ├─ users/
       │  ╰─ {*wildcard} ○
       ╰─ {*wildcard} ○
    "#);

    assert_router_matches!(router, {
        "/" => {
            route: "/",
            data: 1
        }
        "/download" => {
            route: "/{*wildcard}",
            data: 2,
            params: {
                "wildcard" => "download"
            }
        }
        "/users/jordan" => {
            route: "/users/{*wildcard}",
            data: 3,
            params: {
                "wildcard" => "jordan"
            }
        }
    });

    Ok(())
}

#[test]
fn root_catch_all_parameter_1() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{*wildcard}", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ╰─ {*wildcard} ○
    "#);

    assert_router_matches!(router, {
        // NOTE: Different behaviour: path-tree would match "/{*wildcard}"
        "/" => None
        "/download" => {
            route: "/{*wildcard}",
            data: 1,
            params: {
                "wildcard" => "download"
            }
        }
        "/users/jordan" => {
            route: "/{*wildcard}",
            data: 1,
            params: {
                "wildcard" => "users/jordan"
            }
        }
    });

    router.insert("/", 0)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ / ○
       ╰─ {*wildcard} ○
    "#);

    assert_router_matches!(router, {
        "/" => {
            route: "/",
            data: 0
        }
    });

    Ok(())
}

#[test]
fn test_named_routes_with_non_ascii_paths() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/", 0)?;
    router.insert("/{*wildcard}", 1)?;
    router.insert("/matchme/{slug}/", 2)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ / ○
       ├─ matchme/
       │  ╰─ {slug}
       │     ╰─ / ○
       ╰─ {*wildcard} ○
    "#);

    assert_router_matches!(router, {
        "/matchme/abc-s-def/" => {
            route: "/matchme/{slug}/",
            data: 2,
            params: {
                "slug" => "abc-s-def"
            }
        }
        "/matchme/abc-ß-def/" => {
            route: "/matchme/{slug}/",
            data: 2,
            params: {
                "slug" => "abc-ß-def"
            }
        }
        "/matchme/abc-⭐-def/" => {
            route: "/matchme/{slug}/",
            data: 2,
            params: {
                "slug" => "abc-⭐-def"
            }
        }
        "/matchme/abc-def-ß/" => {
            route: "/matchme/{slug}/",
            data: 2,
            params: {
                "slug" => "abc-def-ß"
            }
        }
    });

    Ok(())
}

#[test]
fn test_named_wildcard_collide() -> Result<(), Box<dyn Error>> {
    let mut router = Router::<usize>::new();
    router.insert("/git/{org}/{repo}", 1)?;
    router.insert("/git/{*wildcard}", 2)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /git/
       ├─ {org}
       │  ╰─ /
       │     ╰─ {repo} ○
       ╰─ {*wildcard} ○
    "#);

    assert_router_matches!(router, {
        "/git/rust-lang/rust" => {
            route: "/git/{org}/{repo}",
            data: 1,
            params: {
                "org" => "rust-lang",
                "repo" => "rust"
            }
        }
        "/git/rust-lang" => {
            route: "/git/{*wildcard}",
            data: 2,
            params: {
                "wildcard" => "rust-lang"
            }
        }
    });

    Ok(())
}

#[test]
fn match_params() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/api/v1/{param}/{*wildcard}", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /api/v1/
       ╰─ {param}
          ╰─ /
             ╰─ {*wildcard} ○
    "#);

    assert_router_matches!(router, {
        "/api/v1/entity" => None
        // NOTE: Different behaviour: path-tree would match "/api/v1/{param}/{*wildcard}"
        "/api/v1/entity/" => None
        "/api/v1/entity/1" => {
            route: "/api/v1/{param}/{*wildcard}",
            data: 1,
            params: {
                "param" => "entity",
                "wildcard" => "1"
            }
        }
        "/api/v" => None
        "/api/v2" => None
        "/api/v1/" => None
        "/api/v1/entity/1/foo/bar" => {
            route: "/api/v1/{param}/{*wildcard}",
            data: 1,
            params: {
                "param" => "entity",
                "wildcard" => "1/foo/bar"
            }
        }
    });

    let mut router = Router::new();
    router.insert("/api/v1/{param}/{*plus}", 1)?;

    assert_router_matches!(router, {
        "/api/v1/entity" => None
        "/api/v1/entity/" => None
        "/api/v1/entity/1" => {
            route: "/api/v1/{param}/{*plus}",
            data: 1,
            params: {
                "param" => "entity",
                "plus" => "1"
            }
        }
        "/api/v" => None
        "/api/v2" => None
        "/api/v1/" => None
        "/api/v1/entity/1/foo/bar" => {
            route: "/api/v1/{param}/{*plus}",
            data: 1,
            params: {
                "param" => "entity",
                "plus" => "1/foo/bar"
            }
        }
    });

    let mut router = Router::new();
    router.insert("/v1/some/resource/name:customVerb", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /v1/some/resource/name:customVerb ○
    "#);

    assert_router_matches!(router, {
        "/v1/some/resource/name:customVerb" => {
            route: "/v1/some/resource/name:customVerb",
            data: 1
        }
        "/v1/some/resource/name:test" => None
    });

    let mut router = Router::new();
    router.insert("/v1/some/resource/{name}:customVerb", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /v1/some/resource/
       ╰─ {name}
          ╰─ :customVerb ○
    "#);

    assert_router_matches!(router, {
        "/v1/some/resource/test:customVerb" => {
            route: "/v1/some/resource/{name}:customVerb",
            data: 1,
            params: {
                "name" => "test"
            }
        }
        "/v1/some/resource/test:test" => None
    });

    let mut router = Router::new();
    router.insert("/api/v1/{*wildcard}", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /api/v1/
       ╰─ {*wildcard} ○
    "#);

    assert_router_matches!(router, {
        "/api/v1" => None
        // NOTE: Different behaviour: path-tree would match "/api/v1/{*wildcard}"
        "/api/v1/" => None
        "/api/v1/entity" => {
            route: "/api/v1/{*wildcard}",
            data: 1,
            params: {
                "wildcard" => "entity"
            }
        }
        "/api/v1/entity/1/2" => {
            route: "/api/v1/{*wildcard}",
            data: 1,
            params: {
                "wildcard" => "entity/1/2"
            }
        }
        "/api/v1/Entity/1/2" => {
            route: "/api/v1/{*wildcard}",
            data: 1,
            params: {
                "wildcard" => "Entity/1/2"
            }
        }
    });

    let mut router = Router::new();
    router.insert("/api/v1/{param}", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /api/v1/
       ╰─ {param} ○
    "#);

    assert_router_matches!(router, {
        "/api/v1" => None
        "/api/v1/" => None
        "/api/v1/entity" => {
            route: "/api/v1/{param}",
            data: 1,
            params: {
                "param" => "entity"
            }
        }
        "/api/v1/entity/1/2" => None
        "/api/v1/Entity/1/2" => None
    });

    let mut router = Router::new();
    router.insert("/api/v1/{param}/{param2}", 3)?;
    router.insert("/api/v1/{param}-{param2}", 1)?;
    router.insert("/api/v1/{param}~{param2}", 2)?;
    router.insert("/api/v1/{param}.{param2}", 4)?;
    router.insert("/api/v1/{param}_{param2}", 5)?;
    router.insert("/api/v1/{param}:{param2}", 6)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /api/v1/
       ╰─ {param}
          ├─ -
          │  ╰─ {param2} ○
          ├─ .
          │  ╰─ {param2} ○
          ├─ /
          │  ╰─ {param2} ○
          ├─ :
          │  ╰─ {param2} ○
          ├─ _
          │  ╰─ {param2} ○
          ╰─ ~
             ╰─ {param2} ○
    "#);

    assert_router_matches!(router, {
        "/api/v1/entity-entity2" => {
            route: "/api/v1/{param}-{param2}",
            data: 1,
            params: {
                "param" => "entity",
                "param2" => "entity2"
            }
        }
        "/api/v1/entity~entity2" => {
            route: "/api/v1/{param}~{param2}",
            data: 2,
            params: {
                "param" => "entity",
                "param2" => "entity2"
            }
        }
        "/api/v1/entity.entity2" => {
            route: "/api/v1/{param}.{param2}",
            data: 4,
            params: {
                "param" => "entity",
                "param2" => "entity2"
            }
        }
        "/api/v1/entity_entity2" => {
            route: "/api/v1/{param}_{param2}",
            data: 5,
            params: {
                "param" => "entity",
                "param2" => "entity2"
            }
        }
        "/api/v1/entity:entity2" => {
            route: "/api/v1/{param}:{param2}",
            data: 6,
            params: {
                "param" => "entity",
                "param2" => "entity2"
            }
        }
        "/api/v1/entity/entity2" => {
            route: "/api/v1/{param}/{param2}",
            data: 3,
            params: {
                "param" => "entity",
                "param2" => "entity2"
            }
        }
        "/api/v1" => None
        "/api/v1/" => None
        "/api/v1/test.pdf" => {
            route: "/api/v1/{param}.{param2}",
            data: 4,
            params: {
                "param" => "test",
                "param2" => "pdf"
            }
        }
    });

    let mut router = Router::new();
    router.insert("/api/v1/const", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /api/v1/const ○
    "#);

    assert_router_matches!(router, {
        "/api/v1/const" => {
            route: "/api/v1/const",
            data: 1
        }
        "/api/v1/cons" => None
        "/api/v1/conststatic" => None
        "/api/v1/let" => None
        "/api/v1/" => None
        "/api/v1" => None
    });

    let mut router = Router::new();
    router.insert("/api/{param}/fixedEnd", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /api/
       ╰─ {param}
          ╰─ /fixedEnd ○
    "#);

    assert_router_matches!(router, {
        "/api/abc/fixedEnd" => {
            route: "/api/{param}/fixedEnd",
            data: 1,
            params: {
                "param" => "abc"
            }
        }
        "/api/abc/def/fixedEnd" => None
    });

    let mut router = Router::new();
    router.insert("/shop/product/:{filter}/color:{color}/size:{size}", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /shop/product/:
       ╰─ {filter}
          ╰─ /color:
             ╰─ {color}
                ╰─ /size:
                   ╰─ {size} ○
    "#);

    assert_router_matches!(router, {
        "/shop/product/:test/color:blue/size:xs" => {
            route: "/shop/product/:{filter}/color:{color}/size:{size}",
            data: 1,
            params: {
                "filter" => "test",
                "color" => "blue",
                "size" => "xs"
            }
        }
        "/shop/product/test/color:blue/size:xs" => None
    });

    let mut router = Router::new();
    router.insert("/test{sign}{param}", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /test
       ╰─ {sign}
          ╰─ {param} ○
    "#);

    assert_router_matches!(router, {
        "/test-" => None
        "/test" => None
    });

    let mut router = Router::new();
    router.insert("/name{name}", 1)?;
    router.insert("/@{name}", 2)?;
    router.insert("/-{name}", 3)?;
    router.insert("/.{name}", 4)?;
    router.insert("/~{name}", 5)?;
    router.insert("/_{name}", 6)?;
    router.insert("/{name}", 7)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ name
       │  ╰─ {name} ○
       ├─ -
       │  ╰─ {name} ○
       ├─ .
       │  ╰─ {name} ○
       ├─ @
       │  ╰─ {name} ○
       ├─ _
       │  ╰─ {name} ○
       ├─ ~
       │  ╰─ {name} ○
       ╰─ {name} ○
    "#);

    assert_router_matches!(router, {
        "/name:john" => {
            route: "/name{name}",
            data: 1,
            params: {
                "name" => ":john"
            }
        }
        "/@john" => {
            route: "/@{name}",
            data: 2,
            params: {
                "name" => "john"
            }
        }
        "/-john" => {
            route: "/-{name}",
            data: 3,
            params: {
                "name" => "john"
            }
        }
        "/.john" => {
            route: "/.{name}",
            data: 4,
            params: {
                "name" => "john"
            }
        }
        "/~john" => {
            route: "/~{name}",
            data: 5,
            params: {
                "name" => "john"
            }
        }
        "/_john" => {
            route: "/_{name}",
            data: 6,
            params: {
                "name" => "john"
            }
        }
        "/john" => {
            route: "/{name}",
            data: 7,
            params: {
                "name" => "john"
            }
        }
    });

    let mut router = Router::new();
    router.insert("/api/v1/{param}/abc/{*wildcard}", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /api/v1/
       ╰─ {param}
          ╰─ /abc/
             ╰─ {*wildcard} ○
    "#);

    assert_router_matches!(router, {
        "/api/v1/well/abc/wildcard" => {
            route: "/api/v1/{param}/abc/{*wildcard}",
            data: 1,
            params: {
                "param" => "well",
                "wildcard" => "wildcard"
            }
        }
        // NOTE: Different behaviour: path-tree would match "/api/v1/{param}/abc/{*wildcard}
        "/api/v1/well/abc/" => None
        "/api/v1/well/abc" => None
        "/api/v1/well/ttt" => None
    });

    let mut router = Router::new();
    router.insert("/api/{*wildcard}/{param}", 1)?;

    assert_router_matches!(router, {
        "/api/test/abc" => {
            route: "/api/{*wildcard}/{param}",
            data: 1,
            params: {
                "wildcard" => "test",
                "param" => "abc"
            }
        }
        "/api/joker/batman/robin/1" => {
            route: "/api/{*wildcard}/{param}",
            data: 1,
            params: {
                "wildcard" => "joker/batman/robin",
                "param" => "1"
            }
        }
        // FIXME: This feels wrong. Maybe we should add a check for empty parts?
        "/api//joker" => {
            route: "/api/{*wildcard}/{param}",
            data: 1,
            params: {
                "wildcard" => "",
                "param" => "joker"
            }
        }
        "/api/joker" => None
        "/api/" => None
    });

    let mut router = Router::new();
    router.insert("/api/{*wildcard}/{param}/{param2}", 1)?;

    assert_router_matches!(router, {
        "/api/test/abc/1" => {
            route: "/api/{*wildcard}/{param}/{param2}",
            data: 1,
            params: {
                "wildcard" => "test",
                "param" => "abc",
                "param2" => "1"
            }
        }
        "/api/joker/batman" => None
        "/api/joker/batman-robin/1" => {
            route: "/api/{*wildcard}/{param}/{param2}",
            data: 1,
            params: {
                "wildcard" => "joker",
                "param" => "batman-robin",
                "param2" => "1"
            }
        }
        "/api/joker-batman-robin-1" => None
        "/api/test/abc" => None
        "/api/joker/batman/robin" => {
            route: "/api/{*wildcard}/{param}/{param2}",
            data: 1,
            params: {
                "wildcard" => "joker",
                "param" => "batman",
                "param2" => "robin"
            }
        }
        "/api/joker/batman/robin/1" => {
            route: "/api/{*wildcard}/{param}/{param2}",
            data: 1,
            params: {
                "wildcard" => "joker/batman",
                "param" => "robin",
                "param2" => "1"
            }
        }
        "/api/joker/batman/robin/1/2" => {
            route: "/api/{*wildcard}/{param}/{param2}",
            data: 1,
            params: {
                "wildcard" => "joker/batman/robin",
                "param" => "1",
                "param2" => "2"
            }
        }
        "/api" => None
        "/api/:test" => None
    });

    Ok(())
}

#[test]
fn basic() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/", 0)?;
    router.insert("/login", 1)?;
    router.insert("/signup", 2)?;
    router.insert("/settings", 3)?;
    router.insert("/settings/{page}", 4)?;
    router.insert("/{user}", 5)?;
    router.insert("/{user}/{repo}", 6)?;
    router.insert("/public/{*any}", 7)?;
    router.insert("/{org}/{repo}/releases/download/{tag}/{filename}.{ext}", 8)?;
    router.insert("/{org}/{repo}/tags/{day}-{month}-{year}", 9)?;
    router.insert("/{org}/{repo}/actions/{name}:{verb}", 10)?;
    router.insert("/{org}/{repo}/{page}", 11)?;
    router.insert("/{org}/{repo}/{*path}", 12)?;
    router.insert("/api/{*plus}", 13)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ / ○
       ├─ login ○
       ├─ public/
       │  ╰─ {*any} ○
       ├─ api/
       │  ╰─ {*plus} ○
       ├─ s
       │  ├─ ettings ○
       │  │  ╰─ /
       │  │     ╰─ {page} ○
       │  ╰─ ignup ○
       ├─ {user} ○
       │  ╰─ /
       │     ╰─ {repo} ○
       ╰─ {org}
          ╰─ /
             ╰─ {repo}
                ╰─ /
                   ├─ releases/download/
                   │  ╰─ {tag}
                   │     ╰─ /
                   │        ╰─ {filename}
                   │           ╰─ .
                   │              ╰─ {ext} ○
                   ├─ actions/
                   │  ╰─ {name}
                   │     ╰─ :
                   │        ╰─ {verb} ○
                   ├─ tags/
                   │  ╰─ {day}
                   │     ╰─ -
                   │        ╰─ {month}
                   │           ╰─ -
                   │              ╰─ {year} ○
                   ├─ {page} ○
                   ╰─ {*path} ○
    "#);

    assert_router_matches!(router, {
        "/" => {
            route: "/",
            data: 0
        }
        "/login" => {
            route: "/login",
            data: 1
        }
        "/settings/admin" => {
            route: "/settings/{page}",
            data: 4,
            params: {
                "page" => "admin"
            }
        }
        "/viz-rs" => {
            route: "/{user}",
            data: 5,
            params: {
                "user" => "viz-rs"
            }
        }
        "/viz-rs/path-tree" => {
            route: "/{user}/{repo}",
            data: 6,
            params: {
                "user" => "viz-rs",
                "repo" => "path-tree"
            }
        }
        "/rust-lang/rust-analyzer/releases/download/2022-09-12/rust-analyzer-aarch64-apple-darwin.gz" => {
            route: "/{org}/{repo}/releases/download/{tag}/{filename}.{ext}",
            data: 8,
            params: {
                "org" => "rust-lang",
                "repo" => "rust-analyzer",
                "tag" => "2022-09-12",
                "filename" => "rust-analyzer-aarch64-apple-darwin",
                "ext" => "gz"
            }
        }
        "/rust-lang/rust-analyzer/tags/2022-09-12" => {
            route: "/{org}/{repo}/tags/{day}-{month}-{year}",
            data: 9,
            params: {
                "org" => "rust-lang",
                "repo" => "rust-analyzer",
                "day" => "2022",
                "month" => "09",
                "year" => "12"
            }
        }
        "/rust-lang/rust-analyzer/actions/ci:bench" => {
            route: "/{org}/{repo}/actions/{name}:{verb}",
            data: 10,
            params: {
                "org" => "rust-lang",
                "repo" => "rust-analyzer",
                "name" => "ci",
                "verb" => "bench"
            }
        }
        "/rust-lang/rust-analyzer/stargazers" => {
            route: "/{org}/{repo}/{page}",
            data: 11,
            params: {
                "org" => "rust-lang",
                "repo" => "rust-analyzer",
                "page" => "stargazers"
            }
        }
        "/rust-lang/rust-analyzer/stargazers/404" => {
            route: "/{org}/{repo}/{*path}",
            data: 12,
            params: {
                "org" => "rust-lang",
                "repo" => "rust-analyzer",
                "path" => "stargazers/404"
            }
        }
        "/public/js/main.js" => {
            route: "/public/{*any}",
            data: 7,
            params: {
                "any" => "js/main.js"
            }
        }
        "/api/v1" => {
            route: "/api/{*plus}",
            data: 13,
            params: {
                "plus" => "v1"
            }
        }
    });

    Ok(())
}

#[test]
fn github_tree() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.insert("/", 0)?;
    router.insert("/api", 1)?;
    router.insert("/about", 2)?;
    router.insert("/login", 3)?;
    router.insert("/signup", 4)?;
    router.insert("/pricing", 5)?;

    router.insert("/features", 6)?;
    router.insert("/features/actions", 600)?;
    router.insert("/features/packages", 601)?;
    router.insert("/features/security", 602)?;
    router.insert("/features/codespaces", 603)?;
    router.insert("/features/copilot", 604)?;
    router.insert("/features/code-review", 605)?;
    router.insert("/features/issues", 606)?;
    router.insert("/features/discussions", 607)?;

    router.insert("/enterprise", 7)?;
    router.insert("/team", 8)?;
    router.insert("/customer-stories", 9)?;
    router.insert("/sponsors", 10)?;
    router.insert("/readme", 11)?;
    router.insert("/topics", 12)?;
    router.insert("/trending", 13)?;
    router.insert("/collections", 14)?;
    router.insert("/search", 15)?;
    router.insert("/pulls", 16)?;
    router.insert("/issues", 17)?;
    router.insert("/marketplace", 18)?;
    router.insert("/explore", 19)?;

    router.insert("/sponsors/explore", 100)?;
    router.insert("/sponsors/accounts", 101)?;
    router.insert("/sponsors/{repo}", 102)?;
    router.insert("/sponsors/{repo}/{user}", 103)?;
    router.insert("/sponsors/{repo}/{*plus}", 104)?;
    router.insert("/sponsors/{repo}/issues/{*path}", 106)?;
    router.insert("/sponsors/{repo}/{*plus}/{file}", 107)?;
    router.insert("/sponsors/{repo}/{*plus}/{filename}.{ext}", 108)?;

    router.insert("/about/careers", 200)?;
    router.insert("/about/press", 201)?;
    router.insert("/about/diversity", 202)?;

    router.insert("/settings", 20)?;
    router.insert("/settings/admin", 2000)?;
    router.insert("/settings/appearance", 2001)?;
    router.insert("/settings/accessibility", 2002)?;
    router.insert("/settings/notifications", 2003)?;

    router.insert("/settings/billing", 2004)?;
    router.insert("/settings/billing/plans", 2005)?;
    router.insert("/settings/security", 2006)?;
    router.insert("/settings/keys", 2007)?;
    router.insert("/settings/organizations", 2008)?;

    router.insert("/settings/blocked_users", 2009)?;
    router.insert("/settings/interaction_limits", 2010)?;
    router.insert("/settings/code_review_limits", 2011)?;

    router.insert("/settings/repositories", 2012)?;
    router.insert("/settings/codespaces", 2013)?;
    router.insert("/settings/deleted_packages", 2014)?;
    router.insert("/settings/copilot", 2015)?;
    router.insert("/settings/pages", 2016)?;
    router.insert("/settings/replies", 2017)?;

    router.insert("/settings/security_analysis", 2018)?;

    router.insert("/settings/installations", 2019)?;
    router.insert("/settings/reminders", 2020)?;

    router.insert("/settings/security-log", 2021)?;
    router.insert("/settings/sponsors-log", 2022)?;

    router.insert("/settings/apps", 2023)?;
    router.insert("/settings/developers", 2024)?;
    router.insert("/settings/tokens", 2025)?;

    router.insert("/404", 21)?;
    router.insert("/500", 22)?;
    router.insert("/503", 23)?;

    router.insert("/{org}", 24)?;
    router.insert("/{org}/{repo}", 2400)?;
    router.insert("/{org}/{repo}/issues", 2410)?;
    router.insert("/{org}/{repo}/issues/{id}", 2411)?;
    router.insert("/{org}/{repo}/issues/new", 2412)?;
    router.insert("/{org}/{repo}/pulls", 2420)?;
    router.insert("/{org}/{repo}/pull/{id}", 2421)?;
    router.insert("/{org}/{repo}/compare", 2422)?;
    router.insert("/{org}/{repo}/discussions", 2430)?;
    router.insert("/{org}/{repo}/discussions/{id}", 2431)?;
    router.insert("/{org}/{repo}/actions", 2440)?;
    router.insert("/{org}/{repo}/actions/workflows/{id}", 2441)?;
    router.insert("/{org}/{repo}/actions/runs/{id}", 2442)?;
    router.insert("/{org}/{repo}/wiki", 2450)?;
    router.insert("/{org}/{repo}/wiki/{id}", 2451)?;
    router.insert("/{org}/{repo}/security", 2460)?;
    router.insert("/{org}/{repo}/security/policy", 2461)?;
    router.insert("/{org}/{repo}/security/advisories", 2462)?;
    router.insert("/{org}/{repo}/pulse", 2470)?;
    router.insert("/{org}/{repo}/graphs/contributors", 2480)?;
    router.insert("/{org}/{repo}/graphs/commit-activity", 2481)?;
    router.insert("/{org}/{repo}/graphs/code-frequency", 2482)?;
    router.insert("/{org}/{repo}/community", 2490)?;
    router.insert("/{org}/{repo}/network", 2491)?;
    router.insert("/{org}/{repo}/network/dependencies", 2492)?;
    router.insert("/{org}/{repo}/network/dependents", 2493)?;
    router.insert("/{org}/{repo}/network/members", 2494)?;
    router.insert("/{org}/{repo}/stargazers", 2495)?;
    router.insert("/{org}/{repo}/stargazers/yoou_know", 2496)?;
    router.insert("/{org}/{repo}/watchers", 2497)?;
    router.insert("/{org}/{repo}/releases", 2498)?;
    router.insert("/{org}/{repo}/releases/tag/{id}", 2499)?;
    router.insert("/{org}/{repo}/tags", 2500)?;
    router.insert("/{org}/{repo}/tags/{id}", 2501)?;
    router.insert("/{org}/{repo}/tree/{id}", 2502)?;
    router.insert("/{org}/{repo}/commit/{id}", 2503)?;

    router.insert("/new", 2504)?;
    router.insert("/new/import", 2505)?;
    router.insert("/organizations/new", 2506)?;
    router.insert("/organizations/plan", 2507)?;

    router.insert("/{org}/{repo}/{*path}", 3000)?;
    router.insert("/{org}/{repo}/releases/{*path}", 3001)?;
    router.insert(
        "/{org}/{repo}/releases/download/{tag}/{filename}.{ext}",
        3002,
    )?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ / ○
       ├─ organizations/
       │  ├─ plan ○
       │  ╰─ new ○
       ├─ marketplace ○
       ├─ features ○
       │  ╰─ /
       │     ├─ discussions ○
       │     ├─ packages ○
       │     ├─ security ○
       │     ├─ actions ○
       │     ├─ issues ○
       │     ╰─ co
       │        ├─ de
       │        │  ├─ -review ○
       │        │  ╰─ spaces ○
       │        ╰─ pilot ○
       ├─ issues ○
       ├─ readme ○
       ├─ login ○
       ├─ 404 ○
       ├─ new ○
       │  ╰─ /import ○
       ├─ 50
       │  ├─ 0 ○
       │  ╰─ 3 ○
       ├─ a
       │  ├─ bout ○
       │  │  ╰─ /
       │  │     ├─ diversity ○
       │  │     ├─ careers ○
       │  │     ╰─ press ○
       │  ╰─ pi ○
       ├─ c
       │  ├─ ustomer-stories ○
       │  ╰─ ollections ○
       ├─ e
       │  ├─ nterprise ○
       │  ╰─ xplore ○
       ├─ p
       │  ├─ ricing ○
       │  ╰─ ulls ○
       ├─ s
       │  ├─ ponsors ○
       │  │  ╰─ /
       │  │     ├─ accounts ○
       │  │     ├─ explore ○
       │  │     ╰─ {repo} ○
       │  │        ╰─ /
       │  │           ├─ issues/
       │  │           │  ╰─ {*path} ○
       │  │           ├─ {user} ○
       │  │           ├─ {*plus}
       │  │           │  ╰─ /
       │  │           │     ├─ {file} ○
       │  │           │     ╰─ {filename}
       │  │           │        ╰─ .
       │  │           │           ╰─ {ext} ○
       │  │           ╰─ {*plus} ○
       │  ├─ e
       │  │  ├─ ttings ○
       │  │  │  ╰─ /
       │  │  │     ├─ notifications ○
       │  │  │     ├─ organizations ○
       │  │  │     ├─ tokens ○
       │  │  │     ├─ pages ○
       │  │  │     ├─ keys ○
       │  │  │     ├─ co
       │  │  │     │  ├─ pilot ○
       │  │  │     │  ╰─ de
       │  │  │     │     ├─ _review_limits ○
       │  │  │     │     ╰─ spaces ○
       │  │  │     ├─ de
       │  │  │     │  ├─ leted_packages ○
       │  │  │     │  ╰─ velopers ○
       │  │  │     ├─ in
       │  │  │     │  ├─ teraction_limits ○
       │  │  │     │  ╰─ stallations ○
       │  │  │     ├─ re
       │  │  │     │  ├─ minders ○
       │  │  │     │  ╰─ p
       │  │  │     │     ├─ ositories ○
       │  │  │     │     ╰─ lies ○
       │  │  │     ├─ a
       │  │  │     │  ├─ ccessibility ○
       │  │  │     │  ├─ pp
       │  │  │     │  │  ├─ earance ○
       │  │  │     │  │  ╰─ s ○
       │  │  │     │  ╰─ dmin ○
       │  │  │     ├─ b
       │  │  │     │  ├─ locked_users ○
       │  │  │     │  ╰─ illing ○
       │  │  │     │     ╰─ /plans ○
       │  │  │     ╰─ s
       │  │  │        ├─ ponsors-log ○
       │  │  │        ╰─ ecurity ○
       │  │  │           ├─ _analysis ○
       │  │  │           ╰─ -log ○
       │  │  ╰─ arch ○
       │  ╰─ ignup ○
       ├─ t
       │  ├─ rending ○
       │  ├─ opics ○
       │  ╰─ eam ○
       ╰─ {org} ○
          ╰─ /
             ╰─ {repo} ○
                ╰─ /
                   ├─ discussions ○
                   │  ╰─ /
                   │     ╰─ {id} ○
                   ├─ releases ○
                   │  ╰─ /
                   │     ├─ download/
                   │     │  ╰─ {tag}
                   │     │     ╰─ /
                   │     │        ╰─ {filename}
                   │     │           ╰─ .
                   │     │              ╰─ {ext} ○
                   │     ├─ tag/
                   │     │  ╰─ {id} ○
                   │     ╰─ {*path} ○
                   ├─ actions ○
                   │  ╰─ /
                   │     ├─ workflows/
                   │     │  ╰─ {id} ○
                   │     ╰─ runs/
                   │        ╰─ {id} ○
                   ├─ network ○
                   │  ╰─ /
                   │     ├─ members ○
                   │     ╰─ dependen
                   │        ├─ cies ○
                   │        ╰─ ts ○
                   ├─ issues ○
                   │  ╰─ /
                   │     ├─ new ○
                   │     ╰─ {id} ○
                   ├─ t
                   │  ├─ ags ○
                   │  │  ╰─ /
                   │  │     ╰─ {id} ○
                   │  ╰─ ree/
                   │     ╰─ {id} ○
                   ├─ graphs/co
                   │  ├─ mmit-activity ○
                   │  ├─ de-frequency ○
                   │  ╰─ ntributors ○
                   ├─ com
                   │  ├─ m
                   │  │  ├─ unity ○
                   │  │  ╰─ it/
                   │  │     ╰─ {id} ○
                   │  ╰─ pare ○
                   ├─ pul
                   │  ├─ se ○
                   │  ╰─ l
                   │     ├─ s ○
                   │     ╰─ /
                   │        ╰─ {id} ○
                   ├─ s
                   │  ├─ targazers ○
                   │  │  ╰─ /yoou_know ○
                   │  ╰─ ecurity ○
                   │     ╰─ /
                   │        ├─ advisories ○
                   │        ╰─ policy ○
                   ├─ w
                   │  ├─ atchers ○
                   │  ╰─ iki ○
                   │     ╰─ /
                   │        ╰─ {id} ○
                   ╰─ {*path} ○
    "#);

    assert_router_matches!(router, {
        "/rust-lang/rust" => {
            route: "/{org}/{repo}",
            data: 2400,
            params: {
                "org" => "rust-lang",
                "repo" => "rust"
            }
        }
        "/settings" => {
            route: "/settings",
            data: 20
        }
        "/rust-lang/rust/actions/runs/1" => {
            route: "/{org}/{repo}/actions/runs/{id}",
            data: 2442,
            params: {
                "org" => "rust-lang",
                "repo" => "rust",
                "id" => "1"
            }
        }
        // NOTE: Different behaviour: path-tree would match "/{org}/{repo}/{*path}"
        "/rust-lang/rust/" => None
        "/rust-lang/rust/any" => {
            route: "/{org}/{repo}/{*path}",
            data: 3000,
            params: {
                "org" => "rust-lang",
                "repo" => "rust",
                "path" => "any"
            }
        }
        // NOTE: Different behaviour: path-tree would match "/{org}/{repo}/{*path}"
        "/rust-lang/rust/releases/" => {
            route: "/{org}/{repo}/{*path}",
            data: 3000,
            params: {
                "org" => "rust-lang",
                "repo" => "rust",
                "path" => "releases/"
            }
        }
        "/rust-lang/rust-analyzer/releases/download/2022-09-12/rust-analyzer-aarch64-apple-darwin.gz" => {
            route: "/{org}/{repo}/releases/download/{tag}/{filename}.{ext}",
            data: 3002,
            params: {
                "org" => "rust-lang",
                "repo" => "rust-analyzer",
                "tag" => "2022-09-12",
                "filename" => "rust-analyzer-aarch64-apple-darwin",
                "ext" => "gz"
            }
        }
    });

    Ok(())
}

#[test]
fn cloneable() {
    let router = Router::<usize>::new();
    assert_eq!(
        <dyn std::any::Any>::type_id(&router),
        <dyn std::any::Any>::type_id(&router.clone())
    );
}

#[test]
fn test_dots_no_ext() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{name}", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ╰─ {name} ○
    "#);

    assert_router_matches!(router, {
        "/abc.xyz.123" => {
            route: "/{name}",
            data: 1,
            params: {
                "name" => "abc.xyz.123"
            }
        }
    });

    Ok(())
}

#[test]
#[ignore = "we don't support 'one or more' or 'inline wildcard' logic"]
fn test_dots_ext() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{+name}.123", 2)?;
    router.insert("/{*name}.123.456", 1)?;

    insta::assert_snapshot!(router, @"");

    assert_router_matches!(router, {
        "/abc.xyz.123" => {
            route: "/{+name}.123",
            data: 2,
            params: {
                "name" => "abc.xyz"
            }
        }
        "/abc.xyz.123.456" => {
            route: "/{*name}.123.456",
            data: 1,
            params: {
                "name" => "abc.xyz"
            }
        }
    });

    Ok(())
}

#[test]
fn test_dots_ext_no_qualifier() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{name}.js", 2)?;
    router.insert("/{name}.js.gz", 1)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ╰─ {name}
          ╰─ .js ○
             ╰─ .gz ○
    "#);

    assert_router_matches!(router, {
        "/node.js" => {
            route: "/{name}.js",
            data: 2,
            params: {
                "name" => "node"
            }
        }
        "/path.lib.js" => {
            route: "/{name}.js",
            data: 2,
            params: {
                "name" => "path.lib"
            }
        }
        "/node.js.js" => {
            route: "/{name}.js",
            data: 2,
            params: {
                "name" => "node.js"
            }
        }
        "/node.js.gz" => {
            route: "/{name}.js.gz",
            data: 1,
            params: {
                "name" => "node"
            }
        }
        "/node.js.gz.js.gz" => {
            route: "/{name}.js.gz",
            data: 1,
            params: {
                "name" => "node.js.gz"
            }
        }
    });

    Ok(())
}
