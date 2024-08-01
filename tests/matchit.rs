//! Tests sourced from `matchit` (MIT AND BSD-3-Clause)
//! <https://github.com/ibraheemdev/matchit/blob/v0.8.3/tests/match.rs>

#![allow(clippy::too_many_lines, clippy::cognitive_complexity)]

use wayfind::{assert_router_matches, router::Router};

// https://github.com/ibraheemdev/matchit/issues/22
#[test]
fn partial_overlap() {
    let mut router = Router::new();
    router.insert("/foo_bar", "Welcome!");
    router.insert("/foo/bar", "Welcome!");

    assert_router_matches!(router, {
        "/foo/" => None
    });

    let mut router = Router::new();
    router.insert("/foo", "Welcome!");
    router.insert("/foo/bar", "Welcome!");

    assert_router_matches!(router, {
        "/foo/" => None
    });
}

// https://github.com/ibraheemdev/matchit/issues/31
#[test]
fn wildcard_overlap() {
    let mut router = Router::new();
    router.insert("/path/foo", "foo");
    router.insert("/path/{rest:*}", "wildcard");

    assert_router_matches!(router, {
        "/path/foo" => {
            path: "/path/foo",
            value: "foo"
        }
        "/path/bar" => {
            path: "/path/{rest:*}",
            value: "wildcard",
            params: {
                "rest" => "bar"
            }
        }
        "/path/foo/" => {
            path: "/path/{rest:*}",
            value: "wildcard",
            params: {
                "rest" => "foo/"
            }
        }
    });

    let mut router = Router::new();
    router.insert("/path/foo/{arg}", "foo");
    router.insert("/path/{rest:*}", "wildcard");

    assert_router_matches!(router, {
        "/path/foo/myarg" => {
            path: "/path/foo/{arg}",
            value: "foo",
            params: {
                "arg" => "myarg"
            }
        }
        "/path/foo/myarg/" => {
            path: "/path/{rest:*}",
            value: "wildcard",
            params: {
                "rest" => "foo/myarg/"
            }
        }
        "/path/foo/myarg/bar/baz" => {
            path: "/path/{rest:*}",
            value: "wildcard",
            params: {
                "rest" => "foo/myarg/bar/baz"
            }
        }
    });
}

// https://github.com/ibraheemdev/matchit/issues/12
#[test]
fn overlapping_param_backtracking() {
    let mut router = Router::new();
    router.insert("/{object}/{id}", "object with id");
    router.insert("/secret/{id}/path", "secret with id and path");

    assert_router_matches!(router, {
        "/secret/978/path" => {
            path: "/secret/{id}/path",
            value: "secret with id and path",
            params: {
                "id" => "978"
            }
        }
        "/something/978" => {
            path: "/{object}/{id}",
            value: "object with id",
            params: {
                "object" => "something",
                "id" => "978"
            }
        }
        "/secret/978" => {
            path: "/{object}/{id}",
            value: "object with id",
            params: {
                "object" => "secret",
                "id" => "978"
            }
        }
    });
}

// https://github.com/ibraheemdev/matchit/issues/42
#[test]
fn bare_catchall() {
    let mut router = Router::new();
    router.insert("{foo:*}", 1);
    router.insert("foo/{bar:*}", 2);

    assert_router_matches!(router, {
        "x/y" => {
            path: "{foo:*}",
            value: 1,
            params: {
                "foo" => "x/y"
            }
        }
        "/x/y" => {
            path: "{foo:*}",
            value: 1,
            params: {
                "foo" => "/x/y"
            }
        }
        "/foo/x/y" => {
            path: "{foo:*}",
            value: 1,
            params: {
                "foo" => "/foo/x/y"
            }
        }
        "foo/x/y" => {
            path: "foo/{bar:*}",
            value: 2,
            params: {
                "bar" => "x/y"
            }
        }
    });
}

#[test]
fn normalized() {
    let mut router = Router::new();
    router.insert("/x/{foo}/bar", 1);
    router.insert("/x/{bar}/baz", 2);
    router.insert("/{foo}/{baz}/bax", 3);
    router.insert("/{foo}/{bar}/baz", 4);
    router.insert("/{fod}/{baz}/{bax}/foo", 5);
    router.insert("/{fod}/baz/bax/foo", 6);
    router.insert("/{foo}/baz/bax", 7);
    router.insert("/{bar}/{bay}/bay", 8);
    router.insert("/s", 9);
    router.insert("/s/s", 10);
    router.insert("/s/s/s", 11);
    router.insert("/s/s/s/s", 12);
    router.insert("/s/s/{s}/x", 13);
    router.insert("/s/s/{y}/d", 14);

    assert_router_matches!(router, {
        "/x/foo/bar" => {
            path: "/x/{foo}/bar",
            value: 1,
            params: {
                "foo" => "foo"
            }
        }
        "/x/foo/baz" => {
            path: "/x/{bar}/baz",
            value: 2,
            params: {
                "bar" => "foo"
            }
        }
        "/y/foo/baz" => {
            path: "/{foo}/{bar}/baz",
            value: 4,
            params: {
                "foo" => "y",
                "bar" => "foo"
            }
        }
        "/y/foo/bax" => {
            path: "/{foo}/{baz}/bax",
            value: 3,
            params: {
                "foo" => "y",
                "baz" => "foo"
            }
        }
        "/y/baz/baz" => {
            path: "/{foo}/{bar}/baz",
            value: 4,
            params: {
                "foo" => "y",
                "bar" => "baz"
            }
        }
        "/y/baz/bax/foo" => {
            path: "/{fod}/baz/bax/foo",
            value: 6,
            params: {
                "fod" => "y"
            }
        }
        "/y/baz/b/foo" => {
            path: "/{fod}/{baz}/{bax}/foo",
            value: 5,
            params: {
                "fod" => "y",
                "baz" => "baz",
                "bax" => "b"
            }
        }
        "/y/baz/bax" => {
            path: "/{foo}/baz/bax",
            value: 7,
            params: {
                "foo" => "y"
            }
        }
        "/z/bar/bay" => {
            path: "/{bar}/{bay}/bay",
            value: 8,
            params: {
                "bar" => "z",
                "bay" => "bar"
            }
        }
        "/s" => {
            path: "/s",
            value: 9
        }
        "/s/s" => {
            path: "/s/s",
            value: 10
        }
        "/s/s/s" => {
            path: "/s/s/s",
            value: 11
        }
        "/s/s/s/s" => {
            path: "/s/s/s/s",
            value: 12
        }
        "/s/s/s/x" => {
            path: "/s/s/{s}/x",
            value: 13,
            params: {
                "s" => "s"
            }
        }
        "/s/s/s/d" => {
            path: "/s/s/{y}/d",
            value: 14,
            params: {
                "y" => "s"
            }
        }
    });
}

