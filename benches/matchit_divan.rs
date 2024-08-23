#![allow(clippy::too_many_lines)]

//! Benches sourced from `matchit` (MIT AND BSD-3-Clause)
//! <https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs>

use divan::AllocProfiler;
use matchit_routes::paths;
use percent_encoding::percent_decode;
use std::hint::black_box;

pub mod matchit_routes;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(name = "wayfind")]
fn wayfind(bencher: divan::Bencher) {
    let mut wayfind = wayfind::router::Router::new();
    for route in routes!(brackets) {
        wayfind.insert(route, true).unwrap();
    }

    bencher.bench(|| {
        for route in black_box(paths()) {
            let path = wayfind::path::Path::new(route).unwrap();
            let search = wayfind.search(&path).unwrap();
            let _parameters = black_box(
                search
                    .parameters
                    .iter()
                    .map(|p| (p.key, p.value))
                    .collect::<Vec<(&str, &str)>>(),
            );
        }
    });
}

#[divan::bench(name = "actix-router")]
fn actix_router(bencher: divan::Bencher) {
    let mut actix = actix_router::Router::<bool>::build();
    for route in routes!(brackets) {
        actix.path(route, true);
    }
    let actix = actix.finish();

    bencher.bench(|| {
        for route in black_box(paths()) {
            let route = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let mut path = actix_router::Path::new(route.as_ref());
            actix.recognize(&mut path).unwrap();
            let _parameters = black_box(
                path.iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>(),
            );
        }
    });
}

#[divan::bench(name = "matchit")]
fn matchit(bencher: divan::Bencher) {
    let mut matchit = matchit::Router::new();
    for route in routes!(brackets) {
        matchit.insert(route, true).unwrap();
    }

    bencher.bench(|| {
        for route in black_box(paths()) {
            let route = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let at = matchit.at(route.as_ref()).unwrap();
            let _parameters = black_box(
                at.params
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>(),
            );
        }
    });
}

#[divan::bench(name = "ntex-router")]
fn ntex_router(bencher: divan::Bencher) {
    let mut ntex = ntex_router::Router::<bool>::build();
    for route in routes!(brackets) {
        ntex.path(route, true);
    }
    let ntex = ntex.finish();

    bencher.bench(|| {
        for route in black_box(paths()) {
            let route = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let mut path = ntex_router::Path::new(route.as_ref());
            ntex.recognize(&mut path).unwrap();
            let _parameters = black_box(
                path.iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>(),
            );
        }
    });
}

#[divan::bench(name = "path-tree")]
fn path_tree(bencher: divan::Bencher) {
    let mut path_tree = path_tree::PathTree::new();
    for route in routes!(colon) {
        let _ = path_tree.insert(route, true);
    }

    bencher.bench(|| {
        for route in black_box(paths()) {
            let route = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let route = path_tree.find(route.as_ref()).unwrap();
            let _parameters = black_box(
                route
                    .1
                    .params_iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>(),
            );
        }
    });
}

#[divan::bench(name = "route-recognizer")]
fn route_recognizer(bencher: divan::Bencher) {
    let mut route_recognizer = route_recognizer::Router::new();
    for route in routes!(colon) {
        route_recognizer.add(route, true);
    }

    bencher.bench(|| {
        for route in black_box(paths()) {
            let route = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let recognize = route_recognizer.recognize(route.as_ref()).unwrap();
            let _parameters = black_box(
                recognize
                    .params()
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>(),
            );
        }
    });
}

#[divan::bench(name = "routefinder")]
fn routefinder(bencher: divan::Bencher) {
    let mut routefinder = routefinder::Router::new();
    for route in routes!(colon) {
        routefinder.add(route, true).unwrap();
    }

    bencher.bench(|| {
        for route in black_box(paths()) {
            let route = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let best_match = routefinder.best_match(route.as_ref()).unwrap();
            let _parameters = black_box(
                best_match
                    .captures()
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>(),
            );
        }
    });
}

#[divan::bench(name = "xitca-router")]
fn xitca_router(bencher: divan::Bencher) {
    let mut xitca = xitca_router::Router::new();
    for route in routes!(colon) {
        xitca.insert(route, true).unwrap();
    }

    bencher.bench(|| {
        for route in black_box(paths()) {
            let route = percent_decode(route.as_bytes()).decode_utf8().unwrap();
            let at = xitca.at(route.as_ref()).unwrap();
            let _parameters = black_box(
                at.params
                    .iter()
                    .map(|p| (p.0, p.1))
                    .collect::<Vec<(&str, &str)>>(),
            );
        }
    });
}
