//! Tests sourced from `matchit` (MIT AND BSD-3-Clause)
//! <https://github.com/ibraheemdev/matchit/blob/v0.8.4/tests/match.rs>

#![allow(clippy::too_many_lines)]

use std::error::Error;
use wayfind::Router;

#[path = "./utils.rs"]
mod utils;

// https://github.com/ibraheemdev/matchit/issues/22
#[test]
fn partial_overlap() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/foo_bar", "Welcome!")?;
    router.insert("/foo/bar", "Welcome!")?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /foo
          ├─ /bar ○
          ╰─ _bar ○
    "#);

    assert_router_matches!(router, {
        "/foo/" => None
    });

    let mut router = Router::new();
    router.insert("/foo", "Welcome!")?;
    router.insert("/foo/bar", "Welcome!")?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /foo ○
          ╰─ /bar ○
    "#);

    assert_router_matches!(router, {
        "/foo/" => None
    });

    Ok(())
}

// https://github.com/ibraheemdev/matchit/issues/31
#[test]
fn wildcard_overlap() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/path/foo", "foo")?;
    router.insert("/path/{*rest}", "wildcard")?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /path/
            ├─ foo ○
            ╰─ {*rest} ○
    "#);

    assert_router_matches!(router, {
        "/path/foo" => {
            route: "/path/foo",
            data: "foo"
        }
        "/path/bar" => {
            route: "/path/{*rest}",
            data: "wildcard",
            params: {
                "rest" => "bar"
            }
        }
        "/path/foo/" => {
            route: "/path/{*rest}",
            data: "wildcard",
            params: {
                "rest" => "foo/"
            }
        }
    });

    let mut router = Router::new();
    router.insert("/path/foo/{arg}", "foo")?;
    router.insert("/path/{*rest}", "wildcard")?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /path/
            ├─ foo/
            │     ╰─ {arg} ○
            ╰─ {*rest} ○
    "#);

    assert_router_matches!(router, {
        "/path/foo/myarg" => {
            route: "/path/foo/{arg}",
            data: "foo",
            params: {
                "arg" => "myarg"
            }
        }
        "/path/foo/myarg/" => {
            route: "/path/{*rest}",
            data: "wildcard",
            params: {
                "rest" => "foo/myarg/"
            }
        }
        "/path/foo/myarg/bar/baz" => {
            route: "/path/{*rest}",
            data: "wildcard",
            params: {
                "rest" => "foo/myarg/bar/baz"
            }
        }
    });

    Ok(())
}

// https://github.com/ibraheemdev/matchit/issues/12
#[test]
fn overlapping_param_backtracking() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{object}/{id}", "object with id")?;
    router.insert("/secret/{id}/path", "secret with id and path")?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ secret/
       │        ╰─ {id}
       │              ╰─ /path ○
       ╰─ {object}
                 ╰─ /
                    ╰─ {id} ○
    "#);

    assert_router_matches!(router, {
        "/secret/978/path" => {
            route: "/secret/{id}/path",
            data: "secret with id and path",
            params: {
                "id" => "978"
            }
        }
        "/something/978" => {
            route: "/{object}/{id}",
            data: "object with id",
            params: {
                "object" => "something",
                "id" => "978"
            }
        }
        "/secret/978" => {
            route: "/{object}/{id}",
            data: "object with id",
            params: {
                "object" => "secret",
                "id" => "978"
            }
        }
    });

    Ok(())
}

// https://github.com/ibraheemdev/matchit/issues/42
#[test]
fn bare_catchall() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("{*foo}", 1)?;
    router.insert("foo/{*bar}", 2)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ├─ foo/
    │     ╰─ {*bar} ○
    ╰─ {*foo} ○
    "#);

    assert_router_matches!(router, {
        "x/y" => {
            route: "{*foo}",
            data: 1,
            params: {
                "foo" => "x/y"
            }
        }
        "/x/y" => {
            route: "{*foo}",
            data: 1,
            params: {
                "foo" => "/x/y"
            }
        }
        "/foo/x/y" => {
            route: "{*foo}",
            data: 1,
            params: {
                "foo" => "/foo/x/y"
            }
        }
        "foo/x/y" => {
            route: "foo/{*bar}",
            data: 2,
            params: {
                "bar" => "x/y"
            }
        }
    });

    Ok(())
}

#[test]
fn normalized() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/x/{foo}/bar", 1)?;
    router.insert("/x/{bar}/baz", 2)?;
    router.insert("/{foo}/{baz}/bax", 3)?;
    router.insert("/{foo}/{bar}/baz", 4)?;
    router.insert("/{fod}/{baz}/{bax}/foo", 5)?;
    router.insert("/{fod}/baz/bax/foo", 6)?;
    router.insert("/{foo}/baz/bax", 7)?;
    router.insert("/{bar}/{bay}/bay", 8)?;
    router.insert("/s", 9)?;
    router.insert("/s/s", 10)?;
    router.insert("/s/s/s", 11)?;
    router.insert("/s/s/s/s", 12)?;
    router.insert("/s/s/{s}/x", 13)?;
    router.insert("/s/s/{y}/d", 14)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ s ○
       │  ╰─ /s ○
       │      ╰─ /
       │         ├─ s ○
       │         │  ╰─ /s ○
       │         ├─ {s}
       │         │    ╰─ /x ○
       │         ╰─ {y}
       │              ╰─ /d ○
       ├─ x/
       │   ├─ {bar}
       │   │      ╰─ /baz ○
       │   ╰─ {foo}
       │          ╰─ /bar ○
       ├─ {bar}
       │      ╰─ /
       │         ╰─ {bay}
       │                ╰─ /bay ○
       ├─ {fod}
       │      ╰─ /
       │         ├─ baz/bax/foo ○
       │         ╰─ {baz}
       │                ╰─ /
       │                   ╰─ {bax}
       │                          ╰─ /foo ○
       ╰─ {foo}
              ╰─ /
                 ├─ baz/bax ○
                 ├─ {bar}
                 │      ╰─ /baz ○
                 ╰─ {baz}
                        ╰─ /bax ○
    "#);

    assert_router_matches!(router, {
        "/x/foo/bar" => {
            route: "/x/{foo}/bar",
            data: 1,
            params: {
                "foo" => "foo"
            }
        }
        "/x/foo/baz" => {
            route: "/x/{bar}/baz",
            data: 2,
            params: {
                "bar" => "foo"
            }
        }
        "/y/foo/baz" => {
            route: "/{foo}/{bar}/baz",
            data: 4,
            params: {
                "foo" => "y",
                "bar" => "foo"
            }
        }
        "/y/foo/bax" => {
            route: "/{foo}/{baz}/bax",
            data: 3,
            params: {
                "foo" => "y",
                "baz" => "foo"
            }
        }
        "/y/baz/baz" => {
            route: "/{foo}/{bar}/baz",
            data: 4,
            params: {
                "foo" => "y",
                "bar" => "baz"
            }
        }
        "/y/baz/bax/foo" => {
            route: "/{fod}/baz/bax/foo",
            data: 6,
            params: {
                "fod" => "y"
            }
        }
        "/y/baz/b/foo" => {
            route: "/{fod}/{baz}/{bax}/foo",
            data: 5,
            params: {
                "fod" => "y",
                "baz" => "baz",
                "bax" => "b"
            }
        }
        "/y/baz/bax" => {
            route: "/{foo}/baz/bax",
            data: 7,
            params: {
                "foo" => "y"
            }
        }
        "/z/bar/bay" => {
            route: "/{bar}/{bay}/bay",
            data: 8,
            params: {
                "bar" => "z",
                "bay" => "bar"
            }
        }
        "/s" => {
            route: "/s",
            data: 9
        }
        "/s/s" => {
            route: "/s/s",
            data: 10
        }
        "/s/s/s" => {
            route: "/s/s/s",
            data: 11
        }
        "/s/s/s/s" => {
            route: "/s/s/s/s",
            data: 12
        }
        "/s/s/s/x" => {
            route: "/s/s/{s}/x",
            data: 13,
            params: {
                "s" => "s"
            }
        }
        "/s/s/s/d" => {
            route: "/s/s/{y}/d",
            data: 14,
            params: {
                "y" => "s"
            }
        }
    });

    Ok(())
}

