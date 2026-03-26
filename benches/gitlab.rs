#![expect(missing_docs, clippy::unwrap_used, reason = "Bench")]

use core::hint::black_box;

use divan::AllocProfiler;

#[path = "fixtures/gitlab_routes.rs"]
mod gitlab_routes;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn gitlab_insert(bencher: divan::Bencher<'_, '_>) {
    bencher.bench(|| {
        let mut router = wayfind::Router::new();
        for (index, route) in gitlab_routes::routes().iter().enumerate() {
            router.insert(black_box(route), index).unwrap();
        }

        black_box(router)
    });
}

#[divan::bench]
fn gitlab_delete(bencher: divan::Bencher<'_, '_>) {
    bencher
        .with_inputs(|| {
            let mut router = wayfind::Router::new();
            for (index, route) in gitlab_routes::routes().iter().enumerate() {
                router.insert(route, index).unwrap();
            }

            router
        })
        .bench_values(|mut router| {
            for route in gitlab_routes::routes() {
                router.delete(black_box(route)).unwrap();
            }

            black_box(router)
        });
}

#[divan::bench]
fn gitlab_display(bencher: divan::Bencher<'_, '_>) {
    let mut router = wayfind::Router::new();
    for (index, route) in gitlab_routes::routes().iter().enumerate() {
        router.insert(route, index).unwrap();
    }

    bencher.bench(|| black_box(format!("{router}")));
}
