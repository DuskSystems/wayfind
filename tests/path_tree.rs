//! Tests sourced from `path-tree` (MIT OR Apache-2.0)
//! <https://github.com/viz-rs/path-tree/blob/v0.8.1/tests/tree.rs>

#![allow(clippy::too_many_lines, clippy::cognitive_complexity)]

use wayfind::{assert_router_matches, router::Router};

#[test]
fn statics() {
    let mut router = Router::new();
    router.insert("/", 0);
    router.insert("/hi", 1);
    router.insert("/contact", 2);
    router.insert("/co", 3);
    router.insert("/c", 4);
    router.insert("/a", 5);
    router.insert("/ab", 6);
    router.insert("/doc/", 7);
    router.insert("/doc/go_faq.html", 8);
    router.insert("/doc/go1.html", 9);
    router.insert("/α", 10);
    router.insert("/β", 11);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ / [0]
       ├─ hi [1]
       ├─ c [4]
       │  ╰─ o [3]
       │     ╰─ ntact [2]
       ├─ a [5]
       │  ╰─ b [6]
       ├─ doc/ [7]
       │     ╰─ go
       │         ├─ _faq.html [8]
       │         ╰─ 1.html [9]
       ╰─ �
            ├─ � [10]
            ╰─ � [11]
    "###);

    assert_router_matches!(router, {
        "/" => {
            path: "/",
            value: 0
        }
        "/hi" => {
            path: "/hi",
            value: 1
        }
        "/contact" => {
            path: "/contact",
            value: 2
        }
        "/co" => {
            path: "/co",
            value: 3
        }
        "/c" => {
            path: "/c",
            value: 4
        }
        "/a" => {
            path: "/a",
            value: 5
        }
        "/ab" => {
            path: "/ab",
            value: 6
        }
        "/doc/" => {
            path: "/doc/",
            value: 7
        }
        "/doc/go_faq.html" => {
            path: "/doc/go_faq.html",
            value: 8
        }
        "/doc/go1.html" => {
            path: "/doc/go1.html",
            value: 9
        }
        "/α" => {
            path: "/α",
            value: 10
        }
        "/β" => {
            path: "/β",
            value: 11
        }
    });
}

#[test]
fn wildcards() {
    let mut router = Router::new();
    router.insert("/", 0);
    router.insert("/cmd/{tool}/{sub}", 1);
    router.insert("/cmd/{tool}/", 2);
    router.insert("/cmd/vet", 3);
    router.insert("/src/{filepath:*}", 4);
    router.insert("/src1/", 5);
    router.insert("/src1/{filepath:*}", 6);
    // NOTE: We don't support 'inline wildcard' logic.
    // router.insert("/src2{filepath:*}", 7);
    router.insert("/search/", 8);
    router.insert("/search/{query}", 9);
    router.insert("/search/invalid", 10);
    router.insert("/user_{name}", 11);
    router.insert("/user_{name}/about", 12);
    router.insert("/user_x", 13);
    router.insert("/files/{dir}/{filepath:*}", 14);
    router.insert("/doc/", 15);
    router.insert("/doc/rust_faq.html", 16);
    router.insert("/doc/rust1.html", 17);
    router.insert("/info/{user}/public", 18);
    router.insert("/info/{user}/project/{project}", 19);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ / [0]
       ├─ cmd/
       │     ├─ vet [3]
       │     ╰─ {tool}
       │             ╰─ / [2]
       │                ╰─ {sub} [1]
       ├─ s
       │  ├─ rc
       │  │   ├─ /
       │  │   │  ╰─ {filepath:*} [4]
       │  │   ╰─ 1/ [5]
       │  │       ╰─ {filepath:*} [6]
       │  ╰─ earch/ [8]
       │          ├─ invalid [10]
       │          ╰─ {query} [9]
       ├─ user_
       │      ├─ x [13]
       │      ╰─ {name} [11]
       │              ╰─ /about [12]
       ├─ files/
       │       ╰─ {dir}
       │              ╰─ /
       │                 ╰─ {filepath:*} [14]
       ├─ doc/ [15]
       │     ╰─ rust
       │           ├─ _faq.html [16]
       │           ╰─ 1.html [17]
       ╰─ info/
              ╰─ {user}
                      ╰─ /p
                          ├─ ublic [18]
                          ╰─ roject/
                                   ╰─ {project} [19]
    "###);

    assert_router_matches!(router, {
        "/" => {
            path: "/",
            value: 0
        }
        "/cmd/test/" => {
            path: "/cmd/{tool}/",
            value: 2,
            params: {
                "tool" => "test"
            }
        }
        "/cmd/test/3" => {
            path: "/cmd/{tool}/{sub}",
            value: 1,
            params: {
                "tool" => "test",
                "sub" => "3"
            }
        }
        // NOTE: Different behaviour: path-tree would match "/src/{filepath:*}"
        "/src/" => None
        "/src/some/file.png" => {
            path: "/src/{filepath:*}",
            value: 4,
            params: {
                "filepath" => "some/file.png"
            }
        }
        "/search/someth!ng+in+ünìcodé" => {
            path: "/search/{query}",
            value: 9,
            params: {
                "query" => "someth!ng+in+ünìcodé"
            }
        }
        "/user_rust" => {
            path: "/user_{name}",
            value: 11,
            params: {
                "name" => "rust"
            }
        }
        "/user_rust/about" => {
            path: "/user_{name}/about",
            value: 12,
            params: {
                "name" => "rust"
            }
        }
        "/files/js/inc/framework.js" => {
            path: "/files/{dir}/{filepath:*}",
            value: 14,
            params: {
                "dir" => "js",
                "filepath" => "inc/framework.js"
            }
        }
        "/info/gordon/public" => {
            path: "/info/{user}/public",
            value: 18,
            params: {
                "user" => "gordon"
            }
        }
        "/info/gordon/project/rust" => {
            path: "/info/{user}/project/{project}",
            value: 19,
            params: {
                "user" => "gordon",
                "project" => "rust"
            }
        }
    });
}

#[test]
fn single_named_parameter() {
    let mut router = Router::new();
    router.insert("/users/{id}", 0);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /users/
             ╰─ {id} [0]
    "###);

    assert_router_matches!(router, {
        "/" => None
        "/users/gordon" => {
            path: "/users/{id}",
            value: 0,
            params: {
                "id" => "gordon"
            }
        }
        "/users/you" => {
            path: "/users/{id}",
            value: 0,
            params: {
                "id" => "you"
            }
        }
        "/users/gordon/profile" => None
        "/users/" => None
        "/users" => None
    });
}

#[test]
#[ignore = "undecided on behaviour"]
fn repeated_single_named_param() {
    let mut router = Router::new();
    router.insert("/users/{id}", 0);
    router.insert("/users/{user_id}", 1);

    insta::assert_snapshot!(router, @"");

    // FIXME: Currently we match the first route, since it was inserted first.
    // Possibly we'd be better off erroring here, since it's ambiguous.
    assert_router_matches!(router, {
        "/users/gordon" => {
            path: "/users/{user_id}",
            value: 1,
            params: {
                "user_id" => "gordon"
            }
        }
    });
}

