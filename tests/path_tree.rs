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

    // NOTE: We don't support 'one or more' logic.
    // let mut router = Router::new();
    // router.insert("/api/v1/{param}/{plus:+}", 1);
    //
    // assert_router_matches!(router, {
    //     "/api/v1/entity" => None
    //     "/api/v1/entity/" => None
    //     "/api/v1/entity/1" => None
    //     "/api/v" => None
    //     "/api/v2" => None
    //     "/api/v1/" => None
    //     "/api/v1/entity/1/foo/bar" => None
    // });

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

    assert_router_matches!(router, {
        "/v1/some/resource/name:customVerb" => {
            path: "/v1/some/resource/name:customVerb",
            value: 1
        }
        "/v1/some/resource/name:test" => None
    });

    let mut router = Router::new();
    router.insert("/v1/some/resource/{name}:customVerb", 1);

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
    router.insert("/shop/product/{filter}/color{color}/size{size}", 1);

    assert_router_matches!(router, {
        "/shop/product/:test/color:blue/size:xs" => {
            path: "/shop/product/{filter}/color{color}/size{size}",
            value: 1,
            params: {
                "filter" => ":test",
                "color" => ":blue",
                "size" => ":xs"
            }
        }
        // FIXME: This is a bug in our parser! Oops.
        // "/shop/product/test/color:blue/size:xs" => None
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

    // FIXME: This is a bug in our matcher! We're 'too greedy' in our matching.
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

    // NOTE: We don't support 'one or more' logic.
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

    // FIXME: We don't support 'wildcard' logic.
    // let mut router = Router::new();
    // router.insert("/api/{wildcard:*}/{param}", 1);
    //
    // assert_router_matches!(router, {
    //     "/api/test/abc" => None
    //     "/api/joker/batman/robin/1" => None
    //     "/api//joker" => None
    //     "/api/joker" => None
    //     "/api/" => None
    // });

    // NOTE: We don't support 'one or more' logic.
    // let mut router = Router::new();
    // router.insert("/api/{plus:+}/{param}", 1);
    //
    // assert_router_matches!(router, {
    //     "/api/test/abc" => None
    //     "/api/joker/batman/robin/1" => None
    //     "/api/joker" => None
    //     "/api/" => None
    // });

    // FIXME: We don't support 'wildcard' logic.
    // let mut router = Router::new();
    // router.insert("/api/{wildcard:*}/{param}/{param2}", 1);
    //
    // assert_router_matches!(router, {
    //     "/api/test/abc/1" => None
    //     "/api/joker/batman" => None
    //     "/api/joker/batman-robin/1" => None
    //     "/api/joker-batman-robin-1" => None
    //     "/api/test/abc" => None
    //     "/api/joker/batman/robin" => None
    //     "/api/joker/batman/robin/1" => None
    //     "/api/joker/batman/robin/1/2" => None
    //     "/api" => None
    //     "/api/:test" => None
    // });
}
