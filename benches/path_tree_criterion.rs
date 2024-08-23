#![allow(clippy::too_many_lines)]

//! Benches sourced from `path-tree` (MIT OR Apache-2.0)
//! <https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs>

use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use path_tree_routes::paths;
use percent_encoding::percent_decode;
use std::hint::black_box;

pub mod path_tree_routes;

criterion_group!(benches, benchmark);
criterion_main!(benches);

fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("path-tree benchmarks");

    group.bench_function("path-tree benchmarks/wayfind", |bencher| {
        let mut router = wayfind::router::Router::new();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.insert(route, index).unwrap();
        }

        bencher.iter(|| {
            for route in black_box(paths()) {
                let path = wayfind::path::Path::new(route).unwrap();
                let output = black_box(router.search(black_box(&path)).unwrap());
                let _parameters: Vec<(&str, &str)> =
                    black_box(output.parameters.iter().map(|p| (p.key, p.value)).collect());
            }
        });
    });

    group.bench_function("path-tree benchmarks/actix-router", |bencher| {
        let mut router = actix_router::Router::<usize>::build();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.path(*route, index);
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

    group.bench_function("path-tree benchmarks/matchit", |bencher| {
        let mut router = matchit::Router::new();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.insert(*route, index).unwrap();
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

    group.bench_function("path-tree benchmarks/ntex-router", |bencher| {
        let mut router = ntex_router::Router::<usize>::build();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.path(*route, index);
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

    group.bench_function("path-tree benchmarks/path-tree", |bencher| {
        let mut router = path_tree::PathTree::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            let _ = router.insert(route, index);
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

    group.bench_function("path-tree benchmarks/route-recognizer", |bencher| {
        let mut router = route_recognizer::Router::<usize>::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            router.add(route, index);
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

    group.bench_function("path-tree benchmarks/routefinder", |bencher| {
        let mut router = routefinder::Router::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            router.add(*route, index).unwrap();
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

    group.bench_function("path-tree benchmarks/xitca-router", |bencher| {
        let mut router = xitca_router::Router::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            router.insert(*route, index).unwrap();
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
