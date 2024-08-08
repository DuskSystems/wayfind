// use http::{Method, Request};
// use std::error::Error;
// use wayfind::{
//     assert_router_matches_request, constraints::request::RequestConstraint, route::RouteBuilder, router::Router,
// };

// fn is_get_request<R>(request: &Request<R>) -> bool {
//     request.method() == Method::GET
// }

// fn has_json_content_type<R>(request: &Request<R>) -> bool {
//     request
//         .headers()
//         .get("content-type")
//         .map_or(false, |v| v == "application/json")
// }

// fn has_auth_header<R>(request: &Request<R>) -> bool {
//     request
//         .headers()
//         .contains_key("authorization")
// }

// fn is_https<R>(request: &Request<R>) -> bool {
//     request.uri().scheme_str() == Some("https")
// }

// #[test]
// fn test_request_constraint_functions() -> Result<(), Box<dyn Error>> {
//     let mut router = Router::new();

//     router.insert(
//         RouteBuilder::new("/api/users")
//             .request_constraint(RequestConstraint::Function(is_get_request))
//             .request_constraint(RequestConstraint::Function(has_json_content_type))
//             .build()?,
//         1,
//     )?;

//     router.insert(
//         RouteBuilder::new("/api/admin")
//             .request_constraint(RequestConstraint::Function(has_auth_header))
//             .request_constraint(RequestConstraint::Function(is_https))
//             .build()?,
//         2,
//     )?;

//     router.insert(
//         RouteBuilder::new("/api/public")
//             .request_constraint(RequestConstraint::Function(is_get_request))
//             .build()?,
//         3,
//     )?;

//     insta::assert_snapshot!(router, @r###"
//     $
//     ╰─ /api/
//            ├─ admin [2] [RequestConstraint::Function, RequestConstraint::Function]
//            ├─ public [3] [RequestConstraint::Function]
//            ╰─ users [1] [RequestConstraint::Function, RequestConstraint::Function]
//     "###);

//     assert_router_matches_request!(router, {
//         Request::builder()
//             .method(Method::GET)
//             .uri("/api/users")
//             .header("content-type", "application/json")
//             .body(())?
//         => {
//             path: "/api/users",
//             value: 1
//         }

//         Request::builder()
//             .method(Method::POST)
//             .uri("/api/users")
//             .header("content-type", "application/json")
//             .body(())?
//         => None

//         Request::builder()
//             .method(Method::GET)
//             .uri("https://example.com/api/admin")
//             .header("authorization", "Bearer token")
//             .body(())?
//         => {
//             path: "/api/admin",
//             value: 2
//         }

//         Request::builder()
//             .method(Method::GET)
//             .uri("http://example.com/api/admin")
//             .header("authorization", "Bearer token")
//             .body(())?
//         => None

//         Request::builder()
//             .method(Method::GET)
//             .uri("/api/public")
//             .body(())?
//         => {
//             path: "/api/public",
//             value: 3
//         }

//         Request::builder()
//             .method(Method::POST)
//             .uri("/api/public")
//             .body(())?
//         => None
//     });

//     Ok(())
// }
