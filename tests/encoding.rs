use smallvec::smallvec;
use std::error::Error;
use wayfind::{
    errors::{EncodingError, PathSearchError, RequestError, RouteError, SearchError},
    Match, PathMatch, RequestBuilder, RouteBuilder, Router,
};

#[test]
fn test_encoding_decoding() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/users/{name}").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /users/
    ╰─ {name} [*]
    ");

    let request = RequestBuilder::new().path("/users/jos%C3%A9").build()?; // "José"
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/users/{name}",
                expanded: None,
                parameters: smallvec![("name", "josé")],
            },
        })
    );

    Ok(())
}

#[test]
fn test_encoding_space() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/user files/{name}").build()?;
    router.insert(&route, 1)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /user files/
    ╰─ {name} [*]
    ");

    let request = RequestBuilder::new()
        .path("/user%20files/document%20name")
        .build()?; // "/user files/document name"
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/user files/{name}",
                expanded: None,
                parameters: smallvec![("name", "document name")],
            },
        })
    );

    Ok(())
}

#[test]
fn test_encoding_slash() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/{name}").build()?;
    router.insert(&route, 1)?;
    let route = RouteBuilder::new().route("/{*path}").build()?;
    router.insert(&route, 2)?;

    insta::assert_snapshot!(router, @r"
    === Path
    /
    ├─ {name} [*]
    ╰─ {*path} [*]
    ");

    let request = RequestBuilder::new().path("/johndoe").build()?;
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &1,
            path: PathMatch {
                route: "/{name}",
                expanded: None,
                parameters: smallvec![("name", "johndoe")],
            },
        })
    );

    let request = RequestBuilder::new().path("/john%2Fdoe").build()?; // "john/doe"
    let search = router.search(&request)?;
    assert_eq!(
        search,
        Some(Match {
            data: &2,
            path: PathMatch {
                route: "/{*path}",
                expanded: None,
                parameters: smallvec![("path", "john/doe")],
            },
        })
    );

    Ok(())
}

#[test]
fn test_encoding_invalid_path() {
    let request = RequestBuilder::new().path("/users/%GG").build();
    assert_eq!(
        request,
        Err(RequestError::Encoding(EncodingError::InvalidEncoding {
            input: "/users/%GG".to_owned(),
            position: 7,
            character: *b"%GG"
        }))
    );
}

#[test]
fn test_encoding_invalid_parameter() {
    let route = RouteBuilder::new().route("/users/{%GG}").build();
    assert_eq!(
        route,
        Err(RouteError::Encoding(EncodingError::InvalidEncoding {
            input: "/users/{%GG}".to_owned(),
            position: 8,
            character: *b"%GG"
        }))
    );
}

#[test]
fn test_encoding_invalid_constraint() {
    let route = RouteBuilder::new().route("/users/{id:%GG}").build();
    assert_eq!(
        route,
        Err(RouteError::Encoding(EncodingError::InvalidEncoding {
            input: "/users/{id:%GG}".to_owned(),
            position: 11,
            character: *b"%GG"
        }))
    );
}

#[test]
fn test_encoding_invalid_value() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    let route = RouteBuilder::new().route("/files/{name}").build()?;
    router.insert(&route, 1)?;

    let request = RequestBuilder::new().path("/files/my%80file").build()?;
    let search = router.search(&request);
    assert_eq!(
        search,
        Err(SearchError::Path(PathSearchError::EncodingError(
            EncodingError::Utf8Error {
                input: "my�file".to_owned()
            }
        )))
    );

    Ok(())
}
