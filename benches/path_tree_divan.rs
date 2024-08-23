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
    let mut wayfind = wayfind::router::Router::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        wayfind.insert(route, index).unwrap();
    }

    bencher.bench(|| {
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
}

#[divan::bench(name = "actix-router")]
fn actix_router(bencher: divan::Bencher) {
    let mut router = actix_router::Router::<usize>::build();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.path(*route, index);
    }
    let router = router.finish();

    bencher.bench(|| {
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
}

#[divan::bench(name = "matchit")]
fn matchit(bencher: divan::Bencher) {
    let mut matcher = matchit::Router::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        let _ = matcher.insert(*route, index);
    }

    bencher.bench(|| {
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
}

#[divan::bench(name = "ntex-router")]
fn ntex_router(bencher: divan::Bencher) {
    let mut router = ntex_router::Router::<usize>::build();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.path(*route, index);
    }
    let router = router.finish();

    bencher.bench(|| {
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
}

#[divan::bench(name = "path-tree")]
fn path_tree(bencher: divan::Bencher) {
    let mut tree = path_tree::PathTree::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        let _ = tree.insert(route, index);
    }

    bencher.bench(|| {
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
}

#[divan::bench(name = "route-recognizer")]
fn route_recognizer(bencher: divan::Bencher) {
    let mut router = route_recognizer::Router::<usize>::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        router.add(route, index);
    }

    bencher.bench(|| {
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
}

#[divan::bench(name = "routefinder")]
fn routefinder(bencher: divan::Bencher) {
    let mut router = routefinder::Router::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        router.add(*route, index).unwrap();
    }

    bencher.bench(|| {
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
}

#[divan::bench(name = "xitca-router")]
fn xitca_router(bencher: divan::Bencher) {
    let mut xitca = xitca_router::Router::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        xitca.insert(*route, index).unwrap();
    }

    bencher.bench(|| {
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
}
