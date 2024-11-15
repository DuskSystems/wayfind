use smallvec::smallvec;
use std::error::Error;
use wayfind::{
    errors::{EncodingError, PathError, RoutableError, SearchError},
    Match, Path, RoutableBuilder, Router,
};

#[test]
fn test_encoding_decoding() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/users/{name}").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /users/
    ╰─ {name} [*]
    ");

    let path = Path::new("/users/jos%C3%A9")?; // "José"
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/users/{name}",
            expanded: None,
            data: &1,
            parameters: smallvec![("name", "josé")],
        })
    );

    Ok(())
}

#[test]
fn test_encoding_space() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/user files/{name}").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    /user files/
    ╰─ {name} [*]
    ");

    let path = Path::new("/user%20files/document%20name")?; // "/user files/document name"
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/user files/{name}",
            expanded: None,
            data: &1,
            parameters: smallvec![("name", "document name")],
        })
    );

    Ok(())
}

#[test]
fn test_encoding_slash() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/{name}").build()?;
    router.insert(&route, 1)?;
    let route = RoutableBuilder::new().route("/{*path}").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    /
    ├─ {name} [*]
    ╰─ {*path} [*]
    ");

    let path = Path::new("/johndoe")?;
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{name}",
            expanded: None,
            data: &1,
            parameters: smallvec![("name", "johndoe")],
        })
    );

    let path = Path::new("/john%2Fdoe")?; // "john/doe"
    let search = router.search(&path)?;
    assert_eq!(
        search,
        Some(Match {
            route: "/{*path}",
            expanded: None,
            data: &2,
            parameters: smallvec![("path", "john/doe")],
        })
    );

    Ok(())
}

#[test]
fn test_encoding_invalid_path() {
    let path = Path::new("/users/%GG");
    assert_eq!(
        path,
        Err(PathError::EncodingError(EncodingError::InvalidEncoding {
            input: "/users/%GG".to_owned(),
            position: 7,
            character: *b"%GG"
        }))
    );
}

#[test]
fn test_encoding_invalid_parameter() {
    let route = RoutableBuilder::new().route("/users/{%GG}").build();
    assert_eq!(
        route,
        Err(RoutableError::EncodingError(
            EncodingError::InvalidEncoding {
                input: "/users/{%GG}".to_owned(),
                position: 8,
                character: *b"%GG"
            }
        ))
    );
}

#[test]
fn test_encoding_invalid_constraint() {
    let route = RoutableBuilder::new().route("/users/{id:%GG}").build();
    assert_eq!(
        route,
        Err(RoutableError::EncodingError(
            EncodingError::InvalidEncoding {
                input: "/users/{id:%GG}".to_owned(),
                position: 11,
                character: *b"%GG"
            }
        ))
    );
}

#[test]
fn test_encoding_invalid_value() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RoutableBuilder::new().route("/files/{name}").build()?;
    router.insert(&route, 1)?;

    let path = Path::new("/files/my%80file")?;
    let search = router.search(&path);
    assert_eq!(
        search,
        Err(SearchError::EncodingError(EncodingError::Utf8Error {
            input: "my�file".to_owned()
        }))
    );

    Ok(())
}