#[test]
fn static_and_named_parameter() {
    let mut router = Router::new();
    router.insert("/a/b/c", "/a/b/c");
    router.insert("/a/c/d", "/a/c/d");
    router.insert("/a/c/a", "/a/c/a");
    router.insert("/{id}/c/e", "/{id}/c/e");

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ├─ a/
       │   ├─ b/c [/a/b/c]
       │   ╰─ c/
       │       ├─ d [/a/c/d]
       │       ╰─ a [/a/c/a]
       ╰─ {id}
             ╰─ /c/e [/{id}/c/e]
    "###);

    assert_router_matches!(router, {
        "/" => None
        "/a/b/c" => {
            path: "/a/b/c",
            value: "/a/b/c"
        }
        "/a/c/d" => {
            path: "/a/c/d",
            value: "/a/c/d"
        }
        "/a/c/a" => {
            path: "/a/c/a",
            value: "/a/c/a"
        }
        "/a/c/e" => {
            path: "/{id}/c/e",
            value: "/{id}/c/e",
            params: {
                "id" => "a"
            }
        }
    });
}

#[test]
fn multi_named_parameters() {
    let mut router = Router::new();
    router.insert("/{lang}/{keyword}", true);
    router.insert("/{id}", true);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ├─ {lang}
       │       ╰─ /
       │          ╰─ {keyword} [true]
       ╰─ {id} [true]
    "###);

    assert_router_matches!(router, {
        "/" => None
        "/rust/" => None
        "/rust/let/" => None
        "/rust/let/const" => None
        "/rust/let" => {
            path: "/{lang}/{keyword}",
            value: true,
            params: {
                "lang" => "rust",
                "keyword" => "let"
            }
        }
        "/rust" => {
            path: "/{id}",
            value: true,
            params: {
                "id" => "rust"
            }
        }
    });
}

#[test]
fn catch_all_parameter() {
    let mut router = Router::new();
    router.insert("/src/{filepath:*}", "* files");

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /src/
           ╰─ {filepath:*} [* files]
    "###);

    assert_router_matches!(router, {
        "/src" => None
        // NOTE: Different behaviour: path-tree would match "/src/{filepath:*}"
        "/src/" => None
        "/src/somefile.rs" => {
            path: "/src/{filepath:*}",
            value: "* files",
            params: {
                "filepath" => "somefile.rs"
            }
        }
        "/src/subdir/somefile.rs" => {
            path: "/src/{filepath:*}",
            value: "* files",
            params: {
                "filepath" => "subdir/somefile.rs"
            }
        }
        "/src.rs" => None
        "/rust" => None
    });

    router.insert("/src/", "dir");

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /src/ [dir]
           ╰─ {filepath:*} [* files]
    "###);

    assert_router_matches!(router, {
        "/src/" => {
            path: "/src/",
            value: "dir"
        }
    });
}

#[test]
#[ignore = "wildcards not yet implemented"]
fn catch_all_parameter_with_prefix() {
    let mut router = Router::new();
    router.insert("/commit_{sha:*}", "* sha");
    router.insert("/commit/{sha}", "hex");
    router.insert("/commit/{sha0}/compare/{sha1}", "compare");
    router.insert("/src/", "dir");

    insta::assert_snapshot!(router, @"");

    assert_router_matches!(router, {
        "/src/" => {
            path: "/src/",
            value: "dir"
        }
        "/commit/123" => {
            path: "/commit/{sha}",
            value: "hex",
            params: {
                "sha" => "123"
            }
        }
        "/commit/123/compare/321" => {
            path: "/commit/{sha0}/compare/{sha1}",
            value: "compare",
            params: {
                "sha0" => "123",
                "sha1" => "321"
            }
        }
        "/commit" => None
        "/commit_" => {
            path: "/commit_{sha:*}",
            value: "* sha",
            params: {
                "sha" => ""
            }
        }
        "/commit_/" => {
            path: "/commit_{sha:*}",
            value: "* sha",
            params: {
                "sha" => "/"
            }
        }
        "/commit_/foo" => {
            path: "/commit_{sha:*}",
            value: "* sha",
            params: {
                "sha" => "/foo"
            }
        }
        "/commit123" => None
        "/commit_123" => {
            path: "/commit_{sha:*}",
            value: "* sha",
            params: {
                "sha" => "123"
            }
        }
        "/commit_123/" => {
            path: "/commit_{sha:*}",
            value: "* sha",
            params: {
                "sha" => "123/"
            }
        }
        "/commit_123/foo" => {
            path: "/commit_{sha:*}",
            value: "* sha",
            params: {
                "sha" => "123/foo"
            }
        }
    });
}

