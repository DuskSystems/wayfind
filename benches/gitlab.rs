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
        let mut builder = wayfind::RouterBuilder::new();
        for (index, route) in gitlab_routes::routes().iter().enumerate() {
            builder.insert(black_box(route), index).unwrap();
        }

        black_box(builder.build())
    });
}

#[divan::bench]
fn gitlab_delete(bencher: divan::Bencher<'_, '_>) {
    bencher
        .with_inputs(|| {
            let mut builder = wayfind::RouterBuilder::new();
            for (index, route) in gitlab_routes::routes().iter().enumerate() {
                builder.insert(route, index).unwrap();
            }

            builder.build()
        })
        .bench_values(|router| {
            let mut builder = router.into_builder();
            for route in gitlab_routes::routes() {
                builder.delete(black_box(route)).unwrap();
            }

            black_box(builder)
        });
}

#[divan::bench]
fn gitlab_display(bencher: divan::Bencher<'_, '_>) {
    let mut builder = wayfind::RouterBuilder::new();
    for (index, route) in gitlab_routes::routes().iter().enumerate() {
        builder.insert(route, index).unwrap();
    }

    let router = builder.build();
    bencher.bench(|| black_box(format!("{router}")));
}
