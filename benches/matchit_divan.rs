use core::hint::black_box;

use divan::AllocProfiler;

pub mod matchit_routes;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench_group]
mod wayfind {
    use super::{black_box, routes};

    #[divan::bench]
    fn default(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::wayfind::Router::new();
        for (index, route) in routes!(arrows).iter().enumerate() {
            router.insert(route, index).unwrap();
        }

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let _output = black_box(router.search(black_box(path)).unwrap());
            }
        });
    }

    #[divan::bench]
    fn parameters(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::wayfind::Router::new();
        for (index, route) in routes!(arrows).iter().enumerate() {
            router.insert(route, index).unwrap();
        }

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let output = black_box(router.search(black_box(path)).unwrap());
                let _parameters: Vec<(&str, &str)> =
                    black_box(output.parameters.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    }
}

#[divan::bench_group]
mod actix_router {
    use super::{black_box, routes};

    #[divan::bench]
    fn default(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::actix_router::Router::<usize>::build();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.path(*route, index);
        }
        let router = router.finish();

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let mut path = ::actix_router::Path::new(path);
                let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
            }
        });
    }

    #[divan::bench]
    fn parameters(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::actix_router::Router::<usize>::build();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.path(*route, index);
        }
        let router = router.finish();

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let mut path = ::actix_router::Path::new(path);
                let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
                let _parameters: Vec<(&str, &str)> =
                    black_box(path.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    }
}

#[divan::bench_group]
mod matchit {
    use super::{black_box, routes};

    #[divan::bench]
    fn default(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::matchit::Router::new();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.insert(*route, index).unwrap();
        }

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let _output = black_box(router.at(black_box(path)).unwrap());
            }
        });
    }

    #[divan::bench]
    fn parameters(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::matchit::Router::new();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.insert(*route, index).unwrap();
        }

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let output = black_box(router.at(black_box(path)).unwrap());
                let _parameters: Vec<(&str, &str)> =
                    black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    }
}

#[divan::bench_group]
mod ntex_router {
    use super::{black_box, routes};

    #[divan::bench]
    fn default(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::ntex_router::Router::<usize>::build();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.path(*route, index);
        }
        let router = router.finish();

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let mut path = ::ntex_router::Path::new(path);
                let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
            }
        });
    }

    #[divan::bench]
    fn parameters(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::ntex_router::Router::<usize>::build();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.path(*route, index);
        }
        let router = router.finish();

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let mut path = ::ntex_router::Path::new(path);
                let _output = black_box(router.recognize(black_box(&mut path)).unwrap());
                let _parameters: Vec<(&str, &str)> =
                    black_box(path.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    }
}

#[divan::bench_group]
mod path_tree {
    use super::{black_box, routes};

    #[divan::bench]
    fn default(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::path_tree::PathTree::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            let _ = router.insert(route, index);
        }

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let _output = black_box(router.find(path).unwrap());
            }
        });
    }

    #[divan::bench]
    fn parameters(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::path_tree::PathTree::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            let _ = router.insert(route, index);
        }

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let output = router.find(path).unwrap();
                let _parameters: Vec<(&str, &str)> =
                    black_box(output.1.params_iter().map(|p| (p.0, p.1)).collect());
            }
        });
    }
}

#[divan::bench_group]
mod route_recognizer {
    use super::{black_box, routes};

    #[divan::bench]
    fn default(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::route_recognizer::Router::<usize>::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            router.add(route, index);
        }

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let _output = black_box(router.recognize(path).unwrap());
            }
        });
    }

    #[divan::bench]
    fn parameters(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::route_recognizer::Router::<usize>::new();
        for (index, route) in routes!(colon).iter().enumerate() {
            router.add(route, index);
        }

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let output = router.recognize(path).unwrap();
                let _parameters: Vec<(&str, &str)> =
                    black_box(output.params().iter().map(|p| (p.0, p.1)).collect());
            }
        });
    }
}

#[divan::bench_group]
mod xitca_router {
    use super::{black_box, routes};

    #[divan::bench]
    fn default(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::xitca_router::Router::new();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.insert(*route, index).unwrap();
        }

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let _output = black_box(router.at(path).unwrap());
            }
        });
    }

    #[divan::bench]
    fn parameters(bencher: divan::Bencher<'_, '_>) {
        let mut router = ::xitca_router::Router::new();
        for (index, route) in routes!(brackets).iter().enumerate() {
            router.insert(*route, index).unwrap();
        }

        bencher.bench(|| {
            for path in black_box(routes!(literal)) {
                let output = router.at(path).unwrap();
                let _parameters: Vec<(&str, &str)> =
                    black_box(output.params.iter().map(|p| (p.0, p.1)).collect());
            }
        });
    }
}
