#![allow(clippy::too_many_lines)]

//! Benches sourced from `path-tree` (MIT OR Apache-2.0)
//! <https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs>

use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use path_tree_routes::paths;

pub mod path_tree_routes;

fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("path-tree benchmarks");

    group.bench_function("wayfind", |bencher| {
        let mut wayfind = wayfind::router::Router::new();
        for (index, route) in routes!(chevrons).iter().enumerate() {
            wayfind.insert(route, index).unwrap();
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = wayfind.matches(path).unwrap();
                assert_eq!(n.data.value, index);
            }
        });
    });

    group.bench_function("actix-router", |bencher| {
        let mut router = actix_router::Router::<usize>::build();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.path(*route, index);
        }
        let router = router.finish();

        bencher.iter(|| {
            for (index, path) in paths() {
                let mut path = actix_router::Path::new(path);
                let n = router.recognize(&mut path).unwrap();
                assert_eq!(*n.0, index);
            }
        });
    });

    group.bench_function("gonzales", |bencher| {
        let gonzales = gonzales::RouterBuilder::new().build(routes!(brackets));

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = gonzales.route(path).unwrap();
                assert_eq!(n.get_index(), index);
            }
        });
    });

    group.bench_function("matchit", |bencher| {
        let mut matcher = matchit::Router::new();
        for (index, route) in routes!(brackets).iter().enumerate() {
            let _ = matcher.insert(*route, index);
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = matcher.at(path).unwrap();
                assert_eq!(*n.value, index);
            }
        });
    });

    group.bench_function("ntex-router", |bencher| {
        let mut router = ntex_router::Router::<usize>::build();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.path(*route, index);
        }
        let router = router.finish();

        bencher.iter(|| {
            for (index, path) in paths() {
                let mut path = ntex_router::Path::new(path);
                let n = router.recognize(&mut path).unwrap();
                assert_eq!(*n.0, index);
            }
        });
    });

    group.bench_function("path-table", |bencher| {
        let mut table = path_table::PathTable::new();
        for (index, route) in routes!(brackets).iter().enumerate() {
            *table.setup(route) = index;
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = table.route(path).unwrap();
                assert_eq!(*n.0, index);
            }
        });
    });

    group.bench_function("path-tree", |bencher| {
        let mut tree = path_tree::PathTree::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            let _ = tree.insert(route, index);
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = tree.find(path).unwrap();
                assert_eq!(*n.0, index);
            }
        });
    });

    group.bench_function("regex", |bencher| {
        let regex_set = regex::RegexSet::new(routes!(regex)).unwrap();

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = regex_set.matches(path);
                assert!(n.matched(index));
            }
        });
    });

    group.bench_function("route-recognizer", |bencher| {
        let mut router = route_recognizer::Router::<usize>::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            router.add(route, index);
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = router.recognize(path).unwrap();
                assert_eq!(**n.handler(), index);
            }
        });
    });

    group.bench_function("routefinder", |bencher| {
        let mut router = routefinder::Router::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            router.add(*route, index).unwrap();
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = router.best_match(path).unwrap();
                assert_eq!(*n, index);
            }
        });
    });

    group.bench_function("xitca-router", |bencher| {
        let mut xitca = xitca_router::Router::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            xitca.insert(*route, index).unwrap();
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = xitca.at(path).unwrap();
                assert_eq!(*n.value, index);
            }
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
