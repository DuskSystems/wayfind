use similar_asserts::assert_eq;
use std::{fmt::Debug, sync::Arc};
use wayfind::{Match, Parameter, Path, Router};

#[macro_export]
macro_rules! assert_router_matches {
    ($router:expr, {
        $( $input:expr => $expected:tt )+
    }) => {
        $({
            let expected = assert_router_matches!(@parse_expected $expected);
            $crate::utils::assert_router_match(&$router, $input, expected);
        })+
    };

    (@parse_expected {
        route: $route:expr,
        data: $data:expr
        $(, params: {
            $($param_key:expr => $param_value:expr),+
        })?
    }) => {
        Some($crate::utils::ExpectedMatch {
            route: std::sync::Arc::from($route),
            data: $data,
            params: vec![
                $(
                    $( wayfind::Parameter {
                        key: $param_key,
                        value: $param_value,
                    } ),+
                )?
            ]
        })
    };

    (@parse_expected None) => {
        None
    };
}

pub struct ExpectedMatch<'k, 'v, T> {
    pub route: Arc<str>,
    pub data: T,
    pub params: Vec<Parameter<'k, 'v>>,
}

#[allow(clippy::missing_panics_doc)]
pub fn assert_router_match<'a, T: PartialEq + Debug>(
    router: &'a Router<T>,
    input: &'a str,
    expected: Option<ExpectedMatch<'_, 'a, T>>,
) {
    let path = Path::new(input).expect("Invalid path!");
    let Ok(Some(Match {
        route,
        data,
        parameters,
    })) = router.search(&path)
    else {
        assert!(expected.is_none(), "No match found for input: {input}");
        return;
    };

    if let Some(expected) = expected {
        assert_eq!(route, expected.route, "Path mismatch for input: {input}");
        assert_eq!(*data, expected.data, "Value mismatch for input: {input}");
        assert_eq!(
            parameters, expected.params,
            "Parameters mismatch for input: {input}"
        );
    } else {
        panic!("Unexpected match for input: {input}");
    }
}
