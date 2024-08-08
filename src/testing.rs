use crate::{
    matches::{Match, Parameter},
    router::Router,
};
use http::Request;
use std::{fmt::Debug, sync::Arc};

#[macro_export]
macro_rules! assert_router_matches {
    ($router:expr, {
        $( $input:expr => $expected:tt )+
    }) => {
        $({
            let expected = assert_router_matches!(@parse_expected $expected);
            $crate::testing::assert_router_match(&$router, $input, expected);
        })+
    };

    (@parse_expected $expected:tt) => {
        $crate::parse_expected!($expected)
    };
}

#[macro_export]
macro_rules! assert_router_matches_request {
    ($router:expr, {
        $( $request:expr => $expected:tt )+
    }) => {
        $({
            let expected = assert_router_matches_request!(@parse_expected $expected);
            $crate::testing::assert_router_match_request(&$router, $request, expected);
        })+
    };

    (@parse_expected $expected:tt) => {
        $crate::parse_expected!($expected)
    };
}

#[macro_export]
macro_rules! parse_expected {
    ({
        path: $path:expr,
        value: $value:expr
        $(, params: {
            $($param_key:expr => $param_value:expr),+
        })?
    }) => {
        Some($crate::testing::ExpectedMatch {
            path: std::sync::Arc::from($path),
            value: $value,
            params: vec![
                $(
                    $( $crate::matches::Parameter {
                        key: $param_key.as_bytes(),
                        value: $param_value.as_bytes(),
                    } ),+
                )?
            ]
        })
    };

    (None) => {
        None
    };
}

pub struct ExpectedMatch<'k, 'v, T> {
    pub path: Arc<str>,
    pub value: T,
    pub params: Vec<Parameter<'k, 'v>>,
}

pub fn assert_router_match<'a, T: PartialEq + Debug>(
    router: &'a Router<T>,
    input: &'a str,
    expected: Option<ExpectedMatch<'_, 'a, T>>,
) {
    let result = router.matches(input);

    match (result, expected) {
        (None, None) => {}
        (None, Some(_)) => panic!("No match found for input: {input}"),
        (Some(_), None) => panic!("Unexpected match for input: {input}"),
        (Some(Match { data, parameters }), Some(expected)) => {
            assert_eq!(data.path, expected.path, "Path mismatch for input: {input}");
            assert_eq!(data.value, expected.value, "Value mismatch for input: {input}");
            assert_eq!(
                parameters.to_vec(),
                expected.params,
                "Parameters mismatch for input: {input}"
            );
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn assert_router_match_request<T: PartialEq + Debug, R>(
    router: &Router<T>,
    request: Request<R>,
    expected: Option<ExpectedMatch<'_, '_, T>>,
) {
    let result = router.matches_request(&request);
    let path = request.uri().path();

    match (result, expected) {
        (None, None) => {}
        (None, Some(_)) => panic!("No match found for request: {path}"),
        (Some(_), None) => panic!("Unexpected match for request: {path}"),
        (Some(Match { data, parameters }), Some(expected)) => {
            assert_eq!(data.path, expected.path, "Path mismatch for request: {path}");
            assert_eq!(data.value, expected.value, "Value mismatch for request: {path}");
            assert_eq!(
                parameters.to_vec(),
                expected.params,
                "Parameters mismatch for request: {path}"
            );
        }
    }
}