#[test]
fn blog() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{page}", 1)?;
    router.insert("/posts/{year}/{month}/{post}", 2)?;
    router.insert("/posts/{year}/{month}/index", 3)?;
    router.insert("/posts/{year}/top", 4)?;
    router.insert("/static/{*path}", 5)?;
    router.insert("/favicon.ico", 6)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ favicon.ico ○
       ├─ posts/
       │       ╰─ {year}
       │               ╰─ /
       │                  ├─ top ○
       │                  ╰─ {month}
       │                           ╰─ /
       │                              ├─ index ○
       │                              ╰─ {post} ○
       ├─ static/
       │        ╰─ {*path} ○
       ╰─ {page} ○
    "#);

    assert_router_matches!(router, {
        "/about" => {
            route: "/{page}",
            data: 1,
            params: {
                "page" => "about"
            }
        }
        "/posts/2021/01/rust" => {
            route: "/posts/{year}/{month}/{post}",
            data: 2,
            params: {
                "year" => "2021",
                "month" => "01",
                "post" => "rust"
            }
        }
        "/posts/2021/01/index" => {
            route: "/posts/{year}/{month}/index",
            data: 3,
            params: {
                "year" => "2021",
                "month" => "01"
            }
        }
        "/posts/2021/top" => {
            route: "/posts/{year}/top",
            data: 4,
            params: {
                "year" => "2021"
            }
        }
        "/static/foo.png" => {
            route: "/static/{*path}",
            data: 5,
            params: {
                "path" => "foo.png"
            }
        }
        "/favicon.ico" => {
            route: "/favicon.ico",
            data: 6
        }
    });

    Ok(())
}

#[test]
fn double_overlap() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{object}/{id}", 1)?;
    router.insert("/secret/{id}/path", 2)?;
    router.insert("/secret/978", 3)?;
    router.insert("/other/{object}/{id}/", 4)?;
    router.insert("/other/an_object/{id}", 5)?;
    router.insert("/other/static/path", 6)?;
    router.insert("/other/long/static/path/", 7)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ other/
       │       ├─ an_object/
       │       │           ╰─ {id} ○
       │       ├─ long/static/path/ ○
       │       ├─ static/path ○
       │       ╰─ {object}
       │                 ╰─ /
       │                    ╰─ {id}
       │                          ╰─ / ○
       ├─ secret/
       │        ├─ 978 ○
       │        ╰─ {id}
       │              ╰─ /path ○
       ╰─ {object}
                 ╰─ /
                    ╰─ {id} ○
    "#);

    assert_router_matches!(router, {
        "/secret/978/path" => {
            route: "/secret/{id}/path",
            data: 2,
            params: {
                "id" => "978"
            }
        }
        "/some_object/978" => {
            route: "/{object}/{id}",
            data: 1,
            params: {
                "object" => "some_object",
                "id" => "978"
            }
        }
        "/secret/978" => {
            route: "/secret/978",
            data: 3
        }
        "/super_secret/978/" => None
        "/other/object/1/" => {
            route: "/other/{object}/{id}/",
            data: 4,
            params: {
                "object" => "object",
                "id" => "1"
            }
        }
        "/other/object/1/2" => None
        "/other/an_object/1" => {
            route: "/other/an_object/{id}",
            data: 5,
            params: {
                "id" => "1"
            }
        }
        "/other/static/path" => {
            route: "/other/static/path",
            data: 6
        }
        "/other/long/static/path/" => {
            route: "/other/long/static/path/",
            data: 7
        }
    });

    Ok(())
}

#[test]
fn catchall_off_by_one() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/foo/{*catchall}", 1)?;
    router.insert("/bar", 2)?;
    router.insert("/bar/", 3)?;
    router.insert("/bar/{*catchall}", 4)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ bar ○
       │    ╰─ / ○
       │       ╰─ {*catchall} ○
       ╰─ foo/
             ╰─ {*catchall} ○
    "#);

    assert_router_matches!(router, {
        "/foo" => None
        "/foo/" => None
        "/foo/x" => {
            route: "/foo/{*catchall}",
            data: 1,
            params: {
                "catchall" => "x"
            }
        }
        "/bar" => {
            route: "/bar",
            data: 2
        }
        "/bar/" => {
            route: "/bar/",
            data: 3
        }
        "/bar/x" => {
            route: "/bar/{*catchall}",
            data: 4,
            params: {
                "catchall" => "x"
            }
        }
    });

    Ok(())
}

#[test]
fn overlap() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/foo", 1)?;
    router.insert("/bar", 2)?;
    router.insert("/{*bar}", 3)?;
    router.insert("/baz", 4)?;
    router.insert("/baz/", 5)?;
    router.insert("/baz/x", 6)?;
    router.insert("/baz/{xxx}", 7)?;
    router.insert("/", 8)?;
    router.insert("/xxx/{*x}", 9)?;
    router.insert("/xxx/", 10)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ / ○
       ├─ ba
       │   ├─ r ○
       │   ╰─ z ○
       │      ╰─ / ○
       │         ├─ x ○
       │         ╰─ {xxx} ○
       ├─ foo ○
       ├─ xxx/ ○
       │     ╰─ {*x} ○
       ╰─ {*bar} ○
    "#);

    assert_router_matches!(router, {
        "/foo" => {
            route: "/foo",
            data: 1
        }
        "/bar" => {
            route: "/bar",
            data: 2
        }
        "/baz" => {
            route: "/baz",
            data: 4
        }
        "/baz/" => {
            route: "/baz/",
            data: 5
        }
        "/baz/x" => {
            route: "/baz/x",
            data: 6
        }
        "/???" => {
            route: "/{*bar}",
            data: 3,
            params: {
                "bar" => "???"
            }
        }
        "/" => {
            route: "/",
            data: 8
        }
        "" => None
        "/xxx/y" => {
            route: "/xxx/{*x}",
            data: 9,
            params: {
                "x" => "y"
            }
        }
        "/xxx/" => {
            route: "/xxx/",
            data: 10
        }
        "/xxx" => {
            route: "/{*bar}",
            data: 3,
            params: {
                "bar" => "xxx"
            }
        }
    });

    Ok(())
}

