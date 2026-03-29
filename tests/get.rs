#![expect(missing_docs, clippy::panic_in_result_fn, reason = "Tests")]

use core::error::Error;

use wayfind::RouterBuilder;

#[test]
fn get() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/hello", 42)?;

    assert_eq!(builder.get("/hello"), Some(&42));
    Ok(())
}

#[test]
fn get_dynamic() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/users/<id>", 1)?;
    builder.insert("/users/<id>/posts", 2)?;

    assert_eq!(builder.get("/users/<id>"), Some(&1));
    assert_eq!(builder.get("/users/<id>/posts"), Some(&2));
    assert_eq!(builder.get("/users/<other>"), None);
    Ok(())
}

#[test]
fn get_wildcard() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*path>/edit", 1)?;
    builder.insert("/files/<*rest>", 2)?;

    assert_eq!(builder.get("/<*path>/edit"), Some(&1));
    assert_eq!(builder.get("/files/<*rest>"), Some(&2));
    assert_eq!(builder.get("/<*other>/edit"), None);
    assert_eq!(builder.get("/files/<*other>"), None);
    Ok(())
}

#[test]
fn get_end_wildcard() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/<*catch_all>", 1)?;

    assert_eq!(builder.get("/<*catch_all>"), Some(&1));
    assert_eq!(builder.get("/<*other>"), None);
    Ok(())
}

#[test]
fn get_missing() {
    let builder: RouterBuilder<i32> = RouterBuilder::new();
    assert_eq!(builder.get("/missing"), None);
}

#[test]
fn get_invalid() {
    let builder: RouterBuilder<i32> = RouterBuilder::new();
    assert_eq!(builder.get("/<invalid"), None);
}

#[test]
fn get_mut() -> Result<(), Box<dyn Error>> {
    let mut builder = RouterBuilder::new();
    builder.insert("/hello", 1)?;

    if let Some(data) = builder.get_mut("/hello") {
        *data = 2;
    }

    assert_eq!(builder.get("/hello"), Some(&2));
    Ok(())
}

#[test]
fn get_mut_missing() {
    let mut builder: RouterBuilder<i32> = RouterBuilder::new();
    assert_eq!(builder.get_mut("/missing"), None);
}

#[test]
fn get_mut_invalid() {
    let mut builder: RouterBuilder<i32> = RouterBuilder::new();
    assert_eq!(builder.get_mut("/<invalid"), None);
}
