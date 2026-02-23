use core::hint::black_box;
use std::fmt::Write;

use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

fn segments(count: usize) -> String {
    "/x".repeat(count)
}

fn anchors(count: usize) -> String {
    "/x/-".repeat(count) + "/x"
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_depth<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut router = wayfind::Router::new();
    router.insert("/<*x>/z", 1).unwrap();

    let path = segments(N);
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_anchors<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut router = wayfind::Router::new();
    router.insert("/<*x>/-/<*y>/z", 1).unwrap();

    let path = anchors(N);
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

#[divan::bench(consts = [1, 10, 100, 1000])]
fn wildcard_competing<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut router = wayfind::Router::new();
    for index in 0..N {
        let template = format!("/<*x>/z{index}");
        router.insert(&template, index).unwrap();
    }

    let path = segments(20) + "/zz";
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}

// TODO
// #[divan::bench(consts = [1, 10, 100, 1000])]
#[divan::bench(consts = [1, 10])]
fn wildcard_chain<const N: usize>(bencher: divan::Bencher<'_, '_>) {
    let mut router = wayfind::Router::new();

    let mut template = String::new();
    for index in 0..N {
        if index > 0 {
            template.push_str("/-");
        }

        let _ = write!(template, "/<*x{index}>");
    }

    template.push_str("/z");
    router.insert(&template, 1).unwrap();

    let path = anchors(N);
    bencher.bench(|| black_box(router.search(black_box(path.as_str()))));
}