#[test]
fn blog() {
    let mut router = Router::new();
    router.insert("/{page}", 1);
    router.insert("/posts/{year}/{month}/{post}", 2);
    router.insert("/posts/{year}/{month}/index", 3);
    router.insert("/posts/{year}/top", 4);
    router.insert("/static/{path:*}", 5);
    router.insert("/favicon.ico", 6);

    assert_router_matches!(router, {
        "/about" => {
            path: "/{page}",
            value: 1,
            params: {
                "page" => "about"
            }
        }
        "/posts/2021/01/rust" => {
            path: "/posts/{year}/{month}/{post}",
            value: 2,
            params: {
                "year" => "2021",
                "month" => "01",
                "post" => "rust"
            }
        }
        "/posts/2021/01/index" => {
            path: "/posts/{year}/{month}/index",
            value: 3,
            params: {
                "year" => "2021",
                "month" => "01"
            }
        }
        "/posts/2021/top" => {
            path: "/posts/{year}/top",
            value: 4,
            params: {
                "year" => "2021"
            }
        }
        "/static/foo.png" => {
            path: "/static/{path:*}",
            value: 5,
            params: {
                "path" => "foo.png"
            }
        }
        "/favicon.ico" => {
            path: "/favicon.ico",
            value: 6
        }
    });
}

#[test]
fn double_overlap() {
    let mut router = Router::new();
    router.insert("/{object}/{id}", 1);
    router.insert("/secret/{id}/path", 2);
    router.insert("/secret/978", 3);
    router.insert("/other/{object}/{id}/", 4);
    router.insert("/other/an_object/{id}", 5);
    router.insert("/other/static/path", 6);
    router.insert("/other/long/static/path/", 7);

    assert_router_matches!(router, {
        "/secret/978/path" => {
            path: "/secret/{id}/path",
            value: 2,
            params: {
                "id" => "978"
            }
        }
        "/some_object/978" => {
            path: "/{object}/{id}",
            value: 1,
            params: {
                "object" => "some_object",
                "id" => "978"
            }
        }
        "/secret/978" => {
            path: "/secret/978",
            value: 3
        }
        "/super_secret/978/" => None
        "/other/object/1/" => {
            path: "/other/{object}/{id}/",
            value: 4,
            params: {
                "object" => "object",
                "id" => "1"
            }
        }
        "/other/object/1/2" => None
        "/other/an_object/1" => {
            path: "/other/an_object/{id}",
            value: 5,
            params: {
                "id" => "1"
            }
        }
        "/other/static/path" => {
            path: "/other/static/path",
            value: 6
        }
        "/other/long/static/path/" => {
            path: "/other/long/static/path/",
            value: 7
        }
    });
}

#[test]
fn catchall_off_by_one() {
    let mut router = Router::new();
    router.insert("/foo/{catchall:*}", 1);
    router.insert("/bar", 2);
    router.insert("/bar/", 3);
    router.insert("/bar/{catchall:*}", 4);

    assert_router_matches!(router, {
        "/foo" => None
        "/foo/" => None
        "/foo/x" => {
            path: "/foo/{catchall:*}",
            value: 1,
            params: {
                "catchall" => "x"
            }
        }
        "/bar" => {
            path: "/bar",
            value: 2
        }
        "/bar/" => {
            path: "/bar/",
            value: 3
        }
        "/bar/x" => {
            path: "/bar/{catchall:*}",
            value: 4,
            params: {
                "catchall" => "x"
            }
        }
    });
}

#[test]
fn overlap() {
    let mut router = Router::new();
    router.insert("/foo", 1);
    router.insert("/bar", 2);
    router.insert("/{bar:*}", 3);
    router.insert("/baz", 4);
    router.insert("/baz/", 5);
    router.insert("/baz/x", 6);
    router.insert("/baz/{xxx}", 7);
    router.insert("/", 8);
    router.insert("/xxx/{x:*}", 9);
    router.insert("/xxx/", 10);

    assert_router_matches!(router, {
        "/foo" => {
            path: "/foo",
            value: 1
        }
        "/bar" => {
            path: "/bar",
            value: 2
        }
        "/baz" => {
            path: "/baz",
            value: 4
        }
        "/baz/" => {
            path: "/baz/",
            value: 5
        }
        "/baz/x" => {
            path: "/baz/x",
            value: 6
        }
        "/???" => {
            path: "/{bar:*}",
            value: 3,
            params: {
                "bar" => "???"
            }
        }
        "/" => {
            path: "/",
            value: 8
        }
        "" => None
        "/xxx/y" => {
            path: "/xxx/{x:*}",
            value: 9,
            params: {
                "x" => "y"
            }
        }
        "/xxx/" => {
            path: "/xxx/",
            value: 10
        }
        "/xxx" => {
            path: "/{bar:*}",
            value: 3,
            params: {
                "bar" => "xxx"
            }
        }
    });
}

