#![allow(clippy::too_many_lines)]

//! Benches sourced from `matchit` (MIT AND BSD-3-Clause)
//! <https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs>

use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use matchit_routes::paths;

pub mod matchit_routes;

fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("matchit benchmarks");

    group.bench_function("wayfind", |bencher| {
        let mut wayfind = wayfind::router::Router::new();
        for route in routes!(chevrons) {
            wayfind.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in paths() {
                wayfind.matches(route).unwrap();
            }
        });
    });

    group.bench_function("actix-router", |bencher| {
        let mut actix = actix_router::Router::<bool>::build();
        for route in routes!(brackets) {
            actix.path(route, true);
        }
        let actix = actix.finish();

        bencher.iter(|| {
            for route in paths() {
                let mut path = actix_router::Path::new(route);
                actix.recognize(&mut path).unwrap();
            }
        });
    });

    group.bench_function("gonzales", |bencher| {
        let gonzales = gonzales::RouterBuilder::new().build(routes!(brackets));

        bencher.iter(|| {
            for route in paths() {
                gonzales.route(route).unwrap();
            }
        });
    });

    group.bench_function("matchit", |bencher| {
        let mut matchit = matchit::Router::new();
        for route in routes!(brackets) {
            matchit.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in paths() {
                matchit.at(route).unwrap();
            }
        });
    });

    group.bench_function("ntex-router", |bencher| {
        let mut ntex = ntex_router::Router::<bool>::build();
        for route in routes!(brackets) {
            ntex.path(route, true);
        }
        let ntex = ntex.finish();

        bencher.iter(|| {
            for route in paths() {
                let mut path = ntex_router::Path::new(route);
                ntex.recognize(&mut path).unwrap();
            }
        });
    });

    group.bench_function("path-table", |bencher| {
        let mut table = path_table::PathTable::new();
        for route in routes!(brackets) {
            *table.setup(route) = true;
        }

        bencher.iter(|| {
            for route in paths() {
                table.route(route).unwrap();
            }
        });
    });

    group.bench_function("path-tree", |bencher| {
        let mut path_tree = path_tree::PathTree::new();
        for route in routes!(colon) {
            let _ = path_tree.insert(route, true);
        }

        bencher.iter(|| {
            for route in paths() {
                path_tree.find(route).unwrap();
            }
        });
    });

    group.bench_function("regex", |bencher| {
        let regex_set = regex::RegexSet::new(routes!(regex)).unwrap();

        bencher.iter(|| {
            for route in paths() {
                regex_set.matches(route);
            }
        });
    });

    group.bench_function("route-recognizer", |bencher| {
        let mut route_recognizer = route_recognizer::Router::new();
        for route in routes!(colon) {
            route_recognizer.add(route, true);
        }

        bencher.iter(|| {
            for route in paths() {
                route_recognizer
                    .recognize(route)
                    .unwrap();
            }
        });
    });

    group.bench_function("routefinder", |bencher| {
        let mut routefinder = routefinder::Router::new();
        for route in routes!(colon) {
            routefinder.add(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in paths() {
                routefinder.best_match(route).unwrap();
            }
        });
    });

    group.bench_function("xitca-router", |bencher| {
        let mut xitca = xitca_router::Router::new();
        for route in routes!(colon) {
            xitca.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in paths() {
                xitca.at(route).unwrap();
            }
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
