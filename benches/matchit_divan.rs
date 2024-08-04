//! Benches sourced from `matchit` (MIT AND BSD-3-Clause)
//! <https://github.com/ibraheemdev/matchit/blob/v0.8.3/benches/bench.rs>

use divan::AllocProfiler;
use matchit_routes::paths;

pub mod matchit_routes;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(name = "wayfind")]
fn wayfind() {
    let mut wayfind = wayfind::router::Router::new();
    for route in routes!(brackets) {
        wayfind.insert(route, true).unwrap();
    }

    for route in paths() {
        wayfind.matches(route).unwrap();
    }
}

#[divan::bench(name = "actix-router")]
fn actix_router() {
    let mut actix = actix_router::Router::<bool>::build();
    for route in routes!(brackets) {
        actix.path(route, true);
    }
    let actix = actix.finish();

    for route in paths() {
        let mut path = actix_router::Path::new(route);
        actix.recognize(&mut path).unwrap();
    }
}

#[divan::bench(name = "gonzales")]
fn gonzales() {
    let gonzales = gonzales::RouterBuilder::new().build(routes!(brackets));

    for route in paths() {
        gonzales.route(route).unwrap();
    }
}

#[divan::bench(name = "matchit")]
fn matchit() {
    let mut matchit = matchit::Router::new();
    for route in routes!(brackets) {
        matchit.insert(route, true).unwrap();
    }

    for route in paths() {
        matchit.at(route).unwrap();
    }
}

#[divan::bench(name = "ntex-router")]
fn ntex_router() {
    let mut ntex = ntex_router::Router::<bool>::build();
    for route in routes!(brackets) {
        ntex.path(route, true);
    }
    let ntex = ntex.finish();

    for route in paths() {
        let mut path = ntex_router::Path::new(route);
        ntex.recognize(&mut path).unwrap();
    }
}

#[divan::bench(name = "path-table")]
fn path_table() {
    let mut table = path_table::PathTable::new();
    for route in routes!(brackets) {
        *table.setup(route) = true;
    }

    for route in paths() {
        table.route(route).unwrap();
    }
}

#[divan::bench(name = "path-tree")]
fn path_tree() {
    let mut path_tree = path_tree::PathTree::new();
    for route in routes!(colon) {
        let _ = path_tree.insert(route, true);
    }

    for route in paths() {
        path_tree.find(route).unwrap();
    }
}

#[divan::bench(name = "regex")]
fn regex() {
    let regex_set = regex::RegexSet::new(routes!(regex)).unwrap();

    for route in paths() {
        regex_set.matches(route);
    }
}

#[divan::bench(name = "route-recognizer")]
fn route_recognizer() {
    let mut route_recognizer = route_recognizer::Router::new();
    for route in routes!(colon) {
        route_recognizer.add(route, true);
    }

    for route in paths() {
        route_recognizer
            .recognize(route)
            .unwrap();
    }
}

#[divan::bench(name = "routefinder")]
fn routefinder() {
    let mut routefinder = routefinder::Router::new();
    for route in routes!(colon) {
        routefinder.add(route, true).unwrap();
    }

    for route in paths() {
        routefinder.best_match(route).unwrap();
    }
}