#[test]
fn missing_trailing_slash_param() {
    let mut router = Router::new();
    router.insert("/foo/{object}/{id}", 1);
    router.insert("/foo/bar/baz", 2);
    router.insert("/foo/secret/978/", 3);

    assert_router_matches!(router, {
        "/foo/secret/978/" => {
            path: "/foo/secret/978/",
            value: 3
        }
        "/foo/secret/978" => {
            path: "/foo/{object}/{id}",
            value: 1,
            params: {
                "object" => "secret",
                "id" => "978"
            }
        }
    });
}

#[test]
fn extra_trailing_slash_param() {
    let mut router = Router::new();
    router.insert("/foo/{object}/{id}", 1);
    router.insert("/foo/bar/baz", 2);
    router.insert("/foo/secret/978", 3);

    assert_router_matches!(router, {
        "/foo/secret/978/" => None
        "/foo/secret/978" => {
            path: "/foo/secret/978",
            value: 3
        }
    });
}

#[test]
fn missing_trailing_slash_catch_all() {
    let mut router = Router::new();
    router.insert("/foo/{bar:*}", 1);
    router.insert("/foo/bar/baz", 2);
    router.insert("/foo/secret/978/", 3);

    assert_router_matches!(router, {
        "/foo/secret/978" => {
            path: "/foo/{bar:*}",
            value: 1,
            params: {
                "bar" => "secret/978"
            }
        }
        "/foo/secret/978/" => {
            path: "/foo/secret/978/",
            value: 3
        }
    });
}

#[test]
fn extra_trailing_slash_catch_all() {
    let mut router = Router::new();
    router.insert("/foo/{bar:*}", 1);
    router.insert("/foo/bar/baz", 2);
    router.insert("/foo/secret/978", 3);

    assert_router_matches!(router, {
        "/foo/secret/978/" => {
            path: "/foo/{bar:*}",
            value: 1,
            params: {
                "bar" => "secret/978/"
            }
        }
        "/foo/secret/978" => {
            path: "/foo/secret/978",
            value: 3
        }
    });
}

#[test]
fn double_overlap_trailing_slash() {
    let mut router = Router::new();
    router.insert("/{object}/{id}", 1);
    router.insert("/secret/{id}/path", 2);
    router.insert("/secret/978/", 3);
    router.insert("/other/{object}/{id}/", 4);
    router.insert("/other/an_object/{id}", 5);
    router.insert("/other/static/path", 6);
    router.insert("/other/long/static/path/", 7);

    assert_router_matches!(router, {
        "/secret/978/path/" => None
        "/object/id/" => None
        "/object/id/path" => None
        "/other/object/1" => None
        "/other/object/1/2" => None
        "/other/an_object/1/" => {
            path: "/other/{object}/{id}/",
            value: 4,
            params: {
                "object" => "an_object",
                "id" => "1"
            }
        }
        "/other/static/path/" => {
            path: "/other/{object}/{id}/",
            value: 4,
            params: {
                "object" => "static",
                "id" => "path"
            }
        }
        "/other/long/static/path" => None
        "/other/object/static/path" => None
    });
}

#[test]
fn trailing_slash_overlap() {
    let mut router = Router::new();
    router.insert("/foo/{x}/baz/", 1);
    router.insert("/foo/{x}/baz", 2);
    router.insert("/foo/bar/bar", 3);

    assert_router_matches!(router, {
        "/foo/x/baz/" => {
            path: "/foo/{x}/baz/",
            value: 1,
            params: {
                "x" => "x"
            }
        }
        "/foo/x/baz" => {
            path: "/foo/{x}/baz",
            value: 2,
            params: {
                "x" => "x"
            }
        }
        "/foo/bar/bar" => {
            path: "/foo/bar/bar",
            value: 3
        }
    });
}

#[test]
fn trailing_slash() {
    let mut router = Router::new();
    router.insert("/hi", 1);
    router.insert("/b/", 2);
    router.insert("/search/{query}", 3);
    router.insert("/cmd/{tool}/", 4);
    router.insert("/src/{filepath:*}", 5);
    router.insert("/x", 6);
    router.insert("/x/y", 7);
    router.insert("/y/", 8);
    router.insert("/y/z", 9);
    router.insert("/0/{id}", 10);
    router.insert("/0/{id}/1", 11);
    router.insert("/1/{id}/", 12);
    router.insert("/1/{id}/2", 13);
    router.insert("/aa", 14);
    router.insert("/a/", 15);
    router.insert("/admin", 16);
    router.insert("/admin/static", 17);
    router.insert("/admin/{category}", 18);
    router.insert("/admin/{category}/{page}", 19);
    router.insert("/doc", 20);
    router.insert("/doc/rust_faq.html", 21);
    router.insert("/doc/rust1.26.html", 22);
    router.insert("/no/a", 23);
    router.insert("/no/b", 24);
    router.insert("/no/a/b/{other:*}", 25);
    router.insert("/api/{page}/{name}", 26);
    router.insert("/api/hello/{name}/bar/", 27);
    router.insert("/api/bar/{name}", 28);
    router.insert("/api/baz/foo", 29);
    router.insert("/api/baz/foo/bar", 30);
    router.insert("/foo/{p}", 31);

    assert_router_matches!(router, {
        "/hi/" => None
        "/b" => None
        "/search/rustacean/" => None
        "/cmd/vet" => None
        "/src" => None
        "/src/" => None
        "/x/" => None
        "/y" => None
        "/0/rust/" => None
        "/1/rust" => None
        "/a" => None
        "/admin/" => None
        "/doc/" => None
        "/admin/static/" => None
        "/admin/cfg/" => None
        "/admin/cfg/users/" => None
        "/api/hello/x/bar" => None
        "/api/baz/foo/" => None
        "/api/baz/bax/" => None
        "/api/bar/huh/" => None
        "/api/baz/foo/bar/" => None
        "/api/world/abc/" => None
        "/foo/pp/" => None
        "/" => None
        "/no" => None
        "/no/" => None
        "/no/a/" => None
        "/no/a/b/" => None
        "/_" => None
        "/_/" => None
        "/api" => None
        "/api/" => None
        "/api/hello/x/foo" => None
        "/api/baz/foo/bad" => None
        "/foo/p/p" => None
    });
}

