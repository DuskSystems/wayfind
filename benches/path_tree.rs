#![allow(clippy::too_many_lines)]

//! Benches sourced from `path-tree` (MIT OR Apache-2.0)
//! <https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs>

use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use path_tree_routes::paths;

pub mod path_tree_routes;

fn benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("path-tree benchmarks");

    group.bench_function("path-tree benchmarks/wayfind", |bencher| {
        let mut wayfind = wayfind::router::Router::new();
        for (index, route) in routes!(brackets).iter().enumerate() {
            wayfind.insert(route, index).unwrap();
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let mut path = wayfind::path::Path::new(path);
                let search = wayfind.search(&mut path).unwrap().unwrap();
                assert_eq!(search.data.value, index);
                let _ = search
                    .parameters
                    .iter()
                    .map(|p| (p.key, p.value))
                    .collect::<Vec<(&str, &str)>>();
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
            for (index, path) in paths() {
                let mut path = actix_router::Path::new(path);
                let n = router.recognize(&mut path).unwrap();
                assert_eq!(*n.0, index);
                let _ = path
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("path-tree benchmarks/matchit", |bencher| {
        let mut matcher = matchit::Router::new();
        for (index, route) in routes!(brackets).iter().enumerate() {
            let _ = matcher.insert(*route, index);
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = matcher.at(path).unwrap();
                assert_eq!(*n.value, index);
                let _ = n
                    .params
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>();
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
            for (index, path) in paths() {
                let mut path = ntex_router::Path::new(path);
                let n = router.recognize(&mut path).unwrap();
                assert_eq!(*n.0, index);
                let _ = path
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("path-tree benchmarks/path-tree", |bencher| {
        let mut tree = path_tree::PathTree::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            let _ = tree.insert(route, index);
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = tree.find(path).unwrap();
                assert_eq!(*n.0, index);
                let _ =
                    n.1.params_iter()
                        .map(|p| (p.0, p.1))
                        .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("path-tree benchmarks/regex", |bencher| {
        let regex_set = regex::RegexSet::new(routes!(regex)).unwrap();
        let regexes: Vec<_> = routes!(regex)
            .into_iter()
            .map(|pattern| regex::Regex::new(pattern).unwrap())
            .collect();

        bencher.iter(|| {
            for (index, path) in paths() {
                let matches = regex_set.matches(path).into_iter().collect::<Vec<_>>();
                assert!(matches.contains(&index));
                let i = matches.first().unwrap();
                let captures = regexes[*i].captures(path).unwrap();
                let _ = regexes[*i]
                    .capture_names()
                    .flatten()
                    .filter_map(|name| captures.name(name).map(|m| (name, m.as_str())))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("path-tree benchmarks/route-recognizer", |bencher| {
        let mut router = route_recognizer::Router::<usize>::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            router.add(route, index);
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = router.recognize(path).unwrap();
                assert_eq!(**n.handler(), index);
                let _ = n
                    .params()
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("path-tree benchmarks/routefinder", |bencher| {
        let mut router = routefinder::Router::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            router.add(*route, index).unwrap();
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = router.best_match(path).unwrap();
                assert_eq!(*n, index);
                let _ = n
                    .captures()
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>();
            }
        });
    });

    group.bench_function("path-tree benchmarks/xitca-router", |bencher| {
        let mut xitca = xitca_router::Router::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            xitca.insert(*route, index).unwrap();
        }

        bencher.iter(|| {
            for (index, path) in paths() {
                let n = xitca.at(path).unwrap();
                assert_eq!(*n.value, index);
                let _ = n
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