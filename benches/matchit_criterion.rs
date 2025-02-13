//! Benches sourced from `matchit` (MIT AND BSD-3-Clause)
//! <https://github.com/ibraheemdev/matchit/blob/v0.8.6/benches/bench.rs>

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use smallvec::SmallVec;

pub mod matchit_routes;

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = matchit_benchmark
}

fn matchit_benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("matchit benchmarks");

    group.bench_function("wayfind", |bencher| {
        let mut router = wayfind::Router::new();
        for route in routes!(brackets) {
            router.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let _output = black_box(router.search(black_box(path)).unwrap());
            }
        });
    });

    group.bench_function("wayfind (parameters)", |bencher| {
        let mut router = wayfind::Router::new();
        for route in routes!(brackets) {
            router.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let output = black_box(router.search(black_box(path)).unwrap());
                let _parameters: SmallVec<[(&str, &str); 4]> =
                    black_box(output.parameters.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("actix-router", |bencher| {
        let mut router = actix_router::Router::<bool>::build();
        for route in routes!(brackets) {
            router.path(route, true);
        }
        let router = router.finish();

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let mut path = actix_router::Path::new(path);
                let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
            }
        });
    });

    group.bench_function("actix-router (parameters)", |bencher| {
        let mut router = actix_router::Router::<bool>::build();
        for route in routes!(brackets) {
            router.path(route, true);
        }
        let router = router.finish();

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let mut path = actix_router::Path::new(path);
                let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
                let _parameters: SmallVec<[(&str, &str); 4]> =
                    black_box(path.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("matchit", |bencher| {
        let mut router = matchit::Router::new();
        for route in routes!(brackets) {
            router.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let _output = black_box(router.at(black_box(path)).unwrap());
            }
        });
    });

    group.bench_function("matchit (parameters)", |bencher| {
        let mut router = matchit::Router::new();
        for route in routes!(brackets) {
            router.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let output = black_box(router.at(black_box(path)).unwrap());
                let _parameters: SmallVec<[(&str, &str); 4]> =
                    black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("ntex-router", |bencher| {
        let mut router = ntex_router::Router::<bool>::build();
        for route in routes!(brackets) {
            router.path(route, true);
        }
        let router = router.finish();

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let mut path = ntex_router::Path::new(path);
                let _output = router.recognize(&mut path).unwrap();
            }
        });
    });

    group.bench_function("ntex-router (parameters)", |bencher| {
        let mut router = ntex_router::Router::<bool>::build();
        for route in routes!(brackets) {
            router.path(route, true);
        }
        let router = router.finish();

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let mut path = ntex_router::Path::new(path);
                let _output = router.recognize(&mut path).unwrap();
                let _parameters: SmallVec<[(&str, &str); 4]> =
                    black_box(path.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("path-tree", |bencher| {
        let mut router = path_tree::PathTree::new();
        for route in routes!(colon) {
            let _ = router.insert(route, true);
        }

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let _output = black_box(router.find(path).unwrap());
            }
        });
    });

    group.bench_function("path-tree (parameters)", |bencher| {
        let mut router = path_tree::PathTree::new();
        for route in routes!(colon) {
            let _ = router.insert(route, true);
        }

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let output = router.find(path).unwrap();
                let _parameters: SmallVec<[(&str, &str); 4]> =
                    black_box(output.1.params_iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("route-recognizer", |bencher| {
        let mut router = route_recognizer::Router::new();
        for route in routes!(colon) {
            router.add(route, true);
        }

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let _output = black_box(router.recognize(path).unwrap());
            }
        });
    });

    group.bench_function("route-recognizer (parameters)", |bencher| {
        let mut router = route_recognizer::Router::new();
        for route in routes!(colon) {
            router.add(route, true);
        }

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let output = router.recognize(path).unwrap();
                let _parameters: SmallVec<[(&str, &str); 4]> =
                    black_box(output.params().iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.bench_function("xitca-router", |bencher| {
        let mut router = xitca_router::Router::new();
        for route in routes!(colon) {
            router.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let _output = black_box(router.at(path).unwrap());
            }
        });
    });

    group.bench_function("xitca-router (parameters)", |bencher| {
        let mut router = xitca_router::Router::new();
        for route in routes!(colon) {
            router.insert(route, true).unwrap();
        }

        bencher.iter(|| {
            for path in black_box(routes!(literal)) {
                let output = router.at(path).unwrap();
                let _parameters: SmallVec<[(&str, &str); 4]> =
                    black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    });

    group.finish();
}
