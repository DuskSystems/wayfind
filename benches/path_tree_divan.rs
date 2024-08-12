//! Benches sourced from `path-tree` (MIT OR Apache-2.0)
//! <https://github.com/viz-rs/path-tree/blob/v0.8.1/benches/bench.rs>

use divan::AllocProfiler;
use path_tree_routes::paths;

pub mod path_tree_routes;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(name = "wayfind")]
fn wayfind() {
    let mut wayfind = wayfind::router::Router::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        wayfind.insert(route, index).unwrap();
    }

    for (index, path) in paths() {
        let n = wayfind.matches(path).unwrap();
        assert_eq!(n.data.value, index);
    }
}

#[divan::bench(name = "actix-router")]
fn actix_router() {
    let mut router = actix_router::Router::<usize>::build();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.path(*route, index);
    }
    let router = router.finish();

    for (index, path) in paths() {
        let mut path = actix_router::Path::new(path);
        let n = router.recognize(&mut path).unwrap();
        assert_eq!(*n.0, index);
    }
}

#[divan::bench(name = "gonzales")]
fn gonzales() {
    let gonzales = gonzales::RouterBuilder::new().build(routes!(brackets));

    for (index, path) in paths() {
        let n = gonzales.route(path).unwrap();
        assert_eq!(n.get_index(), index);
    }
}

#[divan::bench(name = "matchit")]
fn matchit() {
    let mut matcher = matchit::Router::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        let _ = matcher.insert(*route, index);
    }

    for (index, path) in paths() {
        let n = matcher.at(path).unwrap();
        assert_eq!(*n.value, index);
    }
}

#[divan::bench(name = "ntex-router")]
fn ntex_router() {
    let mut router = ntex_router::Router::<usize>::build();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.path(*route, index);
    }
    let router = router.finish();

    for (index, path) in paths() {
        let mut path = ntex_router::Path::new(path);
        let n = router.recognize(&mut path).unwrap();
        assert_eq!(*n.0, index);
    }
}

#[divan::bench(name = "path-table")]
fn path_table() {
    let mut table = path_table::PathTable::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        *table.setup(route) = index;
    }

    for (index, path) in paths() {
        let n = table.route(path).unwrap();
        assert_eq!(*n.0, index);
    }
}

#[divan::bench(name = "path-tree")]
fn path_tree() {
    let mut tree = path_tree::PathTree::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        let _ = tree.insert(route, index);
    }

    for (index, path) in paths() {
        let n = tree.find(path).unwrap();
        assert_eq!(*n.0, index);
    }
}

#[divan::bench(name = "regex")]
fn regex() {
    let regex_set = regex::RegexSet::new(routes!(regex)).unwrap();

    for (index, path) in paths() {
        let n = regex_set.matches(path);
        assert!(n.matched(index));
    }
}

#[divan::bench(name = "route-recognizer")]
fn route_recognizer() {
    let mut router = route_recognizer::Router::<usize>::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        router.add(route, index);
    }

    for (index, path) in paths() {
        let n = router.recognize(path).unwrap();
        assert_eq!(**n.handler(), index);
    }
}

#[divan::bench(name = "routefinder")]
fn routefinder() {
    let mut router = routefinder::Router::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        router.add(*route, index).unwrap();
    }

    for (index, path) in paths() {
        let n = router.best_match(path).unwrap();
        assert_eq!(*n, index);
    }
}

#[divan::bench(name = "xitca-router")]
fn xitca_web() {
    let mut xitca = xitca_router::Router::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        xitca.insert(*route, index).unwrap();
    }

    for (index, path) in paths() {
        let n = xitca.at(path).unwrap();
        assert_eq!(*n.value, index);
    }
}
