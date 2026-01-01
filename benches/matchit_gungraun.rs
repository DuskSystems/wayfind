#![allow(
    unsafe_code,
    reason = "https://github.com/gungraun/gungraun/issues/490"
)]

use core::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};

mod matchit_routes;

library_benchmark_group!(
    name = default_group;
    benchmarks =
        wayfind_default,
        // FIXME: SIGIL Crashes on aarch64 with target-cpu=native
        // actix_router_default,
        matchit_default,
        // FIXME: SIGIL Crashes on aarch64 with target-cpu=native
        // ntex_router_default,
        path_tree_default,
        route_recognizer_default,
        xitca_router_default,
);

library_benchmark_group!(
    name = parameters_group;
    benchmarks =
        wayfind_parameters,
        // FIXME: SIGIL Crashes on aarch64 with target-cpu=native
        // actix_router_parameters,
        matchit_parameters,
        // FIXME: SIGIL Crashes on aarch64 with target-cpu=native
        // ntex_router_parameters,
        path_tree_parameters,
        route_recognizer_parameters,
        xitca_router_parameters,
);

main!(library_benchmark_groups = default_group, parameters_group);

fn setup_wayfind() -> wayfind::Router<usize> {
    let mut router = wayfind::Router::new();
    for (index, route) in routes!(arrows).iter().enumerate() {
        router.insert(route, index).unwrap();
    }

    router
}

#[library_benchmark]
#[bench::default(&setup_wayfind())]
fn wayfind_default(router: &wayfind::Router<usize>) {
    for path in black_box(routes!(literal)) {
        let _output = black_box(router.search(black_box(path)).unwrap());
    }
}

#[library_benchmark]
#[bench::default(&setup_wayfind())]
fn wayfind_parameters(router: &wayfind::Router<usize>) {
    for path in black_box(routes!(literal)) {
        let output = black_box(router.search(black_box(path)).unwrap());
        let _parameters: Vec<(&str, &str)> =
            black_box(output.parameters.iter().map(|p| (p.0, p.1)).collect());
    }
}

fn setup_actix_router() -> actix_router::Router<usize> {
    let mut router = actix_router::Router::<usize>::build();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.path(*route, index);
    }

    router.finish()
}

#[library_benchmark]
#[bench::default(&setup_actix_router())]
fn actix_router_default(router: &actix_router::Router<usize>) {
    for path in black_box(routes!(literal)) {
        let mut path = actix_router::Path::new(path);
        let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
    }
}

#[library_benchmark]
#[bench::default(&setup_actix_router())]
fn actix_router_parameters(router: &actix_router::Router<usize>) {
    for path in black_box(routes!(literal)) {
        let mut path = actix_router::Path::new(path);
        let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
        let _parameters: Vec<(&str, &str)> = black_box(path.iter().map(|p| (p.0, p.1)).collect());
    }
}

fn setup_matchit() -> matchit::Router<usize> {
    let mut router = matchit::Router::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.insert(*route, index).unwrap();
    }

    router
}

#[library_benchmark]
#[bench::default(&setup_matchit())]
fn matchit_default(router: &matchit::Router<usize>) {
    for path in black_box(routes!(literal)) {
        let _output = black_box(router.at(black_box(path)).unwrap());
    }
}

#[library_benchmark]
#[bench::default(&setup_matchit())]
fn matchit_parameters(router: &matchit::Router<usize>) {
    for path in black_box(routes!(literal)) {
        let output = black_box(router.at(black_box(path)).unwrap());
        let _parameters: Vec<(&str, &str)> =
            black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
    }
}

fn setup_ntex_router() -> ntex_router::Router<usize> {
    let mut router = ntex_router::Router::<usize>::build();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.path(*route, index);
    }

    router.finish()
}

#[library_benchmark]
#[bench::default(&setup_ntex_router())]
fn ntex_router_default(router: &ntex_router::Router<usize>) {
    for path in black_box(routes!(literal)) {
        let mut path = ntex_router::Path::new(path);
        let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
    }
}

#[library_benchmark]
#[bench::default(&setup_ntex_router())]
fn ntex_router_parameters(router: &ntex_router::Router<usize>) {
    for path in black_box(routes!(literal)) {
        let mut path = ntex_router::Path::new(path);
        let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
        let _parameters: Vec<(&str, &str)> = black_box(path.iter().map(|p| (p.0, p.1)).collect());
    }
}

fn setup_path_tree() -> path_tree::PathTree<usize> {
    let mut router = path_tree::PathTree::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        let _ = router.insert(route, index);
    }

    router
}

#[library_benchmark]
#[bench::default(&setup_path_tree())]
fn path_tree_default(router: &path_tree::PathTree<usize>) {
    for path in black_box(routes!(literal)) {
        let _output = black_box(router.find(path).unwrap());
    }
}

#[library_benchmark]
#[bench::default(&setup_path_tree())]
fn path_tree_parameters(router: &path_tree::PathTree<usize>) {
    for path in black_box(routes!(literal)) {
        let output = router.find(path).unwrap();
        let _parameters: Vec<(&str, &str)> =
            black_box(output.1.params_iter().map(|p| (p.0, p.1)).collect());
    }
}

fn setup_route_recognizer() -> route_recognizer::Router<usize> {
    let mut router = route_recognizer::Router::<usize>::new();
    for (index, route) in routes!(colon).iter().enumerate() {
        router.add(route, index);
    }

    router
}

#[library_benchmark]
#[bench::default(&setup_route_recognizer())]
fn route_recognizer_default(router: &route_recognizer::Router<usize>) {
    for path in black_box(routes!(literal)) {
        let _output = black_box(router.recognize(path).unwrap());
    }
}

#[library_benchmark]
#[bench::default(&setup_route_recognizer())]
fn route_recognizer_parameters(router: &route_recognizer::Router<usize>) {
    for path in black_box(routes!(literal)) {
        let output = router.recognize(path).unwrap();
        let _parameters: Vec<(&str, &str)> =
            black_box(output.params().iter().map(|p| (p.0, p.1)).collect());
    }
}

fn setup_xitca_router() -> xitca_router::Router<usize> {
    let mut router = xitca_router::Router::new();
    for (index, route) in routes!(brackets).iter().enumerate() {
        router.insert(*route, index).unwrap();
    }

    router
}

#[library_benchmark]
#[bench::default(&setup_xitca_router())]
fn xitca_router_default(router: &xitca_router::Router<usize>) {
    for path in black_box(routes!(literal)) {
        let _output = black_box(router.at(path).unwrap());
    }
}

#[library_benchmark]
#[bench::default(&setup_xitca_router())]
fn xitca_router_parameters(router: &xitca_router::Router<usize>) {
    for path in black_box(routes!(literal)) {
        let output = router.at(path).unwrap();
        let _parameters: Vec<(&str, &str)> =
            black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
    }
}
