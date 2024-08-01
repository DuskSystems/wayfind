use crate::{
    matches::{Match, Parameter},
    router::Router,
};
use smallvec::SmallVec;
use std::fmt::Debug;

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

    (@parse_expected {
        path: $path:expr,
        value: $value:expr
        $(, params: {
            $($param_key:expr => $param_value:expr),+
        })?
    }) => {
        Some($crate::testing::ExpectedMatch {
            path: $path,
            value: $value,
            params: smallvec::smallvec![
                $(
                    $( $crate::matches::Parameter {
                        key: $param_key.as_bytes(),
                        value: $param_value.as_bytes(),
                    } ),+
                )?
            ]
        })
    };

    (@parse_expected None) => {
        None
    };
}

pub struct ExpectedMatch<'a, T> {
    pub path: &'a str,
    pub value: T,
    pub params: SmallVec<[Parameter<'a>; 4]>,
}

#[allow(clippy::missing_panics_doc)]
pub fn assert_router_match<'a, T: PartialEq + Debug>(
    router: &'a Router<'a, T>,
    input: &'a str,
    expected: Option<ExpectedMatch<'a, T>>,
) {
    let Some(Match { data, parameters }) = router.matches(input) else {
        assert!(expected.is_none(), "No match found for input: {input}");
        return;
    };

    if let Some(expected) = expected {
        assert_eq!(data.path, expected.path, "Path mismatch for input: {input}");
        assert_eq!(data.value, expected.value, "Value mismatch for input: {input}");

        let parameters = parameters
            .iter()
            .map(|param| (String::from_utf8_lossy(param.key), String::from_utf8_lossy(param.value)))
            .collect::<Vec<(_, _)>>();

        let expected = expected
            .params
            .iter()
            .map(|param| (String::from_utf8_lossy(param.key), String::from_utf8_lossy(param.value)))
            .collect::<Vec<(_, _)>>();

        assert_eq!(parameters, expected, "Parameters mismatch for input: {input}");
    } else {
        panic!("Unexpected match for input: {input}");
    }
}