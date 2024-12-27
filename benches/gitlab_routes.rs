//! Routes extracted from GitLab via `rails routes` command.
//! <https://gist.github.com/CathalMullan/23753db0a6c38b8f74fe288383670583>

use fancy_regex::Regex;
use serde_json::Value;
use std::sync::LazyLock;
use wayfind::{PathConstraint, RouteBuilder, Router};

const ROUTES_JSON: &str = include_str!("../crates/rails/output/routes.json");
static ROUTES_DATA: LazyLock<Value> = LazyLock::new(|| serde_json::from_str(ROUTES_JSON).unwrap());

wayfind_rails_macro::generate_constraints!("crates/rails/output/routes.json");

#[allow(clippy::large_stack_frames, clippy::missing_panics_doc)]
#[must_use]
pub fn routes<'p>() -> impl IntoIterator<Item = RouteBuilder<'p>> {
    ROUTES_DATA["routes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|route| {
            let mut builder = RouteBuilder::new().route(route["path"].as_str().unwrap());

            if let Some(methods) = route["methods"].as_array() {
                if !methods.is_empty() {
                    builder = builder.methods(
                        methods
                            .iter()
                            .map(|m| m.as_str().unwrap())
                            .collect::<Vec<_>>(),
                    );
                }
            }

            builder
        })
        .collect::<Vec<_>>()
}
