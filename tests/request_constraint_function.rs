#![allow(clippy::too_many_lines)]

use http::{HeaderMap, HeaderValue, Method, Request, Uri, Version};
use std::error::Error;
use wayfind::{
    assert_router_matches_request, constraints::request::RequestConstraint, route::RouteBuilder, router::Router,
};

fn is_get_method(method: &Method) -> bool {
    *method == Method::GET
}

fn has_json_content_type(headers: &HeaderMap<HeaderValue>) -> bool {
    headers
        .get("content-type")
        .map_or(false, |v| v == "application/json")
}

fn has_auth_header(headers: &HeaderMap<HeaderValue>) -> bool {
    headers.contains_key("authorization")
}

fn is_https(uri: &Uri) -> bool {
    uri.scheme_str() == Some("https")
}

fn is_http2(version: Version) -> bool {
    version == Version::HTTP_2
}

fn has_api_prefix(uri: &Uri) -> bool {
    uri.path().starts_with("/api")
}

#[test]
fn test_request_part_constraint_functions() -> Result<(), Box<dyn Error>> {
    let mut router = Router::new();

    router.insert(
        RouteBuilder::new("/api/users")
            .request_constraint(RequestConstraint::MethodFunction(is_get_method))
            .request_constraint(RequestConstraint::HeadersFunction(has_json_content_type))
            .build()?,
        1,
    )?;

    router.insert(
        RouteBuilder::new("/api/admin")
            .request_constraint(RequestConstraint::HeadersFunction(has_auth_header))
            .request_constraint(RequestConstraint::UriFunction(is_https))
            .build()?,
        2,
    )?;

    router.insert(
        RouteBuilder::new("/api/public")
            .request_constraint(RequestConstraint::MethodFunction(is_get_method))
            .build()?,
        3,
    )?;

    router.insert(
        RouteBuilder::new("/api/v2")
            .request_constraint(RequestConstraint::VersionFunction(is_http2))
            .request_constraint(RequestConstraint::UriFunction(has_api_prefix))
            .build()?,
        4,
    )?;

    insta::assert_snapshot!(router, @r###"
    $
    ╰─ /api/
           ├─ admin [2] [RequestConstraint::HeadersFunction, RequestConstraint::UriFunction]
           ├─ public [3] [RequestConstraint::MethodFunction]
           ├─ users [1] [RequestConstraint::MethodFunction, RequestConstraint::HeadersFunction]
           ╰─ v2 [4] [RequestConstraint::VersionFunction, RequestConstraint::UriFunction]
    "###);

    assert_router_matches_request!(router, {
        Request::builder()
            .method(Method::GET)
            .uri("/api/users")
            .header("content-type", "application/json")
            .body(())?
        => {
            path: "/api/users",
            value: 1
        }

        Request::builder()
            .method(Method::POST)
            .uri("/api/users")
            .header("content-type", "application/json")
            .body(())?
        => None

        Request::builder()
            .method(Method::GET)
            .uri("https://example.com/api/admin")
            .header("authorization", "Bearer token")
            .body(())?
        => {
            path: "/api/admin",
            value: 2
        }

        Request::builder()
            .method(Method::GET)
            .uri("http://example.com/api/admin")
            .header("authorization", "Bearer token")
            .body(())?
        => None

        Request::builder()
            .method(Method::GET)
            .uri("/api/public")
            .body(())?
        => {
            path: "/api/public",
            value: 3
        }

        Request::builder()
            .method(Method::POST)
            .uri("/api/public")
            .body(())?
        => None

        Request::builder()
            .method(Method::GET)
            .uri("/api/v2")
            .version(Version::HTTP_2)
            .body(())?
        => {
            path: "/api/v2",
            value: 4
        }

        Request::builder()
            .method(Method::GET)
            .uri("/api/v2")
            .version(Version::HTTP_11)
            .body(())?
        => None

        Request::builder()
            .method(Method::GET)
            .uri("/v2")
            .version(Version::HTTP_2)
            .body(())?
        => None
    });

    Ok(())
}
