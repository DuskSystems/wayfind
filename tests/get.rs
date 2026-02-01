use core::error::Error;

use wayfind::Router;

#[test]
fn get() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/hello", 42)?;

    assert_eq!(router.get("/hello"), Some(&42));
    Ok(())
}

#[test]
fn get_dynamic() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/users/<id>", 1)?;
    router.insert("/users/<id>/posts", 2)?;

    assert_eq!(router.get("/users/<id>"), Some(&1));
    assert_eq!(router.get("/users/<id>/posts"), Some(&2));
    assert_eq!(router.get("/users/<other>"), None);
    Ok(())
}

#[test]
fn get_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*path>/edit", 1)?;
    router.insert("/files/<*rest>", 2)?;

    assert_eq!(router.get("/<*path>/edit"), Some(&1));
    assert_eq!(router.get("/files/<*rest>"), Some(&2));
    assert_eq!(router.get("/<*other>/edit"), None);
    assert_eq!(router.get("/files/<*other>"), None);
    Ok(())
}

#[test]
fn get_end_wildcard() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/<*catch_all>", 1)?;

    assert_eq!(router.get("/<*catch_all>"), Some(&1));
    assert_eq!(router.get("/<*other>"), None);
    Ok(())
}

#[test]
fn get_missing() {
    let router: Router<i32> = Router::new();
    assert_eq!(router.get("/missing"), None);
}

#[test]
fn get_invalid() {
    let router: Router<i32> = Router::new();
    assert_eq!(router.get("/<invalid"), None);
}

#[test]
fn get_mut() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/hello", 1)?;

    if let Some(data) = router.get_mut("/hello") {
        *data = 2;
    }

    assert_eq!(router.get("/hello"), Some(&2));
    Ok(())
}

#[test]
fn get_mut_missing() {
    let mut router: Router<i32> = Router::new();
    assert_eq!(router.get_mut("/missing"), None);
}

#[test]
fn get_mut_invalid() {
    let mut router: Router<i32> = Router::new();
    assert_eq!(router.get_mut("/<invalid"), None);
}