#[test]
fn missing_trailing_slash_param() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/foo/{object}/{id}", 1)?;
    router.insert("/foo/bar/baz", 2)?;
    router.insert("/foo/secret/978/", 3)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /foo/
           ├─ bar/baz ○
           ├─ secret/978/ ○
           ╰─ {object}
                     ╰─ /
                        ╰─ {id} ○
    "#);

    assert_router_matches!(router, {
        "/foo/secret/978/" => {
            route: "/foo/secret/978/",
            data: 3
        }
        "/foo/secret/978" => {
            route: "/foo/{object}/{id}",
            data: 1,
            params: {
                "object" => "secret",
                "id" => "978"
            }
        }
    });

    Ok(())
}

#[test]
fn extra_trailing_slash_param() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/foo/{object}/{id}", 1)?;
    router.insert("/foo/bar/baz", 2)?;
    router.insert("/foo/secret/978", 3)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /foo/
           ├─ bar/baz ○
           ├─ secret/978 ○
           ╰─ {object}
                     ╰─ /
                        ╰─ {id} ○
    "#);

    assert_router_matches!(router, {
        "/foo/secret/978/" => None
        "/foo/secret/978" => {
            route: "/foo/secret/978",
            data: 3
        }
    });

    Ok(())
}

#[test]
fn missing_trailing_slash_catch_all() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/foo/{*bar}", 1)?;
    router.insert("/foo/bar/baz", 2)?;
    router.insert("/foo/secret/978/", 3)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /foo/
           ├─ bar/baz ○
           ├─ secret/978/ ○
           ╰─ {*bar} ○
    "#);

    assert_router_matches!(router, {
        "/foo/secret/978" => {
            route: "/foo/{*bar}",
            data: 1,
            params: {
                "bar" => "secret/978"
            }
        }
        "/foo/secret/978/" => {
            route: "/foo/secret/978/",
            data: 3
        }
    });

    Ok(())
}

#[test]
fn extra_trailing_slash_catch_all() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/foo/{*bar}", 1)?;
    router.insert("/foo/bar/baz", 2)?;
    router.insert("/foo/secret/978", 3)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /foo/
           ├─ bar/baz ○
           ├─ secret/978 ○
           ╰─ {*bar} ○
    "#);

    assert_router_matches!(router, {
        "/foo/secret/978/" => {
            route: "/foo/{*bar}",
            data: 1,
            params: {
                "bar" => "secret/978/"
            }
        }
        "/foo/secret/978" => {
            route: "/foo/secret/978",
            data: 3
        }
    });

    Ok(())
}

#[test]
fn double_overlap_trailing_slash() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/{object}/{id}", 1)?;
    router.insert("/secret/{id}/path", 2)?;
    router.insert("/secret/978/", 3)?;
    router.insert("/other/{object}/{id}/", 4)?;
    router.insert("/other/an_object/{id}", 5)?;
    router.insert("/other/static/path", 6)?;
    router.insert("/other/long/static/path/", 7)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ other/
       │       ├─ an_object/
       │       │           ╰─ {id} ○
       │       ├─ long/static/path/ ○
       │       ├─ static/path ○
       │       ╰─ {object}
       │                 ╰─ /
       │                    ╰─ {id}
       │                          ╰─ / ○
       ├─ secret/
       │        ├─ 978/ ○
       │        ╰─ {id}
       │              ╰─ /path ○
       ╰─ {object}
                 ╰─ /
                    ╰─ {id} ○
    "#);

    assert_router_matches!(router, {
        "/secret/978/path/" => None
        "/object/id/" => None
        "/object/id/path" => None
        "/other/object/1" => None
        "/other/object/1/2" => None
        "/other/an_object/1/" => {
            route: "/other/{object}/{id}/",
            data: 4,
            params: {
                "object" => "an_object",
                "id" => "1"
            }
        }
        "/other/static/path/" => {
            route: "/other/{object}/{id}/",
            data: 4,
            params: {
                "object" => "static",
                "id" => "path"
            }
        }
        "/other/long/static/path" => None
        "/other/object/static/path" => None
    });

    Ok(())
}

#[test]
fn trailing_slash_overlap() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/foo/{x}/baz/", 1)?;
    router.insert("/foo/{x}/baz", 2)?;
    router.insert("/foo/bar/bar", 3)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /foo/
           ├─ bar/bar ○
           ╰─ {x}
                ╰─ /baz ○
                      ╰─ / ○
    "#);

    assert_router_matches!(router, {
        "/foo/x/baz/" => {
            route: "/foo/{x}/baz/",
            data: 1,
            params: {
                "x" => "x"
            }
        }
        "/foo/x/baz" => {
            route: "/foo/{x}/baz",
            data: 2,
            params: {
                "x" => "x"
            }
        }
        "/foo/bar/bar" => {
            route: "/foo/bar/bar",
            data: 3
        }
    });

    Ok(())
}

