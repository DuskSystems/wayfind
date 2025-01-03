//! Benches sourced from `matchit` (MIT AND BSD-3-Clause)
//! <https://github.com/ibraheemdev/matchit/blob/v0.8.5/benches/bench.rs>

use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use matchit_routes::paths;
use percent_encoding::percent_decode;
use std::hint::black_box;

pub mod matchit_routes;

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = matchit_benchmark
}

fn matchit_benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("matchit benchmarks");

    group.bench_function("matchit benchmarks/wayfind", |bencher| {
        let mut router = wayfind::Router::new();
        for route in routes!(brackets) {
            let route = wayfind::RouteBuilder::new().route(route).build().unwrap();
            router.insert(&route, true).unwrap();
        }

        bencher.iter(|| {
            for route in black_box(paths()) {
                let request = wayfind::RequestBuilder::new().path(route).build().unwrap();
                let output = black_box(router.search(black_box(&request)).unwrap().unwrap());
                let _parameters: Vec<(&str, &str)> =
                    black_box(output.path.parameters.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("matchit benchmarks/actix-router", |bencher| {
        let mut router = actix_router::Router::<bool>::build();
        for route in routes!(brackets) {
            router.path(route, true);
        }
        let router = router.finish();

        bencher.iter(|| {
            for route in black_box(paths()) {
                let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
                let mut path = actix_router::Path::new(path.as_ref());
                black_box(router.recognize(black_box(&mut path)).unwrap());
                let _parameters: Vec<(&str, &str)> =
                    black_box(path.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("matchit benchmarks/matchit", |bencher| {
        let mut router = matchit::Router::new();
        for route in routes!(brackets) {
            router.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in black_box(paths()) {
                let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
                let output = black_box(router.at(black_box(&path)).unwrap());
                let _parameters: Vec<(&str, &str)> =
                    black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("matchit benchmarks/ntex-router", |bencher| {
        let mut router = ntex_router::Router::<bool>::build();
        for route in routes!(brackets) {
            router.path(route, true);
        }
        let router = router.finish();

        bencher.iter(|| {
            for route in black_box(paths()) {
                let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
                let mut path = ntex_router::Path::new(path.as_ref());
                router.recognize(&mut path).unwrap();
                let _parameters: Vec<(&str, &str)> =
                    black_box(path.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("matchit benchmarks/path-tree", |bencher| {
        let mut router = path_tree::PathTree::new();
        for route in routes!(colon) {
            let _ = router.insert(route, true);
        }

        bencher.iter(|| {
            for route in black_box(paths()) {
                let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
                let output = router.find(&path).unwrap();
                let _parameters: Vec<(&str, &str)> =
                    black_box(output.1.params_iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("matchit benchmarks/route-recognizer", |bencher| {
        let mut router = route_recognizer::Router::new();
        for route in routes!(colon) {
            router.add(route, true);
        }

        bencher.iter(|| {
            for route in black_box(paths()) {
                let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
                let output = router.recognize(&path).unwrap();
                let _parameters: Vec<(&str, &str)> =
                    black_box(output.params().iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("matchit benchmarks/routefinder", |bencher| {
        let mut router = routefinder::Router::new();
        for route in routes!(colon) {
            router.add(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in black_box(paths()) {
                let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
                let output = router.best_match(&path).unwrap();
                let _parameters: Vec<(&str, &str)> =
                    black_box(output.captures().iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("matchit benchmarks/xitca-router", |bencher| {
        let mut router = xitca_router::Router::new();
        for route in routes!(colon) {
            router.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for route in black_box(paths()) {
                let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
                let output = router.at(&path).unwrap();
                let _parameters: Vec<(&str, &str)> =
                    black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.finish();
}
