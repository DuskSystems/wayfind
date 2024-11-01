use similar_asserts::assert_eq;
use std::fmt::Debug;
use wayfind::{Match, Parameter, Path, Router};

pub struct ExpectedMatch<'k, 'v, T> {
    pub route: &'k str,
    pub expanded: Option<&'k str>,
    pub data: T,
    pub params: Vec<Parameter<'k, 'v>>,
}

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
        expanded: $expanded:expr,
        data: $data:expr
        $(, params: {
            $($param_key:expr => $param_value:expr),+
        })?
    }) => {
        Some($crate::utils::ExpectedMatch {
            route: $route,
            expanded: Some($expanded),
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

    (@parse_expected {
        route: $route:expr,
        data: $data:expr
        $(, params: {
            $($param_key:expr => $param_value:expr),+
        })?
    }) => {
        Some($crate::utils::ExpectedMatch {
            route: $route,
            expanded: None,
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

#[allow(clippy::missing_panics_doc)]
pub fn assert_router_match<'a, T: PartialEq + Debug>(
    router: &'a Router<T>,
    input: &'a str,
    expected: Option<ExpectedMatch<'_, 'a, T>>,
) {
    let path = Path::new(input).expect("Invalid path!");
    let Ok(Some(Match {
        route,
        expanded,
        data,
        parameters,
    })) = router.search(&path)
    else {
        assert!(expected.is_none(), "No match found for input: {input}");
        return;
    };

    if let Some(expected) = expected {
        assert_eq!(route, expected.route, "Route mismatch for input: {input}");
        assert_eq!(
            expanded, expected.expanded,
            "Expanded mismatch for input: {input}"
        );
        assert_eq!(*data, expected.data, "Data mismatch for input: {input}");
        assert_eq!(
            parameters.to_vec(),
            expected.params,
            "Parameters mismatch for input: {input}"
        );
    } else {
        panic!("Unexpected match for input: {input}");
    }
}