#[test]
fn trailing_slash() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/hi", 1)?;
    router.insert("/b/", 2)?;
    router.insert("/search/{query}", 3)?;
    router.insert("/cmd/{tool}/", 4)?;
    router.insert("/src/{*filepath}", 5)?;
    router.insert("/x", 6)?;
    router.insert("/x/y", 7)?;
    router.insert("/y/", 8)?;
    router.insert("/y/z", 9)?;
    router.insert("/0/{id}", 10)?;
    router.insert("/0/{id}/1", 11)?;
    router.insert("/1/{id}/", 12)?;
    router.insert("/1/{id}/2", 13)?;
    router.insert("/aa", 14)?;
    router.insert("/a/", 15)?;
    router.insert("/admin", 16)?;
    router.insert("/admin/static", 17)?;
    router.insert("/admin/{category}", 18)?;
    router.insert("/admin/{category}/{page}", 19)?;
    router.insert("/doc", 20)?;
    router.insert("/doc/rust_faq.html", 21)?;
    router.insert("/doc/rust1.26.html", 22)?;
    router.insert("/no/a", 23)?;
    router.insert("/no/b", 24)?;
    router.insert("/no/a/b/{*other}", 25)?;
    router.insert("/api/{page}/{name}", 26)?;
    router.insert("/api/hello/{name}/bar/", 27)?;
    router.insert("/api/bar/{name}", 28)?;
    router.insert("/api/baz/foo", 29)?;
    router.insert("/api/baz/foo/bar", 30)?;
    router.insert("/foo/{p}", 31)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ 0/
       │   ╰─ {id} ○
       │         ╰─ /1 ○
       ├─ 1/
       │   ╰─ {id}
       │         ╰─ / ○
       │            ╰─ 2 ○
       ├─ a
       │  ├─ / ○
       │  ├─ a ○
       │  ├─ dmin ○
       │  │     ╰─ /
       │  │        ├─ static ○
       │  │        ╰─ {category} ○
       │  │                    ╰─ /
       │  │                       ╰─ {page} ○
       │  ╰─ pi/
       │       ├─ ba
       │       │   ├─ r/
       │       │   │   ╰─ {name} ○
       │       │   ╰─ z/foo ○
       │       │          ╰─ /bar ○
       │       ├─ hello/
       │       │       ╰─ {name}
       │       │               ╰─ /bar/ ○
       │       ╰─ {page}
       │               ╰─ /
       │                  ╰─ {name} ○
       ├─ b/ ○
       ├─ cmd/
       │     ╰─ {tool}
       │             ╰─ / ○
       ├─ doc ○
       │    ╰─ /rust
       │           ├─ 1.26.html ○
       │           ╰─ _faq.html ○
       ├─ foo/
       │     ╰─ {p} ○
       ├─ hi ○
       ├─ no/
       │    ├─ a ○
       │    │  ╰─ /b/
       │    │       ╰─ {*other} ○
       │    ╰─ b ○
       ├─ s
       │  ├─ earch/
       │  │       ╰─ {query} ○
       │  ╰─ rc/
       │       ╰─ {*filepath} ○
       ├─ x ○
       │  ╰─ /y ○
       ╰─ y/ ○
           ╰─ z ○
    "#);

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

    Ok(())
}

#[test]
fn backtracking_trailing_slash() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/a/{b}/{c}", 1)?;
    router.insert("/a/b/{c}/d/", 2)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /a/
         ├─ b/
         │   ╰─ {c}
         │        ╰─ /d/ ○
         ╰─ {b}
              ╰─ /
                 ╰─ {c} ○
    "#);

    assert_router_matches!(router, {
        "/a/b/c/d" => None
    });

    Ok(())
}

#[test]
fn root_trailing_slash() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/foo", 1)?;
    router.insert("/bar", 2)?;
    router.insert("/{baz}", 3)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /
       ├─ bar ○
       ├─ foo ○
       ╰─ {baz} ○
    "#);

    assert_router_matches!(router, {
        "/" => None
    });

    Ok(())
}

#[test]
fn catchall_overlap() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/yyy/{*x}", 1)?;
    router.insert("/yyy{*x}", 2)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ /yyy
          ├─ /
          │  ╰─ {*x} ○
          ╰─ {*x} ○
    "#);

    assert_router_matches!(router, {
        "/yyy/y" => {
            route: "/yyy/{*x}",
            data: 1,
            params: {
                "x" => "y"
            }
        }
        "/yyy/" => {
            route: "/yyy{*x}",
            data: 2,
            params: {
                "x" => "/"
            }
        }
    });

    Ok(())
}

