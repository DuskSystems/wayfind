use divan::AllocProfiler;
use gitlab_routes::routes;
use std::hint::black_box;

pub mod gitlab_routes;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(name = "wayfind insert")]
fn wayfind_insert(bencher: divan::Bencher<'_, '_>) {
    let router = wayfind::Router::new();
    bencher.with_inputs(|| router.clone()).bench_refs(|router| {
        for route in black_box(routes()) {
            router.insert(black_box(route), true).unwrap();
        }
    });
}

#[divan::bench(name = "wayfind delete")]
fn wayfind_delete(bencher: divan::Bencher<'_, '_>) {
    let mut router = wayfind::Router::new();
    for route in routes() {
        router.insert(route, true).unwrap();
    }

    bencher.with_inputs(|| router.clone()).bench_refs(|router| {
        for route in black_box(routes()) {
            router.delete(black_box(route)).unwrap();
        }
    });
}

#[divan::bench(name = "wayfind display")]
fn wayfind_display(bencher: divan::Bencher<'_, '_>) {
    let mut router = wayfind::Router::new();
    for route in routes() {
        router.insert(route, true).unwrap();
    }

    bencher
        .with_inputs(|| router.clone())
        .bench_refs(|router| router.to_string());
}
