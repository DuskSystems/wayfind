#![allow(clippy::too_many_lines)]

//! Benches sourced from `matchit` (MIT AND BSD-3-Clause)
//! <https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs>

use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use matchit_routes::paths;

pub mod matchit_routes;

fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("matchit benchmarks");

    group.bench_function("matchit benchmarks/wayfind", |bencher| {
        let mut wayfind = wayfind::router::Router::new();
        for route in routes!(brackets) {
            wayfind.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in paths() {
                let path = wayfind::path::Path::new(route).unwrap();
                let search = wayfind.search(&path).unwrap();
                let _ = search
                    .parameters
                    .iter()
                    .map(|p| (p.key, p.value))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("matchit benchmarks/actix-router", |bencher| {
        let mut actix = actix_router::Router::<bool>::build();
        for route in routes!(brackets) {
            actix.path(route, true);
        }
        let actix = actix.finish();

        bencher.iter(|| {
            for route in paths() {
                let mut path = actix_router::Path::new(route);
                actix.recognize(&mut path).unwrap();
                let _ = path
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("matchit benchmarks/matchit", |bencher| {
        let mut matchit = matchit::Router::new();
        for route in routes!(brackets) {
            matchit.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in paths() {
                let at = matchit.at(route).unwrap();
                let _ = at
                    .params
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("matchit benchmarks/ntex-router", |bencher| {
        let mut ntex = ntex_router::Router::<bool>::build();
        for route in routes!(brackets) {
            ntex.path(route, true);
        }
        let ntex = ntex.finish();

        bencher.iter(|| {
            for route in paths() {
                let mut path = ntex_router::Path::new(route);
                ntex.recognize(&mut path).unwrap();
                let _ = path
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("matchit benchmarks/path-tree", |bencher| {
        let mut path_tree = path_tree::PathTree::new();
        for route in routes!(colon) {
            let _ = path_tree.insert(route, true);
        }

        bencher.iter(|| {
            for route in paths() {
                let route = path_tree.find(route).unwrap();
                let _ = route
                    .1
                    .params_iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("matchit benchmarks/regex", |bencher| {
        let regex_set = regex::RegexSet::new(routes!(regex)).unwrap();
        let regexes: Vec<_> = routes!(regex)
            .into_iter()
            .map(|pattern| regex::Regex::new(pattern).unwrap())
            .collect();

        bencher.iter(|| {
            for route in paths() {
                let matches = regex_set.matches(route).into_iter().collect::<Vec<_>>();
                let index = matches.first().unwrap();
                let captures = regexes[*index].captures(route).unwrap();
                let _ = regexes[*index]
                    .capture_names()
                    .flatten()
                    .filter_map(|name| captures.name(name).map(|m| (name, m.as_str())))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("matchit benchmarks/route-recognizer", |bencher| {
        let mut route_recognizer = route_recognizer::Router::new();
        for route in routes!(colon) {
            route_recognizer.add(route, true);
        }

        bencher.iter(|| {
            for route in paths() {
                let recognize = route_recognizer.recognize(route).unwrap();
                let _ = recognize
                    .params()
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("matchit benchmarks/routefinder", |bencher| {
        let mut routefinder = routefinder::Router::new();
        for route in routes!(colon) {
            routefinder.add(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in paths() {
                let best_match = routefinder.best_match(route).unwrap();
                let _ = best_match
                    .captures()
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("matchit benchmarks/xitca-router", |bencher| {
        let mut xitca = xitca_router::Router::new();
        for route in routes!(colon) {
            xitca.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in paths() {
                let at = xitca.at(route).unwrap();
                let _ = at
                    .params
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