#[test]
fn escaped() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/", 1)?;
    router.insert("/{{", 2)?;
    router.insert("/}}", 3)?;
    router.insert("/{{x", 4)?;
    router.insert("/}}y{{", 5)?;
    router.insert("/xy{{", 6)?;
    router.insert("/{{/xyz", 7)?;
    // router.insert("/{ba{{r}", 8);
    // router.insert("/{ba{{r}/", 9)?;
    // router.insert("/{ba{{r}/x", 10)?;
    router.insert("/baz/{xxx}", 11)?;
    router.insert("/baz/{xxx}/xy{{", 12)?;
    router.insert("/baz/{xxx}/}}xy{{{{", 13)?;
    router.insert("/{{/{x}", 14)?;
    router.insert("/xxx/", 15)?;
    // router.insert("/xxx/{x}}{{}}}}{{}}{{{{}}y}", 16)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ / ○
       ├─ baz/
       │     ╰─ {xxx} ○
       │            ╰─ /
       │               ├─ xy{ ○
       │               ╰─ }xy{{ ○
       ├─ x
       │  ├─ xx/ ○
       │  ╰─ y{ ○
       ├─ { ○
       │  ├─ /
       │  │  ├─ xyz ○
       │  │  ╰─ {x} ○
       │  ╰─ x ○
       ╰─ } ○
          ╰─ y{ ○
    "#);

    assert_router_matches!(router, {
        "/" => {
            route: "/",
            data: 1
        }
        "/{" => {
            route: "/{{",
            data: 2
        }
        "/}" => {
            route: "/}}",
            data: 3
        }
        "/{x" => {
            route: "/{{x",
            data: 4
        }
        "/}y{" => {
            route: "/}}y{{",
            data: 5
        }
        "/xy{" => {
            route: "/xy{{",
            data: 6
        }
        "/{/xyz" => {
            route: "/{{/xyz",
            data: 7
        }
        // "/foo" => {
        //     route: "/{ba{{r}",
        //     data: 8,
        //     params: {
        //         "ba{r" => "foo"
        //     }
        // }
        // "/{{" => {
        //     route: "/{ba{{r}",
        //     data: 8,
        //     params: {
        //         "ba{r" => "{{"
        //     }
        // }
        // "/{{}}/" => {
        //     route: "/{ba{{r}/",
        //     data: 9,
        //     params: {
        //         "ba{r" => "{{}}"
        //     }
        // }
        // "/{{}}{{/x" => {
        //     route: "/{ba{{r}/x",
        //     data: 10,
        //     params: {
        //         "ba{r" => "{{}}{{"
        //     }
        // }
        "/baz/x" => {
            route: "/baz/{xxx}",
            data: 11,
            params: {
                "xxx" => "x"
            }
        }
        "/baz/x/xy{" => {
            route: "/baz/{xxx}/xy{{",
            data: 12,
            params: {
                "xxx" => "x"
            }
        }
        "/baz/x/xy{{" => None
        "/baz/x/}xy{{" => {
            route: "/baz/{xxx}/}}xy{{{{",
            data: 13,
            params: {
                "xxx" => "x"
            }
        }
        "/{/{{" => {
            route: "/{{/{x}",
            data: 14,
            params: {
                "x" => "{{"
            }
        }
        // "/xxx" => {
        //     route: "/{ba{{r}",
        //     data: 8,
        //     params: {
        //         "ba{r" => "xxx"
        //     }
        // }
        "/xxx/" => {
            route: "/xxx/",
            data: 15
        }
        // "/xxx/foo" => {
        //     route: "/xxx/{x}{{}}}}{{}}{{{{}}y}",
        //     data: 16,
        //     params: {
        //         "x}{}}{}{{}y" => "foo"
        //     }
        // }
    });

    Ok(())
}

#[test]
fn basic() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/hi", 1)?;
    router.insert("/contact", 2)?;
    router.insert("/co", 3)?;
    router.insert("/c", 4)?;
    router.insert("/a", 5)?;
    router.insert("/ab", 6)?;
    router.insert("/doc/", 7)?;
    router.insert("/doc/rust_faq.html", 8)?;
    router.insert("/doc/rust1.26.html", 9)?;
    router.insert("ʯ", 10)?;
    router.insert("β", 11)?;
    router.insert("/sd!here", 12)?;
    router.insert("/sd$here", 13)?;
    router.insert("/sd&here", 14)?;
    router.insert("/sd'here", 15)?;
    router.insert("/sd(here", 16)?;
    router.insert("/sd)here", 17)?;
    router.insert("/sd+here", 18)?;
    router.insert("/sd,here", 19)?;
    router.insert("/sd;here", 20)?;
    router.insert("/sd=here", 21)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ├─ /
    │  ├─ a ○
    │  │  ╰─ b ○
    │  ├─ c ○
    │  │  ╰─ o ○
    │  │     ╰─ ntact ○
    │  ├─ doc/ ○
    │  │     ╰─ rust
    │  │           ├─ 1.26.html ○
    │  │           ╰─ _faq.html ○
    │  ├─ hi ○
    │  ╰─ sd
    │      ├─ !here ○
    │      ├─ $here ○
    │      ├─ &here ○
    │      ├─ 'here ○
    │      ├─ (here ○
    │      ├─ )here ○
    │      ├─ +here ○
    │      ├─ ,here ○
    │      ├─ ;here ○
    │      ╰─ =here ○
    ├─ ʯ ○
    ╰─ β ○
    "#);

    assert_router_matches!(router, {
        "/a" => {
            route: "/a",
            data: 5
        }
        "/" => None
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
        "/con" => None
        "/cona" => None
        "/no" => None
        "/ab" => {
            route: "/ab",
            data: 6
        }
        "ʯ" => {
            route: "ʯ",
            data: 10
        }
        "β" => {
            route: "β",
            data: 11
        }
        "/sd!here" => {
            route: "/sd!here",
            data: 12
        }
        "/sd$here" => {
            route: "/sd$here",
            data: 13
        }
        "/sd&here" => {
            route: "/sd&here",
            data: 14
        }
        "/sd'here" => {
            route: "/sd'here",
            data: 15
        }
        "/sd(here" => {
            route: "/sd(here",
            data: 16
        }
        "/sd)here" => {
            route: "/sd)here",
            data: 17
        }
        "/sd+here" => {
            route: "/sd+here",
            data: 18
        }
        "/sd,here" => {
            route: "/sd,here",
            data: 19
        }
        "/sd;here" => {
            route: "/sd;here",
            data: 20
        }
        "/sd=here" => {
            route: "/sd=here",
            data: 21
        }
    });

    Ok(())
}

#[test]
fn wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/", 1)?;
    router.insert("/cmd/{tool}/", 2)?;
    router.insert("/cmd/{tool2}/{sub}", 3)?;
    router.insert("/cmd/whoami", 4)?;
    router.insert("/cmd/whoami/root", 5)?;
    router.insert("/cmd/whoami/root/", 6)?;
    router.insert("/src", 7)?;
    router.insert("/src/", 8)?;
    router.insert("/src/{*filepath}", 9)?;
    router.insert("/search/", 10)?;
    router.insert("/search/{query}", 11)?;
    router.insert("/search/actix-we", 12)?;
    router.insert("/search/google", 13)?;
    router.insert("/user_{name}", 14)?;
    router.insert("/user_{name}/about", 15)?;
    router.insert("/files/{dir}/{*filepath}", 16)?;
    router.insert("/doc/", 17)?;
    router.insert("/doc/rust_faq.html", 18)?;
    router.insert("/doc/rust1.26.html", 19)?;
    router.insert("/info/{user}/public", 20)?;
    router.insert("/info/{user}/project/{project}", 21)?;
    router.insert("/info/{user}/project/rustlang", 22)?;
    router.insert("/aa/{*xx}", 23)?;
    router.insert("/ab/{*xx}", 24)?;
    router.insert("/ab/hello{*xx}", 25)?;
    router.insert("/{cc}", 26)?;
    router.insert("/c1/{dd}/e", 27)?;
    router.insert("/c1/{dd}/e1", 28)?;
    router.insert("/{cc}/cc", 29)?;
    router.insert("/{cc}/{dd}/ee", 30)?;
    router.insert("/{cc}/{dd}/{ee}/ff", 31)?;
    router.insert("/{cc}/{dd}/{ee}/{ff}/gg", 32)?;
    router.insert("/{cc}/{dd}/{ee}/{ff}/{gg}/hh", 33)?;
    router.insert("/get/test/abc/", 34)?;
    router.insert("/get/{param}/abc/", 35)?;
    router.insert("/something/{paramname}/thirdthing", 36)?;
    router.insert("/something/secondthing/test", 37)?;
    router.insert("/get/abc", 38)?;
    router.insert("/get/{param}", 39)?;
    router.insert("/get/abc/123abc", 40)?;
    router.insert("/get/abc/{param}", 41)?;
    router.insert("/get/abc/123abc/xxx8", 42)?;
    router.insert("/get/abc/123abc/{param}", 43)?;
    router.insert("/get/abc/123abc/xxx8/1234", 44)?;
    router.insert("/get/abc/123abc/xxx8/{param}", 45)?;
    router.insert("/get/abc/123abc/xxx8/1234/ffas", 46)?;
    router.insert("/get/abc/123abc/xxx8/1234/{param}", 47)?;
    router.insert("/get/abc/123abc/xxx8/1234/kkdd/12c", 48)?;
    router.insert("/get/abc/123abc/xxx8/1234/kkdd/{param}", 49)?;
    router.insert("/get/abc/{param}/test", 50)?;
    router.insert("/get/abc/123abd/{param}", 51)?;
    router.insert("/get/abc/123abddd/{param}", 52)?;
    router.insert("/get/abc/123/{param}", 53)?;
    router.insert("/get/abc/123abg/{param}", 54)?;
    router.insert("/get/abc/123abf/{param}", 55)?;
    router.insert("/get/abc/123abfff/{param}", 56)?;

    insta::assert_snapshot!(router, @r#"
    ▽
    ╰─ / ○
       ├─ a
       │  ├─ a/
       │  │   ╰─ {*xx} ○
       │  ╰─ b/
       │      ├─ hello
       │      │      ╰─ {*xx} ○
       │      ╰─ {*xx} ○
       ├─ c
       │  ├─ 1/
       │  │   ╰─ {dd}
       │  │         ╰─ /e ○
       │  │             ╰─ 1 ○
       │  ╰─ md/
       │       ├─ whoami ○
       │       │       ╰─ /root ○
       │       │              ╰─ / ○
       │       ├─ {tool}
       │       │       ╰─ / ○
       │       ╰─ {tool2}
       │                ╰─ /
       │                   ╰─ {sub} ○
       ├─ doc/ ○
       │     ╰─ rust
       │           ├─ 1.26.html ○
       │           ╰─ _faq.html ○
       ├─ files/
       │       ╰─ {dir}
       │              ╰─ /
       │                 ╰─ {*filepath} ○
       ├─ get/
       │     ├─ abc ○
       │     │    ╰─ /
       │     │       ├─ 123
       │     │       │    ├─ /
       │     │       │    │  ╰─ {param} ○
       │     │       │    ╰─ ab
       │     │       │        ├─ c ○
       │     │       │        │  ╰─ /
       │     │       │        │     ├─ xxx8 ○
       │     │       │        │     │     ╰─ /
       │     │       │        │     │        ├─ 1234 ○
       │     │       │        │     │        │     ╰─ /
       │     │       │        │     │        │        ├─ ffas ○
       │     │       │        │     │        │        ├─ kkdd/
       │     │       │        │     │        │        │      ├─ 12c ○
       │     │       │        │     │        │        │      ╰─ {param} ○
       │     │       │        │     │        │        ╰─ {param} ○
       │     │       │        │     │        ╰─ {param} ○
       │     │       │        │     ╰─ {param} ○
       │     │       │        ├─ d
       │     │       │        │  ├─ /
       │     │       │        │  │  ╰─ {param} ○
       │     │       │        │  ╰─ dd/
       │     │       │        │       ╰─ {param} ○
       │     │       │        ├─ f
       │     │       │        │  ├─ /
       │     │       │        │  │  ╰─ {param} ○
       │     │       │        │  ╰─ ff/
       │     │       │        │       ╰─ {param} ○
       │     │       │        ╰─ g/
       │     │       │            ╰─ {param} ○
       │     │       ╰─ {param} ○
       │     │                ╰─ /test ○
       │     ├─ test/abc/ ○
       │     ╰─ {param} ○
       │              ╰─ /abc/ ○
       ├─ info/
       │      ╰─ {user}
       │              ╰─ /p
       │                  ├─ roject/
       │                  │        ├─ rustlang ○
       │                  │        ╰─ {project} ○
       │                  ╰─ ublic ○
       ├─ s
       │  ├─ earch/ ○
       │  │       ├─ actix-we ○
       │  │       ├─ google ○
       │  │       ╰─ {query} ○
       │  ├─ omething/
       │  │          ├─ secondthing/test ○
       │  │          ╰─ {paramname}
       │  │                       ╰─ /thirdthing ○
       │  ╰─ rc ○
       │      ╰─ / ○
       │         ╰─ {*filepath} ○
       ├─ user_
       │      ╰─ {name} ○
       │              ╰─ /about ○
       ╰─ {cc} ○
             ╰─ /
                ├─ cc ○
                ╰─ {dd}
                      ╰─ /
                         ├─ ee ○
                         ╰─ {ee}
                               ╰─ /
                                  ├─ ff ○
                                  ╰─ {ff}
                                        ╰─ /
                                           ├─ gg ○
                                           ╰─ {gg}
                                                 ╰─ /hh ○
    "#);

    assert_router_matches!(router, {
        "/" => {
            route: "/",
            data: 1
        }
        "/cmd/test" => None
        "/cmd/test/" => {
            route: "/cmd/{tool}/",
            data: 2,
            params: {
                "tool" => "test"
            }
        }
        "/cmd/test/3" => {
            route: "/cmd/{tool2}/{sub}",
            data: 3,
            params: {
                "tool2" => "test",
                "sub" => "3"
            }
        }
        "/cmd/who" => None
        "/cmd/who/" => {
            route: "/cmd/{tool}/",
            data: 2,
            params: {
                "tool" => "who"
            }
        }
        "/cmd/whoami" => {
            route: "/cmd/whoami",
            data: 4
        }
        "/cmd/whoami/" => {
            route: "/cmd/{tool}/",
            data: 2,
            params: {
                "tool" => "whoami"
            }
        }
        "/cmd/whoami/r" => {
            route: "/cmd/{tool2}/{sub}",
            data: 3,
            params: {
                "tool2" => "whoami",
                "sub" => "r"
            }
        }
        "/cmd/whoami/r/" => None
        "/cmd/whoami/root" => {
            route: "/cmd/whoami/root",
            data: 5
        }
        "/cmd/whoami/root/" => {
            route: "/cmd/whoami/root/",
            data: 6
        }
        "/src" => {
            route: "/src",
            data: 7
        }
        "/src/" => {
            route: "/src/",
            data: 8
        }
        "/src/some/file.png" => {
            route: "/src/{*filepath}",
            data: 9,
            params: {
                "filepath" => "some/file.png"
            }
        }
        "/search/" => {
            route: "/search/",
            data: 10
        }
        "/search/actix" => {
            route: "/search/{query}",
            data: 11,
            params: {
                "query" => "actix"
            }
        }
        "/search/actix-we" => {
            route: "/search/actix-we",
            data: 12
        }
        "/search/someth!ng+in+ünìcodé" => {
            route: "/search/{query}",
            data: 11,
            params: {
                "query" => "someth!ng+in+ünìcodé"
            }
        }
        "/search/someth!ng+in+ünìcodé/" => None
        "/user_rustacean" => {
            route: "/user_{name}",
            data: 14,
            params: {
                "name" => "rustacean"
            }
        }
        "/user_rustacean/about" => {
            route: "/user_{name}/about",
            data: 15,
            params: {
                "name" => "rustacean"
            }
        }
        "/files/js/inc/framework.js" => {
            route: "/files/{dir}/{*filepath}",
            data: 16,
            params: {
                "dir" => "js",
                "filepath" => "inc/framework.js"
            }
        }
        "/info/gordon/public" => {
            route: "/info/{user}/public",
            data: 20,
            params: {
                "user" => "gordon"
            }
        }
        "/info/gordon/project/rust" => {
            route: "/info/{user}/project/{project}",
            data: 21,
            params: {
                "user" => "gordon",
                "project" => "rust"
            }
        }
        "/info/gordon/project/rustlang" => {
            route: "/info/{user}/project/rustlang",
            data: 22,
            params: {
                "user" => "gordon"
            }
        }
        "/aa/" => None
        "/aa/aa" => {
            route: "/aa/{*xx}",
            data: 23,
            params: {
                "xx" => "aa"
            }
        }
        "/ab/ab" => {
            route: "/ab/{*xx}",
            data: 24,
            params: {
                "xx" => "ab"
            }
        }
        "/ab/hello-world" => {
            route: "/ab/hello{*xx}",
            data: 25,
            params: {
                "xx" => "-world"
            }
        }
        "/a" => {
            route: "/{cc}",
            data: 26,
            params: {
                "cc" => "a"
            }
        }
        "/all" => {
            route: "/{cc}",
            data: 26,
            params: {
                "cc" => "all"
            }
        }
        "/d" => {
            route: "/{cc}",
            data: 26,
            params: {
                "cc" => "d"
            }
        }
        "/ad" => {
            route: "/{cc}",
            data: 26,
            params: {
                "cc" => "ad"
            }
        }
        "/dd" => {
            route: "/{cc}",
            data: 26,
            params: {
                "cc" => "dd"
            }
        }
        "/dddaa" => {
            route: "/{cc}",
            data: 26,
            params: {
                "cc" => "dddaa"
            }
        }
        "/aa" => {
            route: "/{cc}",
            data: 26,
            params: {
                "cc" => "aa"
            }
        }
        "/aaa" => {
            route: "/{cc}",
            data: 26,
            params: {
                "cc" => "aaa"
            }
        }
        "/aaa/cc" => {
            route: "/{cc}/cc",
            data: 29,
            params: {
                "cc" => "aaa"
            }
        }
        "/a" => {
            route: "/{cc}",
            data: 26,
            params: {
                "cc" => "a"
            }
        }
        "/ab" => {
            route: "/{cc}",
            data: 26,
            params: {
                "cc" => "ab"
            }
        }
        "/abb/cc" => {
            route: "/{cc}/cc",
            data: 29,
            params: {
                "cc" => "abb"
            }
        }
        "/allxxxx" => {
            route: "/{cc}",
            data: 26,
            params: {
                "cc" => "allxxxx"
            }
        }
        "/alldd" => {
            route: "/{cc}",
            data: 26,
            params: {
                "cc" => "alldd"
            }
        }
        "/all/cc" => {
            route: "/{cc}/cc",
            data: 29,
            params: {
                "cc" => "all"
            }
        }
        "/a/cc" => {
            route: "/{cc}/cc",
            data: 29,
            params: {
                "cc" => "a"
            }
        }
        "/c1/d/e" => {
            route: "/c1/{dd}/e",
            data: 27,
            params: {
                "dd" => "d"
            }
        }
        "/c1/d/e1" => {
            route: "/c1/{dd}/e1",
            data: 28,
            params: {
                "dd" => "d"
            }
        }
        "/c1/d/ee" => {
            route: "/{cc}/{dd}/ee",
            data: 30,
            params: {
                "cc" => "c1",
                "dd" => "d"
            }
        }
        "/cc/cc" => {
            route: "/{cc}/cc",
            data: 29,
            params: {
                "cc" => "cc"
            }
        }
        "/ccc/cc" => {
            route: "/{cc}/cc",
            data: 29,
            params: {
                "cc" => "ccc"
            }
        }
        "/deedwjfs/cc" => {
            route: "/{cc}/cc",
            data: 29,
            params: {
                "cc" => "deedwjfs"
            }
        }
        "/acllcc/cc" => {
            route: "/{cc}/cc",
            data: 29,
            params: {
                "cc" => "acllcc"
            }
        }
        "/get/test/abc/" => {
            route: "/get/test/abc/",
            data: 34
        }
        "/get/te/abc/" => {
            route: "/get/{param}/abc/",
            data: 35,
            params: {
                "param" => "te"
            }
        }
        "/get/testaa/abc/" => {
            route: "/get/{param}/abc/",
            data: 35,
            params: {
                "param" => "testaa"
            }
        }
        "/get/xx/abc/" => {
            route: "/get/{param}/abc/",
            data: 35,
            params: {
                "param" => "xx"
            }
        }
        "/get/tt/abc/" => {
            route: "/get/{param}/abc/",
            data: 35,
            params: {
                "param" => "tt"
            }
        }
        "/get/a/abc/" => {
            route: "/get/{param}/abc/",
            data: 35,
            params: {
                "param" => "a"
            }
        }
        "/get/t/abc/" => {
            route: "/get/{param}/abc/",
            data: 35,
            params: {
                "param" => "t"
            }
        }
        "/get/aa/abc/" => {
            route: "/get/{param}/abc/",
            data: 35,
            params: {
                "param" => "aa"
            }
        }
        "/get/abas/abc/" => {
            route: "/get/{param}/abc/",
            data: 35,
            params: {
                "param" => "abas"
            }
        }
        "/something/secondthing/test" => {
            route: "/something/secondthing/test",
            data: 37
        }
        "/something/abcdad/thirdthing" => {
            route: "/something/{paramname}/thirdthing",
            data: 36,
            params: {
                "paramname" => "abcdad"
            }
        }
        "/something/secondthingaaaa/thirdthing" => {
            route: "/something/{paramname}/thirdthing",
            data: 36,
            params: {
                "paramname" => "secondthingaaaa"
            }
        }
        "/something/se/thirdthing" => {
            route: "/something/{paramname}/thirdthing",
            data: 36,
            params: {
                "paramname" => "se"
            }
        }
        "/something/s/thirdthing" => {
            route: "/something/{paramname}/thirdthing",
            data: 36,
            params: {
                "paramname" => "s"
            }
        }
        "/c/d/ee" => {
            route: "/{cc}/{dd}/ee",
            data: 30,
            params: {
                "cc" => "c",
                "dd" => "d"
            }
        }
        "/c/d/e/ff" => {
            route: "/{cc}/{dd}/{ee}/ff",
            data: 31,
            params: {
                "cc" => "c",
                "dd" => "d",
                "ee" => "e"
            }
        }
        "/c/d/e/f/gg" => {
            route: "/{cc}/{dd}/{ee}/{ff}/gg",
            data: 32,
            params: {
                "cc" => "c",
                "dd" => "d",
                "ee" => "e",
                "ff" => "f"
            }
        }
        "/c/d/e/f/g/hh" => {
            route: "/{cc}/{dd}/{ee}/{ff}/{gg}/hh",
            data: 33,
            params: {
                "cc" => "c",
                "dd" => "d",
                "ee" => "e",
                "ff" => "f",
                "gg" => "g"
            }
        }
        "/cc/dd/ee/ff/gg/hh" => {
            route: "/{cc}/{dd}/{ee}/{ff}/{gg}/hh",
            data: 33,
            params: {
                "cc" => "cc",
                "dd" => "dd",
                "ee" => "ee",
                "ff" => "ff",
                "gg" => "gg"
            }
        }
        "/get/abc" => {
            route: "/get/abc",
            data: 38
        }
        "/get/a" => {
            route: "/get/{param}",
            data: 39,
            params: {
                "param" => "a"
            }
        }
        "/get/abz" => {
            route: "/get/{param}",
            data: 39,
            params: {
                "param" => "abz"
            }
        }
        "/get/12a" => {
            route: "/get/{param}",
            data: 39,
            params: {
                "param" => "12a"
            }
        }
        "/get/abcd" => {
            route: "/get/{param}",
            data: 39,
            params: {
                "param" => "abcd"
            }
        }
        "/get/abc/123abc" => {
            route: "/get/abc/123abc",
            data: 40
        }
        "/get/abc/12" => {
            route: "/get/abc/{param}",
            data: 41,
            params: {
                "param" => "12"
            }
        }
        "/get/abc/123a" => {
            route: "/get/abc/{param}",
            data: 41,
            params: {
                "param" => "123a"
            }
        }
        "/get/abc/xyz" => {
            route: "/get/abc/{param}",
            data: 41,
            params: {
                "param" => "xyz"
            }
        }
        "/get/abc/123abcddxx" => {
            route: "/get/abc/{param}",
            data: 41,
            params: {
                "param" => "123abcddxx"
            }
        }
        "/get/abc/123abc/xxx8" => {
            route: "/get/abc/123abc/xxx8",
            data: 42
        }
        "/get/abc/123abc/x" => {
            route: "/get/abc/123abc/{param}",
            data: 43,
            params: {
                "param" => "x"
            }
        }
        "/get/abc/123abc/xxx" => {
            route: "/get/abc/123abc/{param}",
            data: 43,
            params: {
                "param" => "xxx"
            }
        }
        "/get/abc/123abc/abc" => {
            route: "/get/abc/123abc/{param}",
            data: 43,
            params: {
                "param" => "abc"
            }
        }
        "/get/abc/123abc/xxx8xxas" => {
            route: "/get/abc/123abc/{param}",
            data: 43,
            params: {
                "param" => "xxx8xxas"
            }
        }
        "/get/abc/123abc/xxx8/1234" => {
            route: "/get/abc/123abc/xxx8/1234",
            data: 44
        }
        "/get/abc/123abc/xxx8/1" => {
            route: "/get/abc/123abc/xxx8/{param}",
            data: 45,
            params: {
                "param" => "1"
            }
        }
        "/get/abc/123abc/xxx8/123" => {
            route: "/get/abc/123abc/xxx8/{param}",
            data: 45,
            params: {
                "param" => "123"
            }
        }
        "/get/abc/123abc/xxx8/78k" => {
            route: "/get/abc/123abc/xxx8/{param}",
            data: 45,
            params: {
                "param" => "78k"
            }
        }
        "/get/abc/123abc/xxx8/1234xxxd" => {
            route: "/get/abc/123abc/xxx8/{param}",
            data: 45,
            params: {
                "param" => "1234xxxd"
            }
        }
        "/get/abc/123abc/xxx8/1234/ffas" => {
            route: "/get/abc/123abc/xxx8/1234/ffas",
            data: 46
        }
        "/get/abc/123abc/xxx8/1234/f" => {
            route: "/get/abc/123abc/xxx8/1234/{param}",
            data: 47,
            params: {
                "param" => "f"
            }
        }
        "/get/abc/123abc/xxx8/1234/ffa" => {
            route: "/get/abc/123abc/xxx8/1234/{param}",
            data: 47,
            params: {
                "param" => "ffa"
            }
        }
        "/get/abc/123abc/xxx8/1234/kka" => {
            route: "/get/abc/123abc/xxx8/1234/{param}",
            data: 47,
            params: {
                "param" => "kka"
            }
        }
        "/get/abc/123abc/xxx8/1234/ffas321" => {
            route: "/get/abc/123abc/xxx8/1234/{param}",
            data: 47,
            params: {
                "param" => "ffas321"
            }
        }
        "/get/abc/123abc/xxx8/1234/kkdd/12c" => {
            route: "/get/abc/123abc/xxx8/1234/kkdd/12c",
            data: 48
        }
        "/get/abc/123abc/xxx8/1234/kkdd/1" => {
            route: "/get/abc/123abc/xxx8/1234/kkdd/{param}",
            data: 49,
            params: {
                "param" => "1"
            }
        }
        "/get/abc/123abc/xxx8/1234/kkdd/12" => {
            route: "/get/abc/123abc/xxx8/1234/kkdd/{param}",
            data: 49,
            params: {
                "param" => "12"
            }
        }
        "/get/abc/123abc/xxx8/1234/kkdd/34" => {
            route: "/get/abc/123abc/xxx8/1234/kkdd/{param}",
            data: 49,
            params: {
                "param" => "34"
            }
        }
        "/get/abc/123abc/xxx8/1234/kkdd/12c2e3" => {
            route: "/get/abc/123abc/xxx8/1234/kkdd/{param}",
            data: 49,
            params: {
                "param" => "12c2e3"
            }
        }
        "/get/abc/12/test" => {
            route: "/get/abc/{param}/test",
            data: 50,
            params: {
                "param" => "12"
            }
        }
        "/get/abc/123abdd/test" => {
            route: "/get/abc/{param}/test",
            data: 50,
            params: {
                "param" => "123abdd"
            }
        }
        "/get/abc/123abdddf/test" => {
            route: "/get/abc/{param}/test",
            data: 50,
            params: {
                "param" => "123abdddf"
            }
        }
        "/get/abc/123ab/test" => {
            route: "/get/abc/{param}/test",
            data: 50,
            params: {
                "param" => "123ab"
            }
        }
        "/get/abc/123abgg/test" => {
            route: "/get/abc/{param}/test",
            data: 50,
            params: {
                "param" => "123abgg"
            }
        }
        "/get/abc/123abff/test" => {
            route: "/get/abc/{param}/test",
            data: 50,
            params: {
                "param" => "123abff"
            }
        }
        "/get/abc/123abffff/test" => {
            route: "/get/abc/{param}/test",
            data: 50,
            params: {
                "param" => "123abffff"
            }
        }
        "/get/abc/123abd/test" => {
            route: "/get/abc/123abd/{param}",
            data: 51,
            params: {
                "param" => "test"
            }
        }
        "/get/abc/123abddd/test" => {
            route: "/get/abc/123abddd/{param}",
            data: 52,
            params: {
                "param" => "test"
            }
        }
        "/get/abc/123/test22" => {
            route: "/get/abc/123/{param}",
            data: 53,
            params: {
                "param" => "test22"
            }
        }
        "/get/abc/123abg/test" => {
            route: "/get/abc/123abg/{param}",
            data: 54,
            params: {
                "param" => "test"
            }
        }
        "/get/abc/123abf/testss" => {
            route: "/get/abc/123abf/{param}",
            data: 55,
            params: {
                "param" => "testss"
            }
        }
        "/get/abc/123abfff/te" => {
            route: "/get/abc/123abfff/{param}",
            data: 56,
            params: {
                "param" => "te"
            }
        }
    });

    Ok(())
}
