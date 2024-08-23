#![allow(clippy::too_many_lines)]

//! Benches sourced from `path-tree` (MIT OR Apache-2.0)
//! <https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs>

use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use path_tree_routes::paths;
use percent_encoding::percent_decode;
use std::hint::black_box;

pub mod path_tree_routes;

fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("path-tree benchmarks");

    group.bench_function("path-tree benchmarks/wayfind", |bencher| {
        let mut wayfind = wayfind::router::Router::new();
        for (index, route) in routes!(brackets).iter().enumerate() {
            wayfind.insert(route, index).unwrap();
        }

        bencher.iter(|| {
            for (index, path) in black_box(paths()) {
                let path = wayfind::path::Path::new(path).unwrap();
                let search = black_box(wayfind.search(&path).unwrap());
                assert_eq!(search.data.value, index);
                let _ = black_box(
                    search
                        .parameters
                        .iter()
                        .map(|p| (p.key, p.value))
                        .collect::<Vec<(&str, &str)>>(),
                );
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
            for (index, path) in black_box(paths()) {
                let path = black_box(percent_decode(path.as_bytes()).decode_utf8().unwrap());
                let mut path = actix_router::Path::new(path.as_ref());
                let n = black_box(router.recognize(&mut path).unwrap());
                assert_eq!(*n.0, index);
                let _ = black_box(
                    path.iter()
                        .map(|p| (p.0, p.1))
                        .collect::<Vec<(&str, &str)>>(),
                );
            }
        });
    });

    group.bench_function("path-tree benchmarks/matchit", |bencher| {
        let mut matcher = matchit::Router::new();
        for (index, route) in routes!(brackets).iter().enumerate() {
            let _ = matcher.insert(*route, index);
        }

        bencher.iter(|| {
            for (index, path) in black_box(paths()) {
                let path = black_box(percent_decode(path.as_bytes()).decode_utf8().unwrap());
                let n = black_box(matcher.at(path.as_ref()).unwrap());
                assert_eq!(*n.value, index);
                let _ = black_box(
                    n.params
                        .iter()
                        .map(|p| (p.0, p.1))
                        .collect::<Vec<(&str, &str)>>(),
                );
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
            for (index, path) in black_box(paths()) {
                let path = black_box(percent_decode(path.as_bytes()).decode_utf8().unwrap());
                let mut path = ntex_router::Path::new(path.as_ref());
                let n = black_box(router.recognize(&mut path).unwrap());
                assert_eq!(*n.0, index);
                let _ = black_box(
                    path.iter()
                        .map(|p| (p.0, p.1))
                        .collect::<Vec<(&str, &str)>>(),
                );
            }
        });
    });

    group.bench_function("path-tree benchmarks/path-tree", |bencher| {
        let mut tree = path_tree::PathTree::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            let _ = tree.insert(route, index);
        }

        bencher.iter(|| {
            for (index, path) in black_box(paths()) {
                let path = black_box(percent_decode(path.as_bytes()).decode_utf8().unwrap());
                let n = black_box(tree.find(path.as_ref()).unwrap());
                assert_eq!(*n.0, index);
                let _ = black_box(
                    n.1.params_iter()
                        .map(|p| (p.0, p.1))
                        .collect::<Vec<(&str, &str)>>(),
                );
            }
        });
    });

    group.bench_function("path-tree benchmarks/route-recognizer", |bencher| {
        let mut router = route_recognizer::Router::<usize>::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            router.add(route, index);
        }

        bencher.iter(|| {
            for (index, path) in black_box(paths()) {
                let path = black_box(percent_decode(path.as_bytes()).decode_utf8().unwrap());
                let n = black_box(router.recognize(path.as_ref()).unwrap());
                assert_eq!(**n.handler(), index);
                let _ = black_box(
                    n.params()
                        .iter()
                        .map(|p| (p.0, p.1))
                        .collect::<Vec<(&str, &str)>>(),
                );
            }
        });
    });

    group.bench_function("path-tree benchmarks/routefinder", |bencher| {
        let mut router = routefinder::Router::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            router.add(*route, index).unwrap();
        }

        bencher.iter(|| {
            for (index, path) in black_box(paths()) {
                let path = black_box(percent_decode(path.as_bytes()).decode_utf8().unwrap());
                let n = black_box(router.best_match(path.as_ref()).unwrap());
                assert_eq!(*n, index);
                let _ = black_box(
                    n.captures()
                        .iter()
                        .map(|p| (p.0, p.1))
                        .collect::<Vec<(&str, &str)>>(),
                );
            }
        });
    });

    group.bench_function("path-tree benchmarks/xitca-router", |bencher| {
        let mut xitca = xitca_router::Router::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            xitca.insert(*route, index).unwrap();
        }

        bencher.iter(|| {
            for (index, path) in black_box(paths()) {
                let path = black_box(percent_decode(path.as_bytes()).decode_utf8().unwrap());
                let n = black_box(xitca.at(path.as_ref()).unwrap());
                assert_eq!(*n.value, index);
                let _ = black_box(
                    n.params
                        .iter()
                        .map(|p| (p.0, p.1))
                        .collect::<Vec<(&str, &str)>>(),
                );
            }
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