#[test]
fn static_and_catch_all_parameter() {
    let mut router = Router::new();
    router.insert("/a/b/c", "/a/b/c");
    router.insert("/a/c/d", "/a/c/d");
    router.insert("/a/c/a", "/a/c/a");
    router.insert("/a/{c:*}", "/a/*c");

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /a/
         ├─ b/c [/a/b/c]
         ├─ c/
         │   ├─ d [/a/c/d]
         │   ╰─ a [/a/c/a]
         ╰─ {c:*} [/a/*c]
    "###);

    assert_router_matches!(router, {
        "/" => None
        "/a/b/c" => {
            path: "/a/b/c",
            value: "/a/b/c"
        }
        "/a/c/d" => {
            path: "/a/c/d",
            value: "/a/c/d"
        }
        "/a/c/a" => {
            path: "/a/c/a",
            value: "/a/c/a"
        }
        "/a/c/e" => {
            path: "/a/{c:*}",
            value: "/a/*c",
            params: {
                "c" => "c/e"
            }
        }
    });
}

#[test]
fn root_catch_all_parameter() {
    let mut router = Router::new();
    router.insert("/", 1);
    router.insert("/{wildcard:*}", 2);
    router.insert("/users/{wildcard:*}", 3);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ / [1]
       ├─ users/
       │       ╰─ {wildcard:*} [3]
       ╰─ {wildcard:*} [2]
    "###);

    assert_router_matches!(router, {
        "/" => {
            path: "/",
            value: 1
        }
        "/download" => {
            path: "/{wildcard:*}",
            value: 2,
            params: {
                "wildcard" => "download"
            }
        }
        "/users/jordan" => {
            path: "/users/{wildcard:*}",
            value: 3,
            params: {
                "wildcard" => "jordan"
            }
        }
    });
}

#[test]
fn root_catch_all_parameter_1() {
    let mut router = Router::new();
    router.insert("/{wildcard:*}", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ╰─ {wildcard:*} [1]
    "###);

    assert_router_matches!(router, {
        // NOTE: Different behaviour: path-tree would match "/wildcard:*"
        "/" => None
        "/download" => {
            path: "/{wildcard:*}",
            value: 1,
            params: {
                "wildcard" => "download"
            }
        }
        "/users/jordan" => {
            path: "/{wildcard:*}",
            value: 1,
            params: {
                "wildcard" => "users/jordan"
            }
        }
    });

    router.insert("/", 0);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ / [0]
       ╰─ {wildcard:*} [1]
    "###);

    assert_router_matches!(router, {
        "/" => {
            path: "/",
            value: 0
        }
    });
}

#[test]
fn test_named_routes_with_non_ascii_paths() {
    let mut router = Router::new();
    router.insert("/", 0);
    router.insert("/{wildcard:*}", 1);
    router.insert("/matchme/{slug}/", 2);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ / [0]
       ├─ matchme/
       │         ╰─ {slug}
       │                 ╰─ / [2]
       ╰─ {wildcard:*} [1]
    "###);

    assert_router_matches!(router, {
        "/matchme/abc-s-def/" => {
            path: "/matchme/{slug}/",
            value: 2,
            params: {
                "slug" => "abc-s-def"
            }
        }
        "/matchme/abc-ß-def/" => {
            path: "/matchme/{slug}/",
            value: 2,
            params: {
                "slug" => "abc-ß-def"
            }
        }
        "/matchme/abc-⭐-def/" => {
            path: "/matchme/{slug}/",
            value: 2,
            params: {
                "slug" => "abc-⭐-def"
            }
        }
        "/matchme/abc-def-ß/" => {
            path: "/matchme/{slug}/",
            value: 2,
            params: {
                "slug" => "abc-def-ß"
            }
        }
    });
}

#[test]
fn test_named_wildcard_collide() {
    let mut router = Router::<usize>::new();
    router.insert("/git/{org}/{repo}", 1);
    router.insert("/git/{wildcard:*}", 2);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /git/
           ├─ {org}
           │      ╰─ /
           │         ╰─ {repo} [1]
           ╰─ {wildcard:*} [2]
    "###);

    assert_router_matches!(router, {
        "/git/rust-lang/rust" => {
            path: "/git/{org}/{repo}",
            value: 1,
            params: {
                "org" => "rust-lang",
                "repo" => "rust"
            }
        }
        "/git/rust-lang" => {
            path: "/git/{wildcard:*}",
            value: 2,
            params: {
                "wildcard" => "rust-lang"
            }
        }
    });
}

#[test]
fn match_params() {
    let mut router = Router::new();
    router.insert("/api/v1/{param}/{wildcard:*}", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /api/v1/
              ╰─ {param}
                       ╰─ /
                          ╰─ {wildcard:*} [1]
    "###);

    assert_router_matches!(router, {
        "/api/v1/entity" => None
        // NOTE: Different behaviour: path-tree would match "/api/v1/{param}/{wildcard:*}"
        "/api/v1/entity/" => None
        "/api/v1/entity/1" => {
            path: "/api/v1/{param}/{wildcard:*}",
            value: 1,
            params: {
                "param" => "entity",
                "wildcard" => "1"
            }
        }
        "/api/v" => None
        "/api/v2" => None
        "/api/v1/" => None
        "/api/v1/entity/1/foo/bar" => {
            path: "/api/v1/{param}/{wildcard:*}",
            value: 1,
            params: {
                "param" => "entity",
                "wildcard" => "1/foo/bar"
            }
        }
    });

    let mut router = Router::new();
    router.insert("/api/v1/{param}/{plus:*}", 1);

    assert_router_matches!(router, {
        "/api/v1/entity" => None
        "/api/v1/entity/" => None
        "/api/v1/entity/1" => {
            path: "/api/v1/{param}/{plus:*}",
            value: 1,
            params: {
                "param" => "entity",
                "plus" => "1"
            }
        }
        "/api/v" => None
        "/api/v2" => None
        "/api/v1/" => None
        "/api/v1/entity/1/foo/bar" => {
            path: "/api/v1/{param}/{plus:*}",
            value: 1,
            params: {
                "param" => "entity",
                "plus" => "1/foo/bar"
            }
        }
    });

    // NOTE: We don't support 'optional' logic.
    // let mut router = Router::new();
    // router.insert("/api/v1/{param?}", 1);
    //
    // assert_router_matches!(router, {
    //     "/api/v1/" => None
    //     "/api/v1/entity/1/foo/bar" => None
    //     "/api/v" => None
    //     "/api/v2" => None
    //     "/api/xyz" => None
    // });

    let mut router = Router::new();
    router.insert("/v1/some/resource/name:customVerb", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /v1/some/resource/name:customVerb [1]
    "###);

    assert_router_matches!(router, {
        "/v1/some/resource/name:customVerb" => {
            path: "/v1/some/resource/name:customVerb",
            value: 1
        }
        "/v1/some/resource/name:test" => None
    });

    let mut router = Router::new();
    router.insert("/v1/some/resource/{name}:customVerb", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /v1/some/resource/
                        ╰─ {name}
                                ╰─ :customVerb [1]
    "###);

    assert_router_matches!(router, {
        "/v1/some/resource/test:customVerb" => {
            path: "/v1/some/resource/{name}:customVerb",
            value: 1,
            params: {
                "name" => "test"
            }
        }
        "/v1/some/resource/test:test" => None
    });

    // NOTE: We don't support 'optional' logic.
    // let mut router = Router::new();
    // router.insert("/v1/some/resource/name:customVerb?/{param}/{wildcard:*}", 1);
    //
    // assert_router_matches!(router, {
    //     "/v1/some/resource/name:customVerb??/test/optionalWildCard/character" => None
    //     "/v1/some/resource/name:customVerb??/test/" => None
    //     "/v1/some/resource/name:customVerb??/test" => None
    // });

    let mut router = Router::new();
    router.insert("/api/v1/{wildcard:*}", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /api/v1/
              ╰─ {wildcard:*} [1]
    "###);

    assert_router_matches!(router, {
        "/api/v1" => None
        // NOTE: Different behaviour: path-tree would match "/api/v1/{wildcard:*}"
        "/api/v1/" => None
        "/api/v1/entity" => {
            path: "/api/v1/{wildcard:*}",
            value: 1,
            params: {
                "wildcard" => "entity"
            }
        }
        "/api/v1/entity/1/2" => {
            path: "/api/v1/{wildcard:*}",
            value: 1,
            params: {
                "wildcard" => "entity/1/2"
            }
        }
        "/api/v1/Entity/1/2" => {
            path: "/api/v1/{wildcard:*}",
            value: 1,
            params: {
                "wildcard" => "Entity/1/2"
            }
        }
    });

    let mut router = Router::new();
    router.insert("/api/v1/{param}", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /api/v1/
              ╰─ {param} [1]
    "###);

    assert_router_matches!(router, {
        "/api/v1" => None
        "/api/v1/" => None
        "/api/v1/entity" => {
            path: "/api/v1/{param}",
            value: 1,
            params: {
                "param" => "entity"
            }
        }
        "/api/v1/entity/1/2" => None
        "/api/v1/Entity/1/2" => None
    });

    let mut router = Router::new();
    router.insert("/api/v1/{param}/{param2}", 3);
    router.insert("/api/v1/{param}-{param2}", 1);
    router.insert("/api/v1/{param}~{param2}", 2);
    router.insert("/api/v1/{param}.{param2}", 4);
    router.insert("/api/v1/{param}_{param2}", 5);
    router.insert("/api/v1/{param}:{param2}", 6);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /api/v1/
              ╰─ {param}
                       ├─ /
                       │  ╰─ {param2} [3]
                       ├─ -
                       │  ╰─ {param2} [1]
                       ├─ ~
                       │  ╰─ {param2} [2]
                       ├─ .
                       │  ╰─ {param2} [4]
                       ├─ _
                       │  ╰─ {param2} [5]
                       ╰─ :
                          ╰─ {param2} [6]
    "###);

    assert_router_matches!(router, {
        "/api/v1/entity-entity2" => {
            path: "/api/v1/{param}-{param2}",
            value: 1,
            params: {
                "param" => "entity",
                "param2" => "entity2"
            }
        }
        "/api/v1/entity~entity2" => {
            path: "/api/v1/{param}~{param2}",
            value: 2,
            params: {
                "param" => "entity",
                "param2" => "entity2"
            }
        }
        "/api/v1/entity.entity2" => {
            path: "/api/v1/{param}.{param2}",
            value: 4,
            params: {
                "param" => "entity",
                "param2" => "entity2"
            }
        }
        "/api/v1/entity_entity2" => {
            path: "/api/v1/{param}_{param2}",
            value: 5,
            params: {
                "param" => "entity",
                "param2" => "entity2"
            }
        }
        "/api/v1/entity:entity2" => {
            path: "/api/v1/{param}:{param2}",
            value: 6,
            params: {
                "param" => "entity",
                "param2" => "entity2"
            }
        }
        "/api/v1/entity/entity2" => {
            path: "/api/v1/{param}/{param2}",
            value: 3,
            params: {
                "param" => "entity",
                "param2" => "entity2"
            }
        }
        "/api/v1" => None
        "/api/v1/" => None
        "/api/v1/test.pdf" => {
            path: "/api/v1/{param}.{param2}",
            value: 4,
            params: {
                "param" => "test",
                "param2" => "pdf"
            }
        }
    });

    let mut router = Router::new();
    router.insert("/api/v1/const", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /api/v1/const [1]
    "###);

    assert_router_matches!(router, {
        "/api/v1/const" => {
            path: "/api/v1/const",
            value: 1
        }
        "/api/v1/cons" => None
        "/api/v1/conststatic" => None
        "/api/v1/let" => None
        "/api/v1/" => None
        "/api/v1" => None
    });

    let mut router = Router::new();
    router.insert("/api/{param}/fixedEnd", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /api/
           ╰─ {param}
                    ╰─ /fixedEnd [1]
    "###);

    assert_router_matches!(router, {
        "/api/abc/fixedEnd" => {
            path: "/api/{param}/fixedEnd",
            value: 1,
            params: {
                "param" => "abc"
            }
        }
        "/api/abc/def/fixedEnd" => None
    });

    let mut router = Router::new();
    router.insert("/shop/product/:{filter}/color:{color}/size:{size}", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /shop/product/:
                     ╰─ {filter}
                               ╰─ /color:
                                        ╰─ {color}
                                                 ╰─ /size:
                                                         ╰─ {size} [1]
    "###);

    assert_router_matches!(router, {
        "/shop/product/:test/color:blue/size:xs" => {
            path: "/shop/product/:{filter}/color:{color}/size:{size}",
            value: 1,
            params: {
                "filter" => "test",
                "color" => "blue",
                "size" => "xs"
            }
        }
        "/shop/product/test/color:blue/size:xs" => None
    });

    // NOTE: We don't support 'optional' logic.
    // let mut router = Router::new();
    // router.insert("/{param?}", 1);
    //
    // assert_router_matches!(router, {
    //     "/:hello" => None
    //     "/:" => None
    //     "/" => None
    // });

    let mut router = Router::new();
    router.insert("/test{sign}{param}", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /test
           ╰─ {sign}
                   ╰─ {param} [1]
    "###);

    // FIXME: This is a bug in our matcher! We're 'too greedy' in our matching.
    // TODO: Consider if we want to allow multiple parameters side-by-side?
    // Output below seems ambigous.
    assert_router_matches!(router, {
        // "/test-abc" => {
        //     path: "/test{sign}{param}",
        //     value: 1,
        //     params: {
        //         "sign" => "-",
        //         "param" => "abc"
        //     }
        // }
        // "/test-_" => {
        //     path: "/test{sign}{param}",
        //     value: 1,
        //     params: {
        //         "sign" => "-",
        //         "param" => "_"
        //     }
        // }
        "/test-" => None
        "/test" => None
    });

    // NOTE: We don't support 'optional' logic.
    // let mut router = Router::new();
    // router.insert("/{param1}{param2?}{param3}", 1);
    //
    // assert_router_matches!(router, {
    //     "/abbbc" => None
    //     "/ab" => None
    //     "/a" => None
    // });

    // NOTE: We don't support 'optional' logic.
    // let mut router = Router::new();
    // router.insert("/test{optional?}{mandatory}", 1);
    //
    // assert_router_matches!(router, {
    //     "/testo" => None
    //     "/testoaaa" => None
    //     "/test" => None
    //     "/tes" => None
    // });

    // NOTE: We don't support 'optional' logic.
    // let mut router = Router::new();
    // router.insert("/test{optional?}{optional2?}", 1);
    //
    // assert_router_matches!(router, {
    //     "/testo" => None
    //     "/testoaaa" => None
    //     "/test" => None
    //     "/tes" => None
    // });

    // NOTE: We don't support 'optional' logic.
    // let mut router = Router::new();
    // router.insert("/foo{param?}bar", 1);
    //
    // assert_router_matches!(router, {
    //     "/foofalsebar" => None
    //     "/foobar" => None
    //     "/fooba" => None
    //     "/foo" => None
    // });

    // NOTE: We don't support 'inline wildcard' logic.
    // let mut router = Router::new();
    // router.insert("/foo{wildcard:*}bar", 1);
    //
    // assert_router_matches!(router, {
    //     "/foofalsebar" => None
    //     "/foobar" => None
    //     "/foo/bar" => None
    //     "/foo/baz/bar" => None
    //     "/fooba" => None
    //     "/foo" => None
    // });

    // NOTE: We don't support 'one or more' logic.
    // let mut router = Router::new();
    // router.insert("/foo{plus:+}bar", 1);
    //
    // assert_router_matches!(router, {
    //     "/foofalsebar" => None
    //     "/foobar" => None
    //     "/foo/bar" => None
    //     "/foo/baz/bar" => None
    //     "/fooba" => None
    //     "/foo" => None
    // });

    // NOTE: We don't support 'inline wildcard' logic.
    // let mut router = Router::new();
    // router.insert("/a{wildcard1:*}cde{wildcard2:*}g/", 1);
    //
    // assert_router_matches!(router, {
    //     "/abbbcdefffg" => None
    //     "/abbbcdefffg/" => None
    //     "/acdeg/" => None
    //     "/abcdeg/" => None
    //     "/acdefg/" => None
    //     "/abcdefg/" => None
    //     "/a/cde/g/" => None
    //     "/a/b/cde/f/g/" => None
    // });

    // NOTE: We don't support 'inline wildcard' logic.
    // let mut router = Router::new();
    // router.insert("/{wildcard1:*}v1{wildcard2:*}/proxy", 1);
    //
    // assert_router_matches!(router, {
    //     "/customer/v1/cart/proxy" => None
    //     "/v1/proxy" => None
    //     "/v1/" => None
    // });

    let mut router = Router::new();
    router.insert("/name{name}", 1);
    router.insert("/@{name}", 2);
    router.insert("/-{name}", 3);
    router.insert("/.{name}", 4);
    router.insert("/~{name}", 5);
    router.insert("/_{name}", 6);
    router.insert("/{name}", 7);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ├─ name
       │     ╰─ {name} [1]
       ├─ @
       │  ╰─ {name} [2]
       ├─ -
       │  ╰─ {name} [3]
       ├─ .
       │  ╰─ {name} [4]
       ├─ ~
       │  ╰─ {name} [5]
       ├─ _
       │  ╰─ {name} [6]
       ╰─ {name} [7]
    "###);

    assert_router_matches!(router, {
        "/name:john" => {
            path: "/name{name}",
            value: 1,
            params: {
                "name" => ":john"
            }
        }
        "/@john" => {
            path: "/@{name}",
            value: 2,
            params: {
                "name" => "john"
            }
        }
        "/-john" => {
            path: "/-{name}",
            value: 3,
            params: {
                "name" => "john"
            }
        }
        "/.john" => {
            path: "/.{name}",
            value: 4,
            params: {
                "name" => "john"
            }
        }
        "/~john" => {
            path: "/~{name}",
            value: 5,
            params: {
                "name" => "john"
            }
        }
        "/_john" => {
            path: "/_{name}",
            value: 6,
            params: {
                "name" => "john"
            }
        }
        "/john" => {
            path: "/{name}",
            value: 7,
            params: {
                "name" => "john"
            }
        }
    });

    let mut router = Router::new();
    router.insert("/api/v1/{param}/abc/{wildcard:*}", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /api/v1/
              ╰─ {param}
                       ╰─ /abc/
                              ╰─ {wildcard:*} [1]
    "###);

    assert_router_matches!(router, {
        "/api/v1/well/abc/wildcard" => {
            path: "/api/v1/{param}/abc/{wildcard:*}",
            value: 1,
            params: {
                "param" => "well",
                "wildcard" => "wildcard"
            }
        }
        // NOTE: Different behaviour: path-tree would match "/api/v1/{param}/abc/{wildcard:*}
        "/api/v1/well/abc/" => None
        "/api/v1/well/abc" => None
        "/api/v1/well/ttt" => None
    });

    // NOTE: We don't support 'optional' logic.
    // let mut router = Router::new();
    // router.insert("/api/{day}/{month?}/{year?}", 1);
    //
    // assert_router_matches!(router, {
    //     "/api/1" => None
    //     "/api/1/" => None
    //     "/api/1//" => None
    //     "/api/1/-/" => None
    //     "/api/1-" => None
    //     "/api/1-/" => None
    //     "/api/1/2" => None
    //     "/api/1/2/3" => None
    // });

    // NOTE: We don't support 'optional' logic.
    // let mut router = Router::new();
    // router.insert("/api/{day}.{month?}.{year?}", 1);
    // router.insert("/api/{day}-{month?}-{year?}", 2);
    //
    // assert_router_matches!(router, {
    //     "/api/1" => None
    //     "/api/1/" => None
    //     "/api/1." => None
    //     "/api/1.." => None
    //     "/api/1.2." => None
    //     "/api/1.2.3" => None
    //     "/api/1--" => None
    //     "/api/1-2-" => None
    //     "/api/1-2-3" => None
    //     "/api/1.2-3" => None
    // });

    // NOTE: We don't support 'one or more' logic.
    // let mut router = Router::new();
    // router.insert("/config/abc.json", 1);
    // router.insert("/config/{plus:+}.json", 2);
    // router.insert("/config/{wildcard:*}.json", 3);
    //
    // assert_router_matches!(router, {
    //     "/config/abc.json" => None
    //     "/config/a.json" => None
    //     "/config/ab.json" => None
    //     "/config/a/b.json" => None
    //     "/config/a/b/abc.json" => None
    //     "/config/.json" => None
    // });

    // FIXME: We don't support 'wildcard' logic.
    // let mut router = Router::new();
    // router.insert("/api/{wildcard:*}/{param?}", 1);
    //
    // assert_router_matches!(router, {
    //     "/api/" => None
    //     "/api/joker" => None
    //     "/api/joker/" => None
    //     "/api/joker/batman" => None
    //     "/api/joker/batman/robin" => None
    //     "/api/joker/batman/robin/1" => None
    // });

    let mut router = Router::new();
    router.insert("/api/{wildcard:*}/{param}", 1);

    assert_router_matches!(router, {
        "/api/test/abc" => {
            path: "/api/{wildcard:*}/{param}",
            value: 1,
            params: {
                "wildcard" => "test",
                "param" => "abc"
            }
        }
        "/api/joker/batman/robin/1" => {
            path: "/api/{wildcard:*}/{param}",
            value: 1,
            params: {
                "wildcard" => "joker/batman/robin",
                "param" => "1"
            }
        }
        // FIXME: This feels wrong. Maybe we should add a check for empty parts?
        "/api//joker" => {
            path: "/api/{wildcard:*}/{param}",
            value: 1,
            params: {
                "wildcard" => "",
                "param" => "joker"
            }
        }
        "/api/joker" => None
        "/api/" => None
    });

    let mut router = Router::new();
    router.insert("/api/{wildcard:*}/{param}/{param2}", 1);

    assert_router_matches!(router, {
        "/api/test/abc/1" => {
            path: "/api/{wildcard:*}/{param}/{param2}",
            value: 1,
            params: {
                "wildcard" => "test",
                "param" => "abc",
                "param2" => "1"
            }
        }
        "/api/joker/batman" => None
        "/api/joker/batman-robin/1" => {
            path: "/api/{wildcard:*}/{param}/{param2}",
            value: 1,
            params: {
                "wildcard" => "joker",
                "param" => "batman-robin",
                "param2" => "1"
            }
        }
        "/api/joker-batman-robin-1" => None
        "/api/test/abc" => None
        "/api/joker/batman/robin" => {
            path: "/api/{wildcard:*}/{param}/{param2}",
            value: 1,
            params: {
                "wildcard" => "joker",
                "param" => "batman",
                "param2" => "robin"
            }
        }
        "/api/joker/batman/robin/1" => {
            path: "/api/{wildcard:*}/{param}/{param2}",
            value: 1,
            params: {
                "wildcard" => "joker/batman",
                "param" => "robin",
                "param2" => "1"
            }
        }
        "/api/joker/batman/robin/1/2" => {
            path: "/api/{wildcard:*}/{param}/{param2}",
            value: 1,
            params: {
                "wildcard" => "joker/batman/robin",
                "param" => "1",
                "param2" => "2"
            }
        }
        "/api" => None
        "/api/:test" => None
    });
}

#[test]
fn basic() {
    let mut router = Router::new();
    router.insert("/", 0);
    router.insert("/login", 1);
    router.insert("/signup", 2);
    router.insert("/settings", 3);
    router.insert("/settings/{page}", 4);
    router.insert("/{user}", 5);
    router.insert("/{user}/{repo}", 6);
    router.insert("/public/{any:*}", 7);
    router.insert("/{org}/{repo}/releases/download/{tag}/{filename}.{ext}", 8);
    router.insert("/{org}/{repo}/tags/{day}-{month}-{year}", 9);
    router.insert("/{org}/{repo}/actions/{name}:{verb}", 10);
    router.insert("/{org}/{repo}/{page}", 11);
    router.insert("/{org}/{repo}/{path:*}", 12);
    router.insert("/api/{plus:*}", 13);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ / [0]
       ├─ login [1]
       ├─ s
       │  ├─ ignup [2]
       │  ╰─ ettings [3]
       │           ╰─ /
       │              ╰─ {page} [4]
       ├─ public/
       │        ╰─ {any:*} [7]
       ├─ api/
       │     ╰─ {plus:*} [13]
       ├─ {user} [5]
       │       ╰─ /
       │          ╰─ {repo} [6]
       ╰─ {org}
              ╰─ /
                 ╰─ {repo}
                         ╰─ /
                            ├─ releases/download/
                            │                   ╰─ {tag}
                            │                          ╰─ /
                            │                             ╰─ {filename}
                            │                                         ╰─ .
                            │                                            ╰─ {ext} [8]
                            ├─ tags/
                            │      ╰─ {day}
                            │             ╰─ -
                            │                ╰─ {month}
                            │                         ╰─ -
                            │                            ╰─ {year} [9]
                            ├─ actions/
                            │         ╰─ {name}
                            │                 ╰─ :
                            │                    ╰─ {verb} [10]
                            ├─ {page} [11]
                            ╰─ {path:*} [12]
    "###);

    assert_router_matches!(router, {
        "/" => {
            path: "/",
            value: 0
        }
        "/login" => {
            path: "/login",
            value: 1
        }
        "/settings/admin" => {
            path: "/settings/{page}",
            value: 4,
            params: {
                "page" => "admin"
            }
        }
        "/viz-rs" => {
            path: "/{user}",
            value: 5,
            params: {
                "user" => "viz-rs"
            }
        }
        "/viz-rs/path-tree" => {
            path: "/{user}/{repo}",
            value: 6,
            params: {
                "user" => "viz-rs",
                "repo" => "path-tree"
            }
        }
        "/rust-lang/rust-analyzer/releases/download/2022-09-12/rust-analyzer-aarch64-apple-darwin.gz" => {
            path: "/{org}/{repo}/releases/download/{tag}/{filename}.{ext}",
            value: 8,
            params: {
                "org" => "rust-lang",
                "repo" => "rust-analyzer",
                "tag" => "2022-09-12",
                "filename" => "rust-analyzer-aarch64-apple-darwin",
                "ext" => "gz"
            }
        }
        "/rust-lang/rust-analyzer/tags/2022-09-12" => {
            path: "/{org}/{repo}/tags/{day}-{month}-{year}",
            value: 9,
            params: {
                "org" => "rust-lang",
                "repo" => "rust-analyzer",
                "day" => "2022",
                "month" => "09",
                "year" => "12"
            }
        }
        "/rust-lang/rust-analyzer/actions/ci:bench" => {
            path: "/{org}/{repo}/actions/{name}:{verb}",
            value: 10,
            params: {
                "org" => "rust-lang",
                "repo" => "rust-analyzer",
                "name" => "ci",
                "verb" => "bench"
            }
        }
        "/rust-lang/rust-analyzer/stargazers" => {
            path: "/{org}/{repo}/{page}",
            value: 11,
            params: {
                "org" => "rust-lang",
                "repo" => "rust-analyzer",
                "page" => "stargazers"
            }
        }
        "/rust-lang/rust-analyzer/stargazers/404" => {
            path: "/{org}/{repo}/{path:*}",
            value: 12,
            params: {
                "org" => "rust-lang",
                "repo" => "rust-analyzer",
                "path" => "stargazers/404"
            }
        }
        "/public/js/main.js" => {
            path: "/public/{any:*}",
            value: 7,
            params: {
                "any" => "js/main.js"
            }
        }
        "/api/v1" => {
            path: "/api/{plus:*}",
            value: 13,
            params: {
                "plus" => "v1"
            }
        }
    });
}

#[test]
fn github_tree() {
    let mut router = Router::new();

    router.insert("/", 0);
    router.insert("/api", 1);
    router.insert("/about", 2);
    router.insert("/login", 3);
    router.insert("/signup", 4);
    router.insert("/pricing", 5);

    router.insert("/features", 6);
    router.insert("/features/actions", 600);
    router.insert("/features/packages", 601);
    router.insert("/features/security", 602);
    router.insert("/features/codespaces", 603);
    router.insert("/features/copilot", 604);
    router.insert("/features/code-review", 605);
    router.insert("/features/issues", 606);
    router.insert("/features/discussions", 607);

    router.insert("/enterprise", 7);
    router.insert("/team", 8);
    router.insert("/customer-stories", 9);
    router.insert("/sponsors", 10);
    router.insert("/readme", 11);
    router.insert("/topics", 12);
    router.insert("/trending", 13);
    router.insert("/collections", 14);
    router.insert("/search", 15);
    router.insert("/pulls", 16);
    router.insert("/issues", 17);
    router.insert("/marketplace", 18);
    router.insert("/explore", 19);

    router.insert("/sponsors/explore", 100);
    router.insert("/sponsors/accounts", 101);
    router.insert("/sponsors/{repo}", 102);
    router.insert("/sponsors/{repo}/{user}", 103);
    router.insert("/sponsors/{repo}/{plus:*}", 104);
    router.insert("/sponsors/{repo}/issues/{path:*}", 106);
    router.insert("/sponsors/{repo}/{plus:*}/{file}", 107);
    router.insert("/sponsors/{repo}/{plus:*}/{filename}.{ext}", 108);

    router.insert("/about/careers", 200);
    router.insert("/about/press", 201);
    router.insert("/about/diversity", 202);

    router.insert("/settings", 20);
    router.insert("/settings/admin", 2000);
    router.insert("/settings/appearance", 2001);
    router.insert("/settings/accessibility", 2002);
    router.insert("/settings/notifications", 2003);

    router.insert("/settings/billing", 2004);
    router.insert("/settings/billing/plans", 2005);
    router.insert("/settings/security", 2006);
    router.insert("/settings/keys", 2007);
    router.insert("/settings/organizations", 2008);

    router.insert("/settings/blocked_users", 2009);
    router.insert("/settings/interaction_limits", 2010);
    router.insert("/settings/code_review_limits", 2011);

    router.insert("/settings/repositories", 2012);
    router.insert("/settings/codespaces", 2013);
    router.insert("/settings/deleted_packages", 2014);
    router.insert("/settings/copilot", 2015);
    router.insert("/settings/pages", 2016);
    router.insert("/settings/replies", 2017);

    router.insert("/settings/security_analysis", 2018);

    router.insert("/settings/installations", 2019);
    router.insert("/settings/reminders", 2020);

    router.insert("/settings/security-log", 2021);
    router.insert("/settings/sponsors-log", 2022);

    router.insert("/settings/apps", 2023);
    router.insert("/settings/developers", 2024);
    router.insert("/settings/tokens", 2025);

    router.insert("/404", 21);
    router.insert("/500", 22);
    router.insert("/503", 23);

    router.insert("/{org}", 24);
    router.insert("/{org}/{repo}", 2400);
    router.insert("/{org}/{repo}/issues", 2410);
    router.insert("/{org}/{repo}/issues/{id}", 2411);
    router.insert("/{org}/{repo}/issues/new", 2412);
    router.insert("/{org}/{repo}/pulls", 2420);
    router.insert("/{org}/{repo}/pull/{id}", 2421);
    router.insert("/{org}/{repo}/compare", 2422);
    router.insert("/{org}/{repo}/discussions", 2430);
    router.insert("/{org}/{repo}/discussions/{id}", 2431);
    router.insert("/{org}/{repo}/actions", 2440);
    router.insert("/{org}/{repo}/actions/workflows/{id}", 2441);
    router.insert("/{org}/{repo}/actions/runs/{id}", 2442);
    router.insert("/{org}/{repo}/wiki", 2450);
    router.insert("/{org}/{repo}/wiki/{id}", 2451);
    router.insert("/{org}/{repo}/security", 2460);
    router.insert("/{org}/{repo}/security/policy", 2461);
    router.insert("/{org}/{repo}/security/advisories", 2462);
    router.insert("/{org}/{repo}/pulse", 2470);
    router.insert("/{org}/{repo}/graphs/contributors", 2480);
    router.insert("/{org}/{repo}/graphs/commit-activity", 2481);
    router.insert("/{org}/{repo}/graphs/code-frequency", 2482);
    router.insert("/{org}/{repo}/community", 2490);
    router.insert("/{org}/{repo}/network", 2491);
    router.insert("/{org}/{repo}/network/dependencies", 2492);
    router.insert("/{org}/{repo}/network/dependents", 2493);
    router.insert("/{org}/{repo}/network/members", 2494);
    router.insert("/{org}/{repo}/stargazers", 2495);
    router.insert("/{org}/{repo}/stargazers/yoou_know", 2496);
    router.insert("/{org}/{repo}/watchers", 2497);
    router.insert("/{org}/{repo}/releases", 2498);
    router.insert("/{org}/{repo}/releases/tag/{id}", 2499);
    router.insert("/{org}/{repo}/tags", 2500);
    router.insert("/{org}/{repo}/tags/{id}", 2501);
    router.insert("/{org}/{repo}/tree/{id}", 2502);
    router.insert("/{org}/{repo}/commit/{id}", 2503);

    router.insert("/new", 2504);
    router.insert("/new/import", 2505);
    router.insert("/organizations/new", 2506);
    router.insert("/organizations/plan", 2507);

    router.insert("/{org}/{repo}/{path:*}", 3000);
    router.insert("/{org}/{repo}/releases/{path:*}", 3001);
    router.insert("/{org}/{repo}/releases/download/{tag}/{filename}.{ext}", 3002);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ / [0]
       ├─ a
       │  ├─ pi [1]
       │  ╰─ bout [2]
       │        ╰─ /
       │           ├─ careers [200]
       │           ├─ press [201]
       │           ╰─ diversity [202]
       ├─ login [3]
       ├─ s
       │  ├─ ignup [4]
       │  ├─ ponsors [10]
       │  │        ╰─ /
       │  │           ├─ explore [100]
       │  │           ├─ accounts [101]
       │  │           ╰─ {repo} [102]
       │  │                   ╰─ /
       │  │                      ├─ issues/
       │  │                      │        ╰─ {path:*} [106]
       │  │                      ├─ {user} [103]
       │  │                      ├─ {plus:*}
       │  │                      │         ╰─ /
       │  │                      │            ├─ {file} [107]
       │  │                      │            ╰─ {filename}
       │  │                      │                        ╰─ .
       │  │                      │                           ╰─ {ext} [108]
       │  │                      ╰─ {plus:*} [104]
       │  ╰─ e
       │     ├─ arch [15]
       │     ╰─ ttings [20]
       │             ╰─ /
       │                ├─ a
       │                │  ├─ dmin [2000]
       │                │  ├─ pp
       │                │  │   ├─ earance [2001]
       │                │  │   ╰─ s [2023]
       │                │  ╰─ ccessibility [2002]
       │                ├─ notifications [2003]
       │                ├─ b
       │                │  ├─ illing [2004]
       │                │  │       ╰─ /plans [2005]
       │                │  ╰─ locked_users [2009]
       │                ├─ s
       │                │  ├─ ecurity [2006]
       │                │  │        ├─ _analysis [2018]
       │                │  │        ╰─ -log [2021]
       │                │  ╰─ ponsors-log [2022]
       │                ├─ keys [2007]
       │                ├─ organizations [2008]
       │                ├─ in
       │                │   ├─ teraction_limits [2010]
       │                │   ╰─ stallations [2019]
       │                ├─ co
       │                │   ├─ de
       │                │   │   ├─ _review_limits [2011]
       │                │   │   ╰─ spaces [2013]
       │                │   ╰─ pilot [2015]
       │                ├─ re
       │                │   ├─ p
       │                │   │  ├─ ositories [2012]
       │                │   │  ╰─ lies [2017]
       │                │   ╰─ minders [2020]
       │                ├─ de
       │                │   ├─ leted_packages [2014]
       │                │   ╰─ velopers [2024]
       │                ├─ pages [2016]
       │                ╰─ tokens [2025]
       ├─ p
       │  ├─ ricing [5]
       │  ╰─ ulls [16]
       ├─ features [6]
       │         ╰─ /
       │            ├─ actions [600]
       │            ├─ packages [601]
       │            ├─ security [602]
       │            ├─ co
       │            │   ├─ de
       │            │   │   ├─ spaces [603]
       │            │   │   ╰─ -review [605]
       │            │   ╰─ pilot [604]
       │            ├─ issues [606]
       │            ╰─ discussions [607]
       ├─ e
       │  ├─ nterprise [7]
       │  ╰─ xplore [19]
       ├─ t
       │  ├─ eam [8]
       │  ├─ opics [12]
       │  ╰─ rending [13]
       ├─ c
       │  ├─ ustomer-stories [9]
       │  ╰─ ollections [14]
       ├─ readme [11]
       ├─ issues [17]
       ├─ marketplace [18]
       ├─ 404 [21]
       ├─ 50
       │   ├─ 0 [22]
       │   ╰─ 3 [23]
       ├─ new [2504]
       │    ╰─ /import [2505]
       ├─ organizations/
       │               ├─ new [2506]
       │               ╰─ plan [2507]
       ╰─ {org} [24]
              ╰─ /
                 ╰─ {repo} [2400]
                         ╰─ /
                            ├─ issues [2410]
                            │       ╰─ /
                            │          ├─ new [2412]
                            │          ╰─ {id} [2411]
                            ├─ pul
                            │    ├─ l
                            │    │  ├─ s [2420]
                            │    │  ╰─ /
                            │    │     ╰─ {id} [2421]
                            │    ╰─ se [2470]
                            ├─ com
                            │    ├─ pare [2422]
                            │    ╰─ m
                            │       ├─ unity [2490]
                            │       ╰─ it/
                            │            ╰─ {id} [2503]
                            ├─ discussions [2430]
                            │            ╰─ /
                            │               ╰─ {id} [2431]
                            ├─ actions [2440]
                            │        ╰─ /
                            │           ├─ workflows/
                            │           │           ╰─ {id} [2441]
                            │           ╰─ runs/
                            │                  ╰─ {id} [2442]
                            ├─ w
                            │  ├─ iki [2450]
                            │  │    ╰─ /
                            │  │       ╰─ {id} [2451]
                            │  ╰─ atchers [2497]
                            ├─ s
                            │  ├─ ecurity [2460]
                            │  │        ╰─ /
                            │  │           ├─ policy [2461]
                            │  │           ╰─ advisories [2462]
                            │  ╰─ targazers [2495]
                            │             ╰─ /yoou_know [2496]
                            ├─ graphs/co
                            │          ├─ ntributors [2480]
                            │          ├─ mmit-activity [2481]
                            │          ╰─ de-frequency [2482]
                            ├─ network [2491]
                            │        ╰─ /
                            │           ├─ dependen
                            │           │         ├─ cies [2492]
                            │           │         ╰─ ts [2493]
                            │           ╰─ members [2494]
                            ├─ releases [2498]
                            │         ╰─ /
                            │            ├─ tag/
                            │            │     ╰─ {id} [2499]
                            │            ├─ download/
                            │            │          ╰─ {tag}
                            │            │                 ╰─ /
                            │            │                    ╰─ {filename}
                            │            │                                ╰─ .
                            │            │                                   ╰─ {ext} [3002]
                            │            ╰─ {path:*} [3001]
                            ├─ t
                            │  ├─ ags [2500]
                            │  │    ╰─ /
                            │  │       ╰─ {id} [2501]
                            │  ╰─ ree/
                            │        ╰─ {id} [2502]
                            ╰─ {path:*} [3000]
    "###);

    assert_router_matches!(router, {
        "/rust-lang/rust" => {
            path: "/{org}/{repo}",
            value: 2400,
            params: {
                "org" => "rust-lang",
                "repo" => "rust"
            }
        }
        "/settings" => {
            path: "/settings",
            value: 20
        }
        "/rust-lang/rust/actions/runs/1" => {
            path: "/{org}/{repo}/actions/runs/{id}",
            value: 2442,
            params: {
                "org" => "rust-lang",
                "repo" => "rust",
                "id" => "1"
            }
        }
        // NOTE: Different behaviour: path-tree would match "/{org}/{repo}/{path:*}"
        "/rust-lang/rust/" => None
        "/rust-lang/rust/any" => {
            path: "/{org}/{repo}/{path:*}",
            value: 3000,
            params: {
                "org" => "rust-lang",
                "repo" => "rust",
                "path" => "any"
            }
        }
        // NOTE: Different behaviour: path-tree would match "/{org}/{repo}/{path:*}"
        "/rust-lang/rust/releases/" => {
            path: "/{org}/{repo}/{path:*}",
            value: 3000,
            params: {
                "org" => "rust-lang",
                "repo" => "rust",
                "path" => "releases/"
            }
        }
        "/rust-lang/rust-analyzer/releases/download/2022-09-12/rust-analyzer-aarch64-apple-darwin.gz" => {
            path: "/{org}/{repo}/releases/download/{tag}/{filename}.{ext}",
            value: 3002,
            params: {
                "org" => "rust-lang",
                "repo" => "rust-analyzer",
                "tag" => "2022-09-12",
                "filename" => "rust-analyzer-aarch64-apple-darwin",
                "ext" => "gz"
            }
        }
    });
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
fn test_dots_no_ext() {
    let mut router = Router::new();
    router.insert("/{name}", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ╰─ {name} [1]
    "###);

    assert_router_matches!(router, {
        "/abc.xyz.123" => {
            path: "/{name}",
            value: 1,
            params: {
                "name" => "abc.xyz.123"
            }
        }
    });
}

#[test]
#[ignore = "we don't support 'one or more' or 'inline wildcard' logic"]
fn test_dots_ext() {
    let mut router = Router::new();
    router.insert("/{name:+}.123", 2);
    router.insert("/{name:*}.123.456", 1);

    insta::assert_snapshot!(router, @"");

    assert_router_matches!(router, {
        "/abc.xyz.123" => {
            path: "/{name:+}.123",
            value: 2,
            params: {
                "name" => "abc.xyz"
            }
        }
        "/abc.xyz.123.456" => {
            path: "/{name:*}.123.456",
            value: 1,
            params: {
                "name" => "abc.xyz"
            }
        }
    });
}

#[test]
fn test_dots_ext_no_qualifier() {
    let mut router = Router::new();
    router.insert("/{name}.js", 2);
    router.insert("/{name}.js.gz", 1);

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /
       ╰─ {name}
               ╰─ .js [2]
                    ╰─ .gz [1]
    "###);

    assert_router_matches!(router, {
        "/node.js" => {
            path: "/{name}.js",
            value: 2,
            params: {
                "name" => "node"
            }
        }
        "/path.lib.js" => {
            path: "/{name}.js",
            value: 2,
            params: {
                "name" => "path.lib"
            }
        }
        "/node.js.js" => {
            path: "/{name}.js",
            value: 2,
            params: {
                "name" => "node.js"
            }
        }
        "/node.js.gz" => {
            path: "/{name}.js.gz",
            value: 1,
            params: {
                "name" => "node"
            }
        }
        "/node.js.gz.js.gz" => {
            path: "/{name}.js.gz",
            value: 1,
            params: {
                "name" => "node.js.gz"
            }
        }
    });
}