#[test]
fn backtracking_trailing_slash() {
    let mut router = Router::new();
    router.insert("/a/{b}/{c}", 1);
    router.insert("/a/b/{c}/d/", 2);

    assert_router_matches!(router, {
        "/a/b/c/d" => None
    });
}

#[test]
fn root_trailing_slash() {
    let mut router = Router::new();
    router.insert("/foo", 1);
    router.insert("/bar", 2);
    router.insert("/{baz}", 3);

    assert_router_matches!(router, {
        "/" => None
    });
}

#[test]
fn catchall_overlap() {
    let mut router = Router::new();
    router.insert("/yyy/{x:*}", 1);
    router.insert("/yyy{x:*}", 2);

    assert_router_matches!(router, {
        "/yyy/y" => {
            path: "/yyy/{x:*}",
            value: 1,
            params: {
                "x" => "y"
            }
        }
        "/yyy/" => {
            path: "/yyy{x:*}",
            value: 2,
            params: {
                "x" => "/"
            }
        }
    });
}

#[test]
#[ignore = "escaping not yet implemented"]
fn escaped() {
    let mut router = Router::new();
    router.insert("/", 1);
    router.insert("/{{", 2);
    router.insert("/}}", 3);
    router.insert("/{{x", 4);
    router.insert("/}}y{{", 5);
    router.insert("/xy{{", 6);
    router.insert("/{{/xyz", 7);
    router.insert("/{ba{{r}", 8);
    router.insert("/{ba{{r}/", 9);
    router.insert("/{ba{{r}/x", 10);
    router.insert("/baz/{xxx}", 11);
    router.insert("/baz/{xxx}/xy{{", 12);
    router.insert("/baz/{xxx}/}}xy{{{{", 13);
    router.insert("/{{/{x}", 14);
    router.insert("/xxx/", 15);
    router.insert("/xxx/{x}}{{}}}}{{}}{{{{}}y}", 16);

    assert_router_matches!(router, {
        "/" => {
            path: "/",
            value: 1
        }
        "/{" => {
            path: "/{{",
            value: 2
        }
        "/}" => {
            path: "/}}",
            value: 3
        }
        "/{x" => {
            path: "/{{x",
            value: 4
        }
        "/}y{" => {
            path: "/}}y{{",
            value: 5
        }
        "/xy{" => {
            path: "/xy{{",
            value: 6
        }
        "/{/xyz" => {
            path: "/{{/xyz",
            value: 7
        }
        "/foo" => {
            path: "/{ba{{r}",
            value: 8,
            params: {
                "ba{r" => "foo"
            }
        }
        "/{{" => {
            path: "/{ba{{r}",
            value: 8,
            params: {
                "ba{r" => "{{"
            }
        }
        "/{{}}/" => {
            path: "/{ba{{r}/",
            value: 9,
            params: {
                "ba{r" => "{{}}"
            }
        }
        "/{{}}{{/x" => {
            path: "/{ba{{r}/x",
            value: 10,
            params: {
                "ba{r" => "{{}}{{"
            }
        }
        "/baz/x" => {
            path: "/baz/{xxx}",
            value: 11,
            params: {
                "xxx" => "x"
            }
        }
        "/baz/x/xy{" => {
            path: "/baz/{xxx}/xy{{",
            value: 12,
            params: {
                "xxx" => "x"
            }
        }
        "/baz/x/xy{{" => None
        "/baz/x/}xy{{" => {
            path: "/baz/{xxx}/}}xy{{{{",
            value: 13,
            params: {
                "xxx" => "x"
            }
        }
        "/{/{{" => {
            path: "/{{/{x}",
            value: 14,
            params: {
                "x" => "{{"
            }
        }
        "/xxx" => {
            path: "/{ba{{r}",
            value: 8,
            params: {
                "ba{r" => "xxx"
            }
        }
        "/xxx/" => {
            path: "/xxx/",
            value: 15
        }
        "/xxx/foo" => {
            path: "/xxx/{x}}{{}}}}{{}}{{{{}}y}",
            value: 16,
            params: {
                "x}{}}{}{{}y" => "foo"
            }
        }
    });
}

