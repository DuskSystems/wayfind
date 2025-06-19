//! Test binary for easy `perf` checks.

use std::hint::black_box;

#[path = "../benches/matchit_routes.rs"]
pub mod routes;

// #[path = "../benches/path_tree_routes.rs"]
// pub mod routes;

fn main() {
    let mut router = ::wayfind::Router::new();
    for route in routes!(brackets) {
        router.insert(route, true).unwrap();
    }

    for _ in 0..1_000_000 {
        for path in black_box(routes!(brackets)) {
            let _unused = black_box(router.search(black_box(path)).unwrap());
        }
    }
}
