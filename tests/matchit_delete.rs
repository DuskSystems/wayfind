//! Tests sourced from `matchit` (MIT AND BSD-3-Clause)
//! <https://github.com/ibraheemdev/matchit/blob/v0.8.3/tests/remove.rs>

#![allow(clippy::too_many_lines, clippy::cognitive_complexity)]

use std::error::Error;
use wayfind::{errors::delete::DeleteError, router::Router};

#[test]
fn normalized() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/x/<foo>/bar", 0)?;
    router.insert("/x/<bar>/baz", 1)?;
    router.insert("/<foo>/<baz>/bax", 2)?;
    router.insert("/<foo>/<bar>/baz", 3)?;
    router.insert("/<fod>/<baz>/<bax>/foo", 4)?;
    router.insert("/<fod>/baz/bax/foo", 5)?;
    router.insert("/<foo>/baz/bax", 6)?;
    router.insert("/<bar>/<bay>/bay", 7)?;
    router.insert("/s", 8)?;
    router.insert("/s/s", 9)?;
    router.insert("/s/s/s", 10)?;
    router.insert("/s/s/s/s", 11)?;
    router.insert("/s/s/<s>/x", 12)?;
    router.insert("/s/s/<y>/d", 13)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/x/<foo>/bar"), Ok(()));
    assert_eq!(router.delete("/x/<bar>/baz"), Ok(()));
    assert_eq!(router.delete("/<foo>/<baz>/bax"), Ok(()));
    assert_eq!(router.delete("/<foo>/<bar>/baz"), Ok(()));
    assert_eq!(router.delete("/<fod>/<baz>/<bax>/foo"), Ok(()));
    assert_eq!(router.delete("/<fod>/baz/bax/foo"), Ok(()));
    assert_eq!(router.delete("/<foo>/baz/bax"), Ok(()));
    assert_eq!(router.delete("/<bar>/<bay>/bay"), Ok(()));
    assert_eq!(router.delete("/s"), Ok(()));
    assert_eq!(router.delete("/s/s"), Ok(()));
    assert_eq!(router.delete("/s/s/s"), Ok(()));
    assert_eq!(router.delete("/s/s/s/s"), Ok(()));
    assert_eq!(router.delete("/s/s/<s>/x"), Ok(()));
    assert_eq!(router.delete("/s/s/<y>/d"), Ok(()));

    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/home", 0)?;
    router.insert("/home/<id>", 1)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/home"), Ok(()));
    assert_eq!(router.delete("/home"), Err(DeleteError::NotFound));
    assert_eq!(router.delete("/home/<id>"), Ok(()));
    assert_eq!(router.delete("/home/<id>"), Err(DeleteError::NotFound));

    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn blog() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<page>", 0)?;
    router.insert("/posts/<year>/<month>/<post>", 1)?;
    router.insert("/posts/<year>/<month>/index", 2)?;
    router.insert("/posts/<year>/top", 3)?;
    router.insert("/static/<path:*>", 4)?;
    router.insert("/favicon.ico", 5)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/<page>"), Ok(()));
    assert_eq!(router.delete("/posts/<year>/<month>/<post>"), Ok(()));
    assert_eq!(router.delete("/posts/<year>/<month>/index"), Ok(()));
    assert_eq!(router.delete("/posts/<year>/top"), Ok(()));
    assert_eq!(router.delete("/static/<path:*>"), Ok(()));
    assert_eq!(router.delete("/favicon.ico"), Ok(()));

    insta::assert_snapshot!(router, @r###"
    $
    "###);

    Ok(())
}

#[test]
fn catchall() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/foo/<catchall:*>", 0)?;
    router.insert("/bar", 1)?;
    router.insert("/bar/", 2)?;
    router.insert("/bar/<catchall:*>", 3)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/foo/<catchall:*>"), Ok(()));
    assert_eq!(router.delete("/bar/"), Ok(()));

    insta::assert_snapshot!(router, @"");

    router.insert("/foo/<catchall:*>", 4)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/bar/<catchall:*>"), Ok(()));

    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn overlapping_routes() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/home", 0)?;
    router.insert("/home/<id>", 1)?;
    router.insert("/users", 2)?;
    router.insert("/users/<id>", 3)?;
    router.insert("/users/<id>/posts", 4)?;
    router.insert("/users/<id>/posts/<post_id>", 5)?;
    router.insert("/articles", 6)?;
    router.insert("/articles/<category>", 7)?;
    router.insert("/articles/<category>/<id>", 8)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/home"), Ok(()));

    insta::assert_snapshot!(router, @"");

    router.insert("/home", 9)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/home/<id>"), Ok(()));

    insta::assert_snapshot!(router, @"");

    router.insert("/home/<id>", 10)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/users"), Ok(()));

    insta::assert_snapshot!(router, @"");

    router.insert("/users", 11)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/users/<id>"), Ok(()));

    insta::assert_snapshot!(router, @"");

    router.insert("/users/<id>", 12)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/users/<id>/posts"), Ok(()));

    insta::assert_snapshot!(router, @"");

    router.insert("/users/<id>/posts", 13)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/users/<id>/posts/<post_id>"), Ok(()));

    insta::assert_snapshot!(router, @"");

    router.insert("/users/<id>/posts/<post_id>", 14)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/articles"), Ok(()));

    insta::assert_snapshot!(router, @"");

    router.insert("/articles", 15)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/articles/<category>"), Ok(()));

    insta::assert_snapshot!(router, @"");

    router.insert("/articles/<category>", 16)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/articles/<category>/<id>"), Ok(()));

    insta::assert_snapshot!(router, @"");

    router.insert("/articles/<category>/<id>", 17)?;

    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn trailing_slash() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<home>/", 0)?;
    router.insert("/foo", 1)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/"), Err(DeleteError::NotFound));

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/<home>"), Err(DeleteError::NotFound));

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/foo/"), Err(DeleteError::NotFound));

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/foo"), Ok(()));

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/<home>"), Err(DeleteError::NotFound));

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/<home>/"), Ok(()));

    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn remove_root() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/", 0)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/"), Ok(()));

    insta::assert_snapshot!(router, @"");

    Ok(())
}

#[test]
fn check_escaped_params() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/foo/<id>", 0)?;
    router.insert("/foo/<id>/bar", 1)?;
    router.insert("/bar/<user>/<id>", 2)?;
    router.insert("/bar/<user>/<id>/baz", 3)?;
    router.insert("/baz/<product>/<user>/<id>", 4)?;

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/foo/<a>"), Err(DeleteError::NotFound));

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/foo/<a>/bar"), Err(DeleteError::NotFound));

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/bar/<a>/<b>"), Err(DeleteError::NotFound));

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/bar/<a>/<b>/baz"), Err(DeleteError::NotFound));

    insta::assert_snapshot!(router, @"");

    assert_eq!(router.delete("/baz/<a>/<b>/<c>"), Err(DeleteError::NotFound));

    insta::assert_snapshot!(router, @"");

    Ok(())
}
