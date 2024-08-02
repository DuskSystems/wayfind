//! Benches sourced from `matchit` (MIT AND BSD-3-Clause)
//! <https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs>

use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};
use matchit_routes::call;

pub mod matchit_routes;

fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("matchit benchmarks");

    group.bench_function("wayfind", |bencher| {
        let mut wayfind = wayfind::router::Router::new();
        for route in register!(brackets) {
            wayfind.insert(route, true);
        }

        bencher.iter(|| {
            for route in black_box(call()) {
                black_box(wayfind.matches(route).unwrap());
            }
        });
    });

    group.bench_function("matchit", |bencher| {
        let mut matchit = matchit::Router::new();
        for route in register!(brackets) {
            matchit.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in black_box(call()) {
                black_box(matchit.at(route).unwrap());
            }
        });
    });

    group.bench_function("path-tree", |bencher| {
        let mut path_tree = path_tree::PathTree::new();
        for route in register!(colon) {
            let _ = path_tree.insert(route, true);
        }

        bencher.iter(|| {
            for route in black_box(call()) {
                black_box(path_tree.find(route).unwrap());
            }
        });
    });

    group.bench_function("gonzales", |bencher| {
        let gonzales = gonzales::RouterBuilder::new().build(register!(brackets));

        bencher.iter(|| {
            for route in black_box(call()) {
                black_box(gonzales.route(route).unwrap());
            }
        });
    });

    group.bench_function("actix-router", |bencher| {
        let mut actix = actix_router::Router::<bool>::build();
        for route in register!(brackets) {
            actix.path(route, true);
        }
        let actix = actix.finish();

        bencher.iter(|| {
            for route in black_box(call()) {
                let mut path = actix_router::Path::new(route);
                black_box(actix.recognize(&mut path).unwrap());
            }
        });
    });

    group.bench_function("regex", |bencher| {
        let regex_set = regex::RegexSet::new(register!(regex)).unwrap();

        bencher.iter(|| {
            for route in black_box(call()) {
                black_box(regex_set.matches(route));
            }
        });
    });

    group.bench_function("route-recognizer", |bencher| {
        let mut route_recognizer = route_recognizer::Router::new();
        for route in register!(colon) {
            route_recognizer.add(route, true);
        }

        bencher.iter(|| {
            for route in black_box(call()) {
                black_box(
                    route_recognizer
                        .recognize(route)
                        .unwrap(),
                );
            }
        });
    });

    group.bench_function("routefinder", |bencher| {
        let mut routefinder = routefinder::Router::new();
        for route in register!(colon) {
            routefinder.add(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in black_box(call()) {
                black_box(routefinder.best_match(route).unwrap());
            }
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
