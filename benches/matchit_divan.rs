//! Benches sourced from `matchit` (MIT AND BSD-3-Clause)
//! <https://github.com/ibraheemdev/matchit/blob/v0.8.6/benches/bench.rs>

use criterion::black_box;
use divan::AllocProfiler;
use smallvec::SmallVec;

pub mod matchit_routes;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(name = "wayfind")]
fn wayfind(bencher: divan::Bencher<'_, '_>) {
    let mut router = wayfind::Router::new();
    for route in routes!(brackets) {
        router.insert(route, true).unwrap();
    }

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let _output = black_box(router.search(black_box(path)).unwrap());
        }
    });
}

#[divan::bench(name = "wayfind (parameters)")]
fn wayfind_params(bencher: divan::Bencher<'_, '_>) {
    let mut router = wayfind::Router::new();
    for route in routes!(brackets) {
        router.insert(route, true).unwrap();
    }

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let output = black_box(router.search(black_box(path)).unwrap());
            let _parameters: SmallVec<[(&str, &str); 4]> =
                black_box(output.parameters.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "actix-router")]
fn actix_router(bencher: divan::Bencher<'_, '_>) {
    let mut router = actix_router::Router::<bool>::build();
    for route in routes!(brackets) {
        router.path(route, true);
    }
    let router = router.finish();

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let mut path = actix_router::Path::new(path);
            let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
        }
    });
}

#[divan::bench(name = "actix-router (parameters)")]
fn actix_router_params(bencher: divan::Bencher<'_, '_>) {
    let mut router = actix_router::Router::<bool>::build();
    for route in routes!(brackets) {
        router.path(route, true);
    }
    let router = router.finish();

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let mut path = actix_router::Path::new(path);
            let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
            let _parameters: SmallVec<[(&str, &str); 4]> =
                black_box(path.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "matchit")]
fn matchit(bencher: divan::Bencher<'_, '_>) {
    let mut router = matchit::Router::new();
    for route in routes!(brackets) {
        router.insert(route, true).unwrap();
    }

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let _output = black_box(router.at(black_box(path)).unwrap());
        }
    });
}

#[divan::bench(name = "matchit (parameters)")]
fn matchit_params(bencher: divan::Bencher<'_, '_>) {
    let mut router = matchit::Router::new();
    for route in routes!(brackets) {
        router.insert(route, true).unwrap();
    }

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let output = black_box(router.at(black_box(path)).unwrap());
            let _parameters: SmallVec<[(&str, &str); 4]> =
                black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "ntex-router")]
fn ntex_router(bencher: divan::Bencher<'_, '_>) {
    let mut router = ntex_router::Router::<bool>::build();
    for route in routes!(brackets) {
        router.path(route, true);
    }
    let router = router.finish();

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let mut path = ntex_router::Path::new(path);
            let _output = router.recognize(&mut path).unwrap();
        }
    });
}

#[divan::bench(name = "ntex-router (parameters)")]
fn ntex_router_params(bencher: divan::Bencher<'_, '_>) {
    let mut router = ntex_router::Router::<bool>::build();
    for route in routes!(brackets) {
        router.path(route, true);
    }
    let router = router.finish();

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let mut path = ntex_router::Path::new(path);
            let _output = router.recognize(&mut path).unwrap();
            let _parameters: SmallVec<[(&str, &str); 4]> =
                black_box(path.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "path-tree")]
fn path_tree(bencher: divan::Bencher<'_, '_>) {
    let mut router = path_tree::PathTree::new();
    for route in routes!(colon) {
        let _ = router.insert(route, true);
    }

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let _output = black_box(router.find(path).unwrap());
        }
    });
}

#[divan::bench(name = "path-tree (parameters)")]
fn path_tree_params(bencher: divan::Bencher<'_, '_>) {
    let mut router = path_tree::PathTree::new();
    for route in routes!(colon) {
        let _ = router.insert(route, true);
    }

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let output = router.find(path).unwrap();
            let _parameters: SmallVec<[(&str, &str); 4]> =
                black_box(output.1.params_iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "route-recognizer")]
fn route_recognizer(bencher: divan::Bencher<'_, '_>) {
    let mut router = route_recognizer::Router::new();
    for route in routes!(colon) {
        router.add(route, true);
    }

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let _output = black_box(router.recognize(path).unwrap());
        }
    });
}

#[divan::bench(name = "route-recognizer (parameters)")]
fn route_recognizer_params(bencher: divan::Bencher<'_, '_>) {
    let mut router = route_recognizer::Router::new();
    for route in routes!(colon) {
        router.add(route, true);
    }

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let output = router.recognize(path).unwrap();
            let _parameters: SmallVec<[(&str, &str); 4]> =
                black_box(output.params().iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "xitca-router")]
fn xitca_router(bencher: divan::Bencher<'_, '_>) {
    let mut router = xitca_router::Router::new();
    for route in routes!(colon) {
        router.insert(route, true).unwrap();
    }

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let _output = black_box(router.at(path).unwrap());
        }
    });
}

#[divan::bench(name = "xitca-router (parameters)")]
fn xitca_router_params(bencher: divan::Bencher<'_, '_>) {
    let mut router = xitca_router::Router::new();
    for route in routes!(colon) {
        router.insert(route, true).unwrap();
    }

    bencher.bench(|| {
        for path in black_box(routes!(literal)) {
            let output = router.at(path).unwrap();
            let _parameters: SmallVec<[(&str, &str); 4]> =
                black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}