#[test]
fn basic() {
    let mut router = Router::new();
    router.insert("/hi", 1);
    router.insert("/contact", 2);
    router.insert("/co", 3);
    router.insert("/c", 4);
    router.insert("/a", 5);
    router.insert("/ab", 6);
    router.insert("/doc/", 7);
    router.insert("/doc/rust_faq.html", 8);
    router.insert("/doc/rust1.26.html", 9);
    router.insert("ʯ", 10);
    router.insert("β", 11);
    router.insert("/sd!here", 12);
    router.insert("/sd$here", 13);
    router.insert("/sd&here", 14);
    router.insert("/sd'here", 15);
    router.insert("/sd(here", 16);
    router.insert("/sd)here", 17);
    router.insert("/sd+here", 18);
    router.insert("/sd,here", 19);
    router.insert("/sd;here", 20);
    router.insert("/sd=here", 21);

    assert_router_matches!(router, {
        "/a" => {
            path: "/a",
            value: 5
        }
        "/" => None
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
        "/con" => None
        "/cona" => None
        "/no" => None
        "/ab" => {
            path: "/ab",
            value: 6
        }
        "ʯ" => {
            path: "ʯ",
            value: 10
        }
        "β" => {
            path: "β",
            value: 11
        }
        "/sd!here" => {
            path: "/sd!here",
            value: 12
        }
        "/sd$here" => {
            path: "/sd$here",
            value: 13
        }
        "/sd&here" => {
            path: "/sd&here",
            value: 14
        }
        "/sd'here" => {
            path: "/sd'here",
            value: 15
        }
        "/sd(here" => {
            path: "/sd(here",
            value: 16
        }
        "/sd)here" => {
            path: "/sd)here",
            value: 17
        }
        "/sd+here" => {
            path: "/sd+here",
            value: 18
        }
        "/sd,here" => {
            path: "/sd,here",
            value: 19
        }
        "/sd;here" => {
            path: "/sd;here",
            value: 20
        }
        "/sd=here" => {
            path: "/sd=here",
            value: 21
        }
    });
}

