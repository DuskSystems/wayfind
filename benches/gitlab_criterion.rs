use codspeed_criterion_compat::{criterion_group, criterion_main, BatchSize, Criterion};
use gitlab_routes::routes;
use std::hint::black_box;
use tikv_jemallocator::Jemalloc;

pub mod gitlab_routes;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = insert_benchmark, delete_benchmark, display_benchmark
}

fn insert_benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("gitlab insert benchmarks");

    group.bench_function("gitlab insert benchmarks/wayfind", |bencher| {
        let router = wayfind::Router::new();
        bencher.iter_batched(
            || router.clone(),
            |mut router| {
                for route in black_box(routes()) {
                    let route = wayfind::RoutableBuilder::new()
                        .route(black_box(route))
                        .build()
                        .unwrap();
                    router.insert(black_box(&route), true).unwrap();
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn delete_benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("gitlab delete benchmarks");

    group.bench_function("gitlab delete benchmarks/wayfind", |bencher| {
        let mut router = wayfind::Router::new();
        for route in routes() {
            let route = wayfind::RoutableBuilder::new()
                .route(route)
                .build()
                .unwrap();
            router.insert(&route, true).unwrap();
        }

        bencher.iter_batched(
            || router.clone(),
            |mut router| {
                for route in black_box(routes()) {
                    let route = wayfind::RoutableBuilder::new()
                        .route(black_box(route))
                        .build()
                        .unwrap();
                    router.delete(black_box(&route)).unwrap();
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn display_benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("gitlab display benchmarks");

    group.bench_function("gitlab display benchmarks/wayfind", |bencher| {
        let mut router = wayfind::Router::new();
        for route in routes() {
            let route = wayfind::RoutableBuilder::new()
                .route(route)
                .build()
                .unwrap();
            router.insert(&route, true).unwrap();
        }

        bencher.iter_batched(
            || router.clone(),
            |router| router.to_string(),
            BatchSize::SmallInput,
        );
    });

    group.finish();
}
