//! Benches sourced from `path-tree` (MIT OR Apache-2.0)
//! <https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs>

use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use path_tree_routes::{ROUTES_URLS, ROUTES_WITH_BRACES, ROUTES_WITH_COLON};

pub mod path_tree_routes;

fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("path-tree benchmarks");

    group
        .bench_function("wayfind", |bencher| {
            let mut wayfind = wayfind::router::Router::new();
            for (index, route) in ROUTES_WITH_BRACES.iter().enumerate() {
                wayfind.insert(route, index);
            }

            bencher.iter(|| {
                for (index, route) in ROUTES_URLS.iter().enumerate() {
                    let n = wayfind.matches(route).unwrap();
                    assert_eq!(n.data.value, index);
                }
            });
        })
        .bench_function("actix-router", |bencher| {
            let mut router = actix_router::Router::<usize>::build();
            for (index, route) in ROUTES_WITH_BRACES.iter().enumerate() {
                router.path(*route, index);
            }
            let router = router.finish();

            bencher.iter(|| {
                for (index, route) in ROUTES_URLS.iter().enumerate() {
                    let mut path = actix_router::Path::new(*route);
                    let n = router.recognize(&mut path).unwrap();
                    assert_eq!(*n.0, index);
                }
            });
        })
        .bench_function("ntex-router", |bencher| {
            let mut router = ntex_router::Router::<usize>::build();
            for (index, route) in ROUTES_WITH_BRACES.iter().enumerate() {
                router.path(*route, index);
            }
            let router = router.finish();

            bencher.iter(|| {
                for (index, route) in ROUTES_URLS.iter().enumerate() {
                    let mut path = ntex_router::Path::new(*route);
                    let n = router.recognize(&mut path).unwrap();
                    assert_eq!(*n.0, index);
                }
            });
        })
        .bench_function("path-table", |bencher| {
            let mut table = path_table::PathTable::new();
            for (index, route) in ROUTES_WITH_BRACES.iter().enumerate() {
                *table.setup(route) = index;
            }

            bencher.iter(|| {
                for (index, route) in ROUTES_URLS.iter().enumerate() {
                    let n = table.route(route).unwrap();
                    assert_eq!(*n.0, index);
                }
            });
        })
        .bench_function("path-tree", |bencher| {
            let mut tree = path_tree::PathTree::new();
            for (index, route) in ROUTES_WITH_COLON.iter().enumerate() {
                let _ = tree.insert(route, index);
            }

            bencher.iter(|| {
                for (index, route) in ROUTES_URLS.iter().enumerate() {
                    let n = tree.find(route).unwrap();
                    assert_eq!(*n.0, index);
                }
            });
        })
        .bench_function("matchit", |bencher| {
            let mut matcher = matchit::Router::new();
            for (index, route) in ROUTES_WITH_BRACES.iter().enumerate() {
                let _ = matcher.insert(*route, index);
            }

            bencher.iter(|| {
                for (index, route) in ROUTES_URLS.iter().enumerate() {
                    let n = matcher.at(route).unwrap();
                    assert_eq!(*n.value, index);
                }
            });
        })
        .bench_function("route-recognizer", |bencher| {
            let mut router = route_recognizer::Router::<usize>::new();
            for (index, route) in ROUTES_WITH_COLON.iter().enumerate() {
                router.add(route, index);
            }

            bencher.iter(|| {
                for (index, route) in ROUTES_URLS.iter().enumerate() {
                    let n = router.recognize(route).unwrap();
                    assert_eq!(**n.handler(), index);
                }
            });
        });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