#[test]
fn wildcard() {
    let mut router = Router::new();
    router.insert("/", 1);
    router.insert("/cmd/{tool}/", 2);
    router.insert("/cmd/{tool2}/{sub}", 3);
    router.insert("/cmd/whoami", 4);
    router.insert("/cmd/whoami/root", 5);
    router.insert("/cmd/whoami/root/", 6);
    router.insert("/src", 7);
    router.insert("/src/", 8);
    router.insert("/src/{filepath:*}", 9);
    router.insert("/search/", 10);
    router.insert("/search/{query}", 11);
    router.insert("/search/actix-we", 12);
    router.insert("/search/google", 13);
    router.insert("/user_{name}", 14);
    router.insert("/user_{name}/about", 15);
    router.insert("/files/{dir}/{filepath:*}", 16);
    router.insert("/doc/", 17);
    router.insert("/doc/rust_faq.html", 18);
    router.insert("/doc/rust1.26.html", 19);
    router.insert("/info/{user}/public", 20);
    router.insert("/info/{user}/project/{project}", 21);
    router.insert("/info/{user}/project/rustlang", 22);
    router.insert("/aa/{xx:*}", 23);
    router.insert("/ab/{xx:*}", 24);
    router.insert("/ab/hello{xx:*}", 25);
    router.insert("/{cc}", 26);
    router.insert("/c1/{dd}/e", 27);
    router.insert("/c1/{dd}/e1", 28);
    router.insert("/{cc}/cc", 29);
    router.insert("/{cc}/{dd}/ee", 30);
    router.insert("/{cc}/{dd}/{ee}/ff", 31);
    router.insert("/{cc}/{dd}/{ee}/{ff}/gg", 32);
    router.insert("/{cc}/{dd}/{ee}/{ff}/{gg}/hh", 33);
    router.insert("/get/test/abc/", 34);
    router.insert("/get/{param}/abc/", 35);
    router.insert("/something/{paramname}/thirdthing", 36);
    router.insert("/something/secondthing/test", 37);
    router.insert("/get/abc", 38);
    router.insert("/get/{param}", 39);
    router.insert("/get/abc/123abc", 40);
    router.insert("/get/abc/{param}", 41);
    router.insert("/get/abc/123abc/xxx8", 42);
    router.insert("/get/abc/123abc/{param}", 43);
    router.insert("/get/abc/123abc/xxx8/1234", 44);
    router.insert("/get/abc/123abc/xxx8/{param}", 45);
    router.insert("/get/abc/123abc/xxx8/1234/ffas", 46);
    router.insert("/get/abc/123abc/xxx8/1234/{param}", 47);
    router.insert("/get/abc/123abc/xxx8/1234/kkdd/12c", 48);
    router.insert("/get/abc/123abc/xxx8/1234/kkdd/{param}", 49);
    router.insert("/get/abc/{param}/test", 50);
    router.insert("/get/abc/123abd/{param}", 51);
    router.insert("/get/abc/123abddd/{param}", 52);
    router.insert("/get/abc/123/{param}", 53);
    router.insert("/get/abc/123abg/{param}", 54);
    router.insert("/get/abc/123abf/{param}", 55);
    router.insert("/get/abc/123abfff/{param}", 56);

    assert_router_matches!(router, {
        "/" => {
            path: "/",
            value: 1
        }
        "/cmd/test" => None
        "/cmd/test/" => {
            path: "/cmd/{tool}/",
            value: 2,
            params: {
                "tool" => "test"
            }
        }
        "/cmd/test/3" => {
            path: "/cmd/{tool2}/{sub}",
            value: 3,
            params: {
                "tool2" => "test",
                "sub" => "3"
            }
        }
        "/cmd/who" => None
        "/cmd/who/" => {
            path: "/cmd/{tool}/",
            value: 2,
            params: {
                "tool" => "who"
            }
        }
        "/cmd/whoami" => {
            path: "/cmd/whoami",
            value: 4
        }
        "/cmd/whoami/" => {
            path: "/cmd/{tool}/",
            value: 2,
            params: {
                "tool" => "whoami"
            }
        }
        "/cmd/whoami/r" => {
            path: "/cmd/{tool2}/{sub}",
            value: 3,
            params: {
                "tool2" => "whoami",
                "sub" => "r"
            }
        }
        "/cmd/whoami/r/" => None
        "/cmd/whoami/root" => {
            path: "/cmd/whoami/root",
            value: 5
        }
        "/cmd/whoami/root/" => {
            path: "/cmd/whoami/root/",
            value: 6
        }
        "/src" => {
            path: "/src",
            value: 7
        }
        "/src/" => {
            path: "/src/",
            value: 8
        }
        "/src/some/file.png" => {
            path: "/src/{filepath:*}",
            value: 9,
            params: {
                "filepath" => "some/file.png"
            }
        }
        "/search/" => {
            path: "/search/",
            value: 10
        }
        "/search/actix" => {
            path: "/search/{query}",
            value: 11,
            params: {
                "query" => "actix"
            }
        }
        "/search/actix-we" => {
            path: "/search/actix-we",
            value: 12
        }
        "/search/someth!ng+in+ünìcodé" => {
            path: "/search/{query}",
            value: 11,
            params: {
                "query" => "someth!ng+in+ünìcodé"
            }
        }
        "/search/someth!ng+in+ünìcodé/" => None
        "/user_rustacean" => {
            path: "/user_{name}",
            value: 14,
            params: {
                "name" => "rustacean"
            }
        }
        "/user_rustacean/about" => {
            path: "/user_{name}/about",
            value: 15,
            params: {
                "name" => "rustacean"
            }
        }
        "/files/js/inc/framework.js" => {
            path: "/files/{dir}/{filepath:*}",
            value: 16,
            params: {
                "dir" => "js",
                "filepath" => "inc/framework.js"
            }
        }
        "/info/gordon/public" => {
            path: "/info/{user}/public",
            value: 20,
            params: {
                "user" => "gordon"
            }
        }
        "/info/gordon/project/rust" => {
            path: "/info/{user}/project/{project}",
            value: 21,
            params: {
                "user" => "gordon",
                "project" => "rust"
            }
        }
        "/info/gordon/project/rustlang" => {
            path: "/info/{user}/project/rustlang",
            value: 22,
            params: {
                "user" => "gordon"
            }
        }
        "/aa/" => None
        "/aa/aa" => {
            path: "/aa/{xx:*}",
            value: 23,
            params: {
                "xx" => "aa"
            }
        }
        "/ab/ab" => {
            path: "/ab/{xx:*}",
            value: 24,
            params: {
                "xx" => "ab"
            }
        }
        "/ab/hello-world" => {
            path: "/ab/hello{xx:*}",
            value: 25,
            params: {
                "xx" => "-world"
            }
        }
        "/a" => {
            path: "/{cc}",
            value: 26,
            params: {
                "cc" => "a"
            }
        }
        "/all" => {
            path: "/{cc}",
            value: 26,
            params: {
                "cc" => "all"
            }
        }
        "/d" => {
            path: "/{cc}",
            value: 26,
            params: {
                "cc" => "d"
            }
        }
        "/ad" => {
            path: "/{cc}",
            value: 26,
            params: {
                "cc" => "ad"
            }
        }
        "/dd" => {
            path: "/{cc}",
            value: 26,
            params: {
                "cc" => "dd"
            }
        }
        "/dddaa" => {
            path: "/{cc}",
            value: 26,
            params: {
                "cc" => "dddaa"
            }
        }
        "/aa" => {
            path: "/{cc}",
            value: 26,
            params: {
                "cc" => "aa"
            }
        }
        "/aaa" => {
            path: "/{cc}",
            value: 26,
            params: {
                "cc" => "aaa"
            }
        }
        "/aaa/cc" => {
            path: "/{cc}/cc",
            value: 29,
            params: {
                "cc" => "aaa"
            }
        }
        "/a" => {
            path: "/{cc}",
            value: 26,
            params: {
                "cc" => "a"
            }
        }
        "/ab" => {
            path: "/{cc}",
            value: 26,
            params: {
                "cc" => "ab"
            }
        }
        "/abb/cc" => {
            path: "/{cc}/cc",
            value: 29,
            params: {
                "cc" => "abb"
            }
        }
        "/allxxxx" => {
            path: "/{cc}",
            value: 26,
            params: {
                "cc" => "allxxxx"
            }
        }
        "/alldd" => {
            path: "/{cc}",
            value: 26,
            params: {
                "cc" => "alldd"
            }
        }
        "/all/cc" => {
            path: "/{cc}/cc",
            value: 29,
            params: {
                "cc" => "all"
            }
        }
        "/a/cc" => {
            path: "/{cc}/cc",
            value: 29,
            params: {
                "cc" => "a"
            }
        }
        "/c1/d/e" => {
            path: "/c1/{dd}/e",
            value: 27,
            params: {
                "dd" => "d"
            }
        }
        "/c1/d/e1" => {
            path: "/c1/{dd}/e1",
            value: 28,
            params: {
                "dd" => "d"
            }
        }
        "/c1/d/ee" => {
            path: "/{cc}/{dd}/ee",
            value: 30,
            params: {
                "cc" => "c1",
                "dd" => "d"
            }
        }
        "/cc/cc" => {
            path: "/{cc}/cc",
            value: 29,
            params: {
                "cc" => "cc"
            }
        }
        "/ccc/cc" => {
            path: "/{cc}/cc",
            value: 29,
            params: {
                "cc" => "ccc"
            }
        }
        "/deedwjfs/cc" => {
            path: "/{cc}/cc",
            value: 29,
            params: {
                "cc" => "deedwjfs"
            }
        }
        "/acllcc/cc" => {
            path: "/{cc}/cc",
            value: 29,
            params: {
                "cc" => "acllcc"
            }
        }
        "/get/test/abc/" => {
            path: "/get/test/abc/",
            value: 34
        }
        "/get/te/abc/" => {
            path: "/get/{param}/abc/",
            value: 35,
            params: {
                "param" => "te"
            }
        }
        "/get/testaa/abc/" => {
            path: "/get/{param}/abc/",
            value: 35,
            params: {
                "param" => "testaa"
            }
        }
        "/get/xx/abc/" => {
            path: "/get/{param}/abc/",
            value: 35,
            params: {
                "param" => "xx"
            }
        }
        "/get/tt/abc/" => {
            path: "/get/{param}/abc/",
            value: 35,
            params: {
                "param" => "tt"
            }
        }
        "/get/a/abc/" => {
            path: "/get/{param}/abc/",
            value: 35,
            params: {
                "param" => "a"
            }
        }
        "/get/t/abc/" => {
            path: "/get/{param}/abc/",
            value: 35,
            params: {
                "param" => "t"
            }
        }
        "/get/aa/abc/" => {
            path: "/get/{param}/abc/",
            value: 35,
            params: {
                "param" => "aa"
            }
        }
        "/get/abas/abc/" => {
            path: "/get/{param}/abc/",
            value: 35,
            params: {
                "param" => "abas"
            }
        }
        "/something/secondthing/test" => {
            path: "/something/secondthing/test",
            value: 37
        }
        "/something/abcdad/thirdthing" => {
            path: "/something/{paramname}/thirdthing",
            value: 36,
            params: {
                "paramname" => "abcdad"
            }
        }
        "/something/secondthingaaaa/thirdthing" => {
            path: "/something/{paramname}/thirdthing",
            value: 36,
            params: {
                "paramname" => "secondthingaaaa"
            }
        }
        "/something/se/thirdthing" => {
            path: "/something/{paramname}/thirdthing",
            value: 36,
            params: {
                "paramname" => "se"
            }
        }
        "/something/s/thirdthing" => {
            path: "/something/{paramname}/thirdthing",
            value: 36,
            params: {
                "paramname" => "s"
            }
        }
        "/c/d/ee" => {
            path: "/{cc}/{dd}/ee",
            value: 30,
            params: {
                "cc" => "c",
                "dd" => "d"
            }
        }
        "/c/d/e/ff" => {
            path: "/{cc}/{dd}/{ee}/ff",
            value: 31,
            params: {
                "cc" => "c",
                "dd" => "d",
                "ee" => "e"
            }
        }
        "/c/d/e/f/gg" => {
            path: "/{cc}/{dd}/{ee}/{ff}/gg",
            value: 32,
            params: {
                "cc" => "c",
                "dd" => "d",
                "ee" => "e",
                "ff" => "f"
            }
        }
        "/c/d/e/f/g/hh" => {
            path: "/{cc}/{dd}/{ee}/{ff}/{gg}/hh",
            value: 33,
            params: {
                "cc" => "c",
                "dd" => "d",
                "ee" => "e",
                "ff" => "f",
                "gg" => "g"
            }
        }
        "/cc/dd/ee/ff/gg/hh" => {
            path: "/{cc}/{dd}/{ee}/{ff}/{gg}/hh",
            value: 33,
            params: {
                "cc" => "cc",
                "dd" => "dd",
                "ee" => "ee",
                "ff" => "ff",
                "gg" => "gg"
            }
        }
        "/get/abc" => {
            path: "/get/abc",
            value: 38
        }
        "/get/a" => {
            path: "/get/{param}",
            value: 39,
            params: {
                "param" => "a"
            }
        }
        "/get/abz" => {
            path: "/get/{param}",
            value: 39,
            params: {
                "param" => "abz"
            }
        }
        "/get/12a" => {
            path: "/get/{param}",
            value: 39,
            params: {
                "param" => "12a"
            }
        }
        "/get/abcd" => {
            path: "/get/{param}",
            value: 39,
            params: {
                "param" => "abcd"
            }
        }
        "/get/abc/123abc" => {
            path: "/get/abc/123abc",
            value: 40
        }
        "/get/abc/12" => {
            path: "/get/abc/{param}",
            value: 41,
            params: {
                "param" => "12"
            }
        }
        "/get/abc/123a" => {
            path: "/get/abc/{param}",
            value: 41,
            params: {
                "param" => "123a"
            }
        }
        "/get/abc/xyz" => {
            path: "/get/abc/{param}",
            value: 41,
            params: {
                "param" => "xyz"
            }
        }
        "/get/abc/123abcddxx" => {
            path: "/get/abc/{param}",
            value: 41,
            params: {
                "param" => "123abcddxx"
            }
        }
        "/get/abc/123abc/xxx8" => {
            path: "/get/abc/123abc/xxx8",
            value: 42
        }
        "/get/abc/123abc/x" => {
            path: "/get/abc/123abc/{param}",
            value: 43,
            params: {
                "param" => "x"
            }
        }
        "/get/abc/123abc/xxx" => {
            path: "/get/abc/123abc/{param}",
            value: 43,
            params: {
                "param" => "xxx"
            }
        }
        "/get/abc/123abc/abc" => {
            path: "/get/abc/123abc/{param}",
            value: 43,
            params: {
                "param" => "abc"
            }
        }
        "/get/abc/123abc/xxx8xxas" => {
            path: "/get/abc/123abc/{param}",
            value: 43,
            params: {
                "param" => "xxx8xxas"
            }
        }
        "/get/abc/123abc/xxx8/1234" => {
            path: "/get/abc/123abc/xxx8/1234",
            value: 44
        }
        "/get/abc/123abc/xxx8/1" => {
            path: "/get/abc/123abc/xxx8/{param}",
            value: 45,
            params: {
                "param" => "1"
            }
        }
        "/get/abc/123abc/xxx8/123" => {
            path: "/get/abc/123abc/xxx8/{param}",
            value: 45,
            params: {
                "param" => "123"
            }
        }
        "/get/abc/123abc/xxx8/78k" => {
            path: "/get/abc/123abc/xxx8/{param}",
            value: 45,
            params: {
                "param" => "78k"
            }
        }
        "/get/abc/123abc/xxx8/1234xxxd" => {
            path: "/get/abc/123abc/xxx8/{param}",
            value: 45,
            params: {
                "param" => "1234xxxd"
            }
        }
        "/get/abc/123abc/xxx8/1234/ffas" => {
            path: "/get/abc/123abc/xxx8/1234/ffas",
            value: 46
        }
        "/get/abc/123abc/xxx8/1234/f" => {
            path: "/get/abc/123abc/xxx8/1234/{param}",
            value: 47,
            params: {
                "param" => "f"
            }
        }
        "/get/abc/123abc/xxx8/1234/ffa" => {
            path: "/get/abc/123abc/xxx8/1234/{param}",
            value: 47,
            params: {
                "param" => "ffa"
            }
        }
        "/get/abc/123abc/xxx8/1234/kka" => {
            path: "/get/abc/123abc/xxx8/1234/{param}",
            value: 47,
            params: {
                "param" => "kka"
            }
        }
        "/get/abc/123abc/xxx8/1234/ffas321" => {
            path: "/get/abc/123abc/xxx8/1234/{param}",
            value: 47,
            params: {
                "param" => "ffas321"
            }
        }
        "/get/abc/123abc/xxx8/1234/kkdd/12c" => {
            path: "/get/abc/123abc/xxx8/1234/kkdd/12c",
            value: 48
        }
        "/get/abc/123abc/xxx8/1234/kkdd/1" => {
            path: "/get/abc/123abc/xxx8/1234/kkdd/{param}",
            value: 49,
            params: {
                "param" => "1"
            }
        }
        "/get/abc/123abc/xxx8/1234/kkdd/12" => {
            path: "/get/abc/123abc/xxx8/1234/kkdd/{param}",
            value: 49,
            params: {
                "param" => "12"
            }
        }
        "/get/abc/123abc/xxx8/1234/kkdd/12" => {
            path: "/get/abc/123abc/xxx8/1234/kkdd/{param}",
            value: 49,
            params: {
                "param" => "12"
            }
        }
        "/get/abc/123abc/xxx8/1234/kkdd/34" => {
            path: "/get/abc/123abc/xxx8/1234/kkdd/{param}",
            value: 49,
            params: {
                "param" => "34"
            }
        }
        "/get/abc/123abc/xxx8/1234/kkdd/12c2e3" => {
            path: "/get/abc/123abc/xxx8/1234/kkdd/{param}",
            value: 49,
            params: {
                "param" => "12c2e3"
            }
        }
        "/get/abc/12/test" => {
            path: "/get/abc/{param}/test",
            value: 50,
            params: {
                "param" => "12"
            }
        }
        "/get/abc/123abdd/test" => {
            path: "/get/abc/{param}/test",
            value: 50,
            params: {
                "param" => "123abdd"
            }
        }
        "/get/abc/123abdddf/test" => {
            path: "/get/abc/{param}/test",
            value: 50,
            params: {
                "param" => "123abdddf"
            }
        }
        "/get/abc/123ab/test" => {
            path: "/get/abc/{param}/test",
            value: 50,
            params: {
                "param" => "123ab"
            }
        }
        "/get/abc/123abgg/test" => {
            path: "/get/abc/{param}/test",
            value: 50,
            params: {
                "param" => "123abgg"
            }
        }
        "/get/abc/123abff/test" => {
            path: "/get/abc/{param}/test",
            value: 50,
            params: {
                "param" => "123abff"
            }
        }
        "/get/abc/123abffff/test" => {
            path: "/get/abc/{param}/test",
            value: 50,
            params: {
                "param" => "123abffff"
            }
        }
        "/get/abc/123abd/test" => {
            path: "/get/abc/123abd/{param}",
            value: 51,
            params: {
                "param" => "test"
            }
        }
        "/get/abc/123abddd/test" => {
            path: "/get/abc/123abddd/{param}",
            value: 52,
            params: {
                "param" => "test"
            }
        }
        "/get/abc/123/test22" => {
            path: "/get/abc/123/{param}",
            value: 53,
            params: {
                "param" => "test22"
            }
        }
        "/get/abc/123abg/test" => {
            path: "/get/abc/123abg/{param}",
            value: 54,
            params: {
                "param" => "test"
            }
        }
        "/get/abc/123abf/testss" => {
            path: "/get/abc/123abf/{param}",
            value: 55,
            params: {
                "param" => "testss"
            }
        }
        "/get/abc/123abfff/te" => {
            path: "/get/abc/123abfff/{param}",
            value: 56,
            params: {
                "param" => "te"
            }
        }
    });
}
