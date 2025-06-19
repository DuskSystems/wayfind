use std::error::Error;

use wayfind::Router;

#[test]
fn test_get() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/hello", 42)?;

    assert_eq!(router.get("/hello"), Some(&42));
    Ok(())
}

#[test]
fn test_get_missing() {
    let router: Router<i32> = Router::new();
    assert_eq!(router.get("/missing"), None);
}

#[test]
fn test_get_invalid() {
    let router: Router<i32> = Router::new();
    assert_eq!(router.get("/{invalid"), None);
}

#[test]
fn test_get_expanded() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("(/hello)", 123)?;

    assert_eq!(router.get("(/hello)"), Some(&123));
    assert_eq!(router.get("/hello"), None);

    Ok(())
}

#[test]
fn test_get_mut() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("/hello", 1)?;

    if let Some(data) = router.get_mut("/hello") {
        *data = 2;
    }

    assert_eq!(router.get("/hello"), Some(&2));
    Ok(())
}

#[test]
fn test_get_mut_missing() {
    let mut router: Router<i32> = Router::new();
    assert_eq!(router.get_mut("/missing"), None);
}

#[test]
fn test_get_mut_invalid() {
    let mut router: Router<i32> = Router::new();
    assert_eq!(router.get_mut("/{invalid"), None);
}

#[test]
fn test_get_mut_expanded() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();
    router.insert("(/world)", 456)?;

    if let Some(data) = router.get_mut("(/world)") {
        *data = 789;
    }

    assert_eq!(router.get("(/world)"), Some(&789));
    Ok(())
}
