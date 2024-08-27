#![allow(clippy::too_many_lines)]

//! Benches sourced from `path-tree` (MIT OR Apache-2.0)
//! <https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs>

use divan::AllocProfiler;
use path_tree_routes::paths;
use percent_encoding::percent_decode;
use std::hint::black_box;

pub mod path_tree_routes;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(name = "wayfind")]
fn wayfind(bencher: divan::Bencher) {
    let mut router = wayfind::Router::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.insert(route, index).unwrap();
    }

    bencher.bench(|| {
        for route in black_box(paths()) {
            let path = wayfind::Path::new(route).unwrap();
            let output = black_box(router.search(black_box(&path)).unwrap());
            let _parameters: Vec<(&str, &str)> =
                black_box(output.parameters.iter().map(|p| (p.key, p.value)).collect());
        }
    });
}

#[divan::bench(name = "actix-router")]
fn actix_router(bencher: divan::Bencher) {
    let mut router = actix_router::Router::<usize>::build();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.path(*route, index);
    }
    let router = router.finish();

    bencher.bench(|| {
        for route in black_box(paths()) {
            let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let mut path = actix_router::Path::new(path.as_ref());
            black_box(router.recognize(black_box(&mut path)).unwrap());
            let _parameters: Vec<(&str, &str)> =
                black_box(path.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "matchit")]
fn matchit(bencher: divan::Bencher) {
    let mut router = matchit::Router::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.insert(*route, index).unwrap();
    }

    bencher.bench(|| {
        for route in black_box(paths()) {
            let route = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let output = black_box(router.at(black_box(&route)).unwrap());
            let _parameters: Vec<(&str, &str)> =
                black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "ntex-router")]
fn ntex_router(bencher: divan::Bencher) {
    let mut router = ntex_router::Router::<usize>::build();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.path(*route, index);
    }
    let router = router.finish();

    bencher.bench(|| {
        for route in black_box(paths()) {
            let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let mut path = ntex_router::Path::new(path.as_ref());
            router.recognize(&mut path).unwrap();
            let _parameters: Vec<(&str, &str)> =
                black_box(path.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "path-tree")]
fn path_tree(bencher: divan::Bencher) {
    let mut router = path_tree::PathTree::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        let _ = router.insert(route, index);
    }

    bencher.bench(|| {
        for route in black_box(paths()) {
            let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let output = router.find(&path).unwrap();
            let _parameters: Vec<(&str, &str)> =
                black_box(output.1.params_iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "route-recognizer")]
fn route_recognizer(bencher: divan::Bencher) {
    let mut router = route_recognizer::Router::<usize>::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        router.add(route, index);
    }

    bencher.bench(|| {
        for route in black_box(paths()) {
            let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let output = router.recognize(&path).unwrap();
            let _parameters: Vec<(&str, &str)> =
                black_box(output.params().iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "routefinder")]
fn routefinder(bencher: divan::Bencher) {
    let mut router = routefinder::Router::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        router.add(*route, index).unwrap();
    }

    bencher.bench(|| {
        for route in black_box(paths()) {
            let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let output = router.best_match(&path).unwrap();
            let _parameters: Vec<(&str, &str)> =
                black_box(output.captures().iter().map(|p| (p.0, p.1)).collect());
        }
    });
}

#[divan::bench(name = "xitca-router")]
fn xitca_router(bencher: divan::Bencher) {
    let mut router = xitca_router::Router::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        router.insert(*route, index).unwrap();
    }

    bencher.bench(|| {
        for route in black_box(paths()) {
            let path = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let output = router.at(&path).unwrap();
            let _parameters: Vec<(&str, &str)> =
                black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
        }
    });
}
