//! Benches sourced from `matchit` (MIT AND BSD-3-Clause)
//! <https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs>

use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};
use matchit_routes::call;

pub mod matchit_routes;

fn compare_routers(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare Routers");

    let mut wayfind = wayfind::router::Router::new();
    for route in register!(brackets) {
        wayfind.insert(route, true);
    }
    group.bench_function("wayfind", |b| {
        b.iter(|| {
            for route in black_box(call()) {
                black_box(wayfind.matches(route).unwrap());
            }
        });
    });

    let mut matchit = matchit::Router::new();
    for route in register!(brackets) {
        matchit.insert(route, true).unwrap();
    }
    group.bench_function("matchit", |b| {
        b.iter(|| {
            for route in black_box(call()) {
                black_box(matchit.at(route).unwrap());
            }
        });
    });

    let mut path_tree = path_tree::PathTree::new();
    for route in register!(colon) {
        path_tree.insert(route, true);
    }
    group.bench_function("path-tree", |b| {
        b.iter(|| {
            for route in black_box(call()) {
                black_box(path_tree.find(route).unwrap());
            }
        });
    });

    let gonzales = gonzales::RouterBuilder::new().build(register!(brackets));
    group.bench_function("gonzales", |b| {
        b.iter(|| {
            for route in black_box(call()) {
                black_box(gonzales.route(route).unwrap());
            }
        });
    });

    let mut actix = actix_router::Router::<bool>::build();
    for route in register!(brackets) {
        actix.path(route, true);
    }
    let actix = actix.finish();
    group.bench_function("actix", |b| {
        b.iter(|| {
            for route in black_box(call()) {
                let mut path = actix_router::Path::new(route);
                black_box(actix.recognize(&mut path).unwrap());
            }
        });
    });

    let regex_set = regex::RegexSet::new(register!(regex)).unwrap();
    group.bench_function("regex", |b| {
        b.iter(|| {
            for route in black_box(call()) {
                black_box(regex_set.matches(route));
            }
        });
    });

    let mut route_recognizer = route_recognizer::Router::new();
    for route in register!(colon) {
        route_recognizer.add(route, true);
    }
    group.bench_function("route-recognizer", |b| {
        b.iter(|| {
            for route in black_box(call()) {
                black_box(
                    route_recognizer
                        .recognize(route)
                        .unwrap(),
                );
            }
        });
    });

    let mut routefinder = routefinder::Router::new();
    for route in register!(colon) {
        routefinder.add(route, true).unwrap();
    }
    group.bench_function("routefinder", |b| {
        b.iter(|| {
            for route in black_box(call()) {
                black_box(routefinder.best_match(route).unwrap());
            }
        });
    });

    group.finish();
}

criterion_group!(benches, compare_routers);
criterion_main!(benches);
